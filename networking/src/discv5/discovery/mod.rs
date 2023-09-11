// #![deny(unsafe_code)]

// pub mod enr;
// pub mod events;

// use super::discovery::enr::*;
// use super::discovery::events::discv5_events;

// use clap::{App, Arg};
// use discv5::*;
// use discv5::{
//     enr as discv5_enr, enr::CombinedKey, handler, kbucket, metrics, packet, permit_ban, rpc,
//     service, socket, Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Event, Enr, ListenConfig,
// };
// use futures::Future;
// use libp2p::swarm::dummy::ConnectionHandler;
// use libp2p::swarm::NetworkBehaviour;
// use libp2p::PeerId;
// use lru::LruCache;
// use slog::Logger;
// use std::error::Error;
// use std::net::Ipv4Addr;
// use std::num::NonZeroUsize;
// use std::pin::Pin;
// use std::time::Duration;
// use void::Void;

// pub struct Discovery {
//     cached_enrs: LruCache<PeerId, Enr>,
//     enr: Enr,
//     discv5: Discv5,
//     log: Logger,
// }

// impl Discovery {
//     pub async fn new(log: slog::Logger) -> Result<Self, Box<dyn Error>> {
//         let log_clone = log.clone();
//         let (local_enr, enr, enr_key) = generate_enr(log_clone).await?;

//         let listen_port = enr.udp4().unwrap();

//         let discv5_listen_config =
//             discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), listen_port);

//         let discv5_config = Discv5ConfigBuilder::new(discv5_listen_config)
//             .ban_duration(Some(Duration::from_secs(60)))
//             .query_timeout(Duration::from_secs(10))
//             .request_retries(1)
//             .request_timeout(Duration::from_secs(1))
//             .query_parallelism(3)
//             .query_peer_timeout(Duration::from_secs(3))
//             .ping_interval(Duration::from_secs(300))
//             .build();

//         let discv5: Discv5 = Discv5::new(enr.clone(), enr_key, discv5_config)?;
//         let cached_enrs = LruCache::new(NonZeroUsize::new(1000).unwrap());

//         Ok(Discovery {
//             cached_enrs,
//             enr,
//             discv5,
//             log,
//         })
//     }

//     pub fn get_enr(&self) -> Enr {
//         self.enr.clone()
//     }

//     pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
//         self.discv5.start().await.unwrap();

//         Ok(())
//     }
// }

// impl NetworkBehaviour for Discovery {
//     type ConnectionHandler = ConnectionHandler;
//     type OutEvent = Void;

//     fn new_handler(&mut self) -> Self::ConnectionHandler {
//         ConnectionHandler
//     }

//     fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<libp2p::Multiaddr> {
//         Vec::new()
//     }

//     fn handle_established_inbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         peer: PeerId,
//         local_addr: &libp2p::Multiaddr,
//         remote_addr: &libp2p::Multiaddr,
//     ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
//         Ok(ConnectionHandler)
//     }

//     fn handle_established_outbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         peer: PeerId,
//         addr: &libp2p::Multiaddr,
//         role_override: libp2p::core::Endpoint,
//     ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
//         Ok(ConnectionHandler)
//     }

//     fn handle_pending_inbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         local_addr: &libp2p::Multiaddr,
//         remote_addr: &libp2p::Multiaddr,
//     ) -> Result<(), libp2p::swarm::ConnectionDenied> {
//         Ok(())
//     }

//     fn handle_pending_outbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         maybe_peer: Option<PeerId>,
//         addresses: &[libp2p::Multiaddr],
//         effective_role: libp2p::core::Endpoint,
//     ) -> Result<Vec<libp2p::Multiaddr>, libp2p::swarm::ConnectionDenied> {
//         Ok(Vec::new())
//     }

//     fn on_connection_handler_event(
//         &mut self,
//         peer_id: PeerId,
//         connection_id: libp2p::swarm::ConnectionId,
//         event: libp2p::swarm::THandlerOutEvent<Self>,
//     ) {
//     }

//     fn on_swarm_event(&mut self, event: libp2p::swarm::FromSwarm<Self::ConnectionHandler>) {}

//     fn poll(
//         &mut self,
//         cx: &mut std::task::Context<'_>,
//         params: &mut impl libp2p::swarm::PollParameters,
//     ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::OutEvent, libp2p::swarm::THandlerInEvent<Self>>>
//     {
//         std::task::Poll::Pending
//     }
// }
