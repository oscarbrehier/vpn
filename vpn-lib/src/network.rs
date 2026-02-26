use std::net::Ipv4Addr;

use surge_ping::{Client, Config, PingIdentifier, PingSequence};

pub async fn ping_endpoint(ip: Ipv4Addr) -> Option<u128> {
    let client = Client::new(&Config::default()).ok()?;
    let mut pinger = client.pinger(ip.into(), PingIdentifier(0)).await;

    let payload = [0u8; 56];
    match pinger.ping(PingSequence(0), &payload).await {
        Ok((surge_ping::IcmpPacket::V4(_packet), duration)) => Some(duration.as_millis()),
        _ => None,
    }
}
