use std::{net::Ipv4Addr, thread, time::Duration};

pub fn get_interface_index(virtual_ip: Ipv4Addr) -> Result<u32, String> {
    for _ in 0..10 {
        let interfaces = netdev::get_interfaces();
        let found = interfaces
            .iter()
            .find(|i| i.ipv4.iter().any(|addr| addr.addr() == virtual_ip));

        if let Some(iface) = found {
            return Ok(iface.index);
        }

        thread::sleep(Duration::from_millis(500));
    }

    Err("Wireguard interface not found after timeout".into())
}
