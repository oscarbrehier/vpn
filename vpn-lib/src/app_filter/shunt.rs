use std::net::Ipv4Addr;

use anyhow::Ok;
use etherparse::SlicedPacket;
use tokio::sync::mpsc;
use windivert::{layer::NetworkLayer, packet::WinDivertPacket};

fn build_filter_string(pids: &[u32]) -> String {
    if pids.is_empty() {
        return "false".into();
    };

    let pid_conditions: Vec<String> = pids
        .iter()
        .map(|pid| format!("processId === {}", pid))
        .collect();

    format!("outbound and ({})", pid_conditions.join(" or "))
}

fn redirect_to_tunnel(
    packet: &mut WinDivertPacket<'_>,
    virtual_ip: Ipv4Addr,
    interface_i: u32,
) -> Result<(), String> {
    let data = &mut packet.data;

    if data.len() < 20 {
        return Err("Packet too short".into());
    }

    let version = data[0] >> 4;

    if version == 4 {
        let ip_bytes = virtual_ip.octets();
        data[12..16].copy_from_slice(&ip_bytes);

        packet.address.if_idx = interface_i;
        packet.address.set_outbound(true);
    } else {
        return Err("IPv6 is not supported".to_string());
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn start_packet_redirection(
    initial_pids: Vec<u32>,
    mut filter_rx: mpsc::UnboundedReceiver<Vec<u32>>,
) -> Result<(), String> {
    use windivert::{WinDivert, prelude::WinDivertFlags};

    let current_filter = build_filter_string(&initial_pids);

    let mut divert = WinDivert::network(&current_filter, 0, WinDivertFlags::default())
        .map_err(|e| format!("Failed to open windivert: {}", e))?;

    let mut buffer = [0u8; 65535];

    loop {
        tokio::select! {

            Some(new_pids) = filter_rx.recv() => {

                current_filter = build_filter_string(&new_pids);

                // reopen with new filters
                divert = WinDivert::network(&current_filter, 0, WinDivertFlags::default()).map_err(|e| format!("Failed to open windivert: {}", e))?;

            }

            packet = async { divert.recv(Some(&mut buffer)) } => {

                if let Ok(wd_packet) = packet {

                    let mut modified_packet = wd_packet;

                    divert.send(&modified_packet).ok();

                }

            }

        }
    }

    Ok(())
}

#[cfg(target_os = "macos")]
pub async fn start_packet_redirection(
    target_pids: Vec<u32>,
    mut filter_rx: mpsc::UnboundedReceiver<Vec<u32>>,
) -> Result<(), String> {
    // utun
}

#[cfg(target_os = "linux")]
pub async fn start_packet_redirection(
    target_pids: Vec<u32>,
    mut filter_rx: mpsc::UnboundedReceiver<Vec<u32>>,
) -> Result<(), String> {
    // netlink
}

pub fn update_filter_rules(
    tx: &mpsc::UnboundedSender<Vec<u32>>,
    pids: Vec<u32>,
) -> Result<(), String> {
    tx.send(pids).map_err(|e| e.to_string())
}
