#![deny(unsafe_code)]

use discv5::{
    enr,
    enr::{CombinedKey, EnrBuilder},
    Enr,
};
use eyre::Result;
use reqwest::header::{HeaderMap, ACCEPT};
use serde_json::Value;
use std::error::Error;
use std::str::FromStr;

pub async fn get_local_peer_info(
) -> Result<(String, String, String, String, String, String), Box<dyn Error>> {
    let url = "http://127.0.0.1:5052/eth/v1/node/identity";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    let res = client.get(url).headers(headers).send().await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;
    let peer_id = json["data"]["peer_id"]
        .as_str()
        .ok_or("Peer ID not found")?
        .to_owned();
    let enr = json["data"]["enr"]
        .as_str()
        .ok_or("ENR not found")?
        .to_owned();
    let p2p_address = json["data"]["p2p_addresses"][0]
        .as_str()
        .ok_or("P2P address not found")?
        .to_owned();
    let discovery_address = json["data"]["discovery_addresses"][0]
        .as_str()
        .ok_or("Discovery address not found")?
        .to_owned();
    let attnets = json["data"]["metadata"]["attnets"]
        .as_str()
        .ok_or("attnets not found")?
        .to_owned();
    let syncnets = json["data"]["metadata"]["syncnets"]
        .as_str()
        .ok_or("syncnets not found")?
        .to_owned();
    Ok((
        peer_id,
        enr,
        p2p_address,
        discovery_address,
        attnets,
        syncnets,
    ))
}

pub async fn decode_hex_value(hex_string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes =
        hex::decode(&hex_string.replace("0x", "")).map_err(|_| "Failed to parse hex string")?;
    Ok(bytes)
}

pub async fn get_eth2_value(enr_string: &str) -> Option<String> {
    if let Some(start) = enr_string.find("\"eth2\", \"") {
        let rest = &enr_string[start + 9..];
        if let Some(end) = rest.find("\")") {
            return Some(rest[..end].to_string());
        }
    }
    None
}

pub async fn generate_enr() -> Result<(Enr, CombinedKey), Box<dyn Error>> {
    let (_, enr, _, _, attnets, syncnets) = get_local_peer_info().await?;

    let ip4 = "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap();
    let port: u16 = 7777;

    let syncnets_bytes = decode_hex_value(&syncnets).await?;
    let attnets_bytes = decode_hex_value(&attnets).await?;

    let decoded_enr: enr::Enr<CombinedKey> = Enr::from_str(&enr)?;

    let enr_string = format!("{:?}", decoded_enr);
    let eth2_value = get_eth2_value(&enr_string).await;

    // If eth2_value is None, return early
    let eth2_value = match eth2_value {
        Some(value) => value,
        None => return Err("Failed to get eth2 value from ENR".into()),
    };

    let eth2_bytes = decode_hex_value(&eth2_value).await?;

    let combined_key = CombinedKey::generate_secp256k1();

    let enr = EnrBuilder::new("v4")
        .ip4(ip4)
        .tcp4(port)
        .udp4(port)
        .add_value("syncnets", &syncnets_bytes)
        .add_value("attnets", &attnets_bytes)
        .add_value_rlp("eth2", eth2_bytes.into())
        .build(&combined_key)
        .map_err(|_| "Failed to generate ENR")?;

    println!(
        "FROM FILE: src/networking/discv5/enr.rs ||| ENR: {:?}\n",
        enr
    );
    println!("FROM FILE: src/networking/discv5/enr.rs ||| ENR: {}\n", enr);

    Ok((enr, combined_key))
}
