#![deny(unsafe_code)]

use crate::create_logger;
use crate::discv5::enr::generate_enr;
use discv5::*;
use discv5::{
    enr, handler, kbucket, metrics, packet, permit_ban, rpc, service, socket, Discv5, Discv5Config,
    Discv5ConfigBuilder, Discv5Event, Enr, ListenConfig,
};
use futures::Future;
use libp2p::PeerId;
use lru::LruCache;
use std::error::Error;
use std::net::Ipv4Addr;
use std::time::Duration;
use std::pin::Pin;

// https://github.com/sigp/lighthouse/blob/stable/beacon_node/lighthouse_network/src/discovery/mod.rs#L191C27-L191C27
// pub struct Discovery<TSubstream> {
//     cached_enrs: LruCache<PeerId, Enr>,
//     discv5: Discv5,
//     event_stream: ,
// }

// impl<TSubstream> Discovery<TSubstream> {
//     pub fn new() -> Result<Self, Box<dyn Error>> {

//     }
// }

pub async fn start_discv5() -> Result<Discv5, Box<dyn Error>> {
    let log = create_logger();
    let (local_enr, enr, enr_key) = generate_enr().await?;

    let listen_addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
    let listen_port = enr.udp4().unwrap();
    slog::debug!(log, "Listening on"; "listen_addr" => %listen_addr);

    let discv5_listen_config =
        discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), listen_port);
    slog::debug!(log, "discv5_listen_config"; "config" => ?discv5_listen_config);

    let discv5_config = Discv5ConfigBuilder::new(discv5_listen_config)
        .ban_duration(Some(Duration::from_secs(60)))
        .query_timeout(Duration::from_secs(10))
        .request_retries(1)
        .request_timeout(Duration::from_secs(1))
        .query_parallelism(3)
        .query_peer_timeout(Duration::from_secs(3))
        .ping_interval(Duration::from_secs(300))
        .build();

    slog::debug!(log, "discv5_config"; "config" => ?discv5_config);

    let cloned_enr = enr.clone();
    let mut discv5: Discv5 = Discv5::new(enr, enr_key, discv5_config).unwrap();

    let cloned_local_enr = local_enr.clone();
    discv5.start().await.unwrap();

    let mut discv_events = discv5.event_stream().await.unwrap();

    loop {
        match discv_events.recv().await {
            Some(Discv5Event::SocketUpdated(socket_addr)) => {
                slog::debug!(log, "Socket Updated"; "socket_addr" => ?socket_addr);
            }
            Some(Discv5Event::Discovered(enr)) => {
                slog::debug!(log, "Discovered"; "enr" => ?enr);
            }
            Some(Discv5Event::NodeInserted { node_id, replaced }) => {
                slog::debug!(log, "Node Inserted"; "node_id" => %node_id, "replaced" => ?replaced);
            }
            Some(Discv5Event::EnrAdded { enr, replaced }) => {
                slog::debug!(log, "Enr Added"; "enr" => ?enr, "replaced" => ?replaced);
            }
            Some(Discv5Event::SessionEstablished(enr, socket_addr)) => {
                slog::debug!(log, "Session Established"; "enr" => ?enr, "socket_addr" => ?socket_addr);
            }
            Some(Discv5Event::TalkRequest(_)) => {
                slog::debug!(log, "Talk Request Received");
            }
            None => {
                slog::debug!(log, "No events");
            }
        }
    }
}
