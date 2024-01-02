use crate::wol::Wol;
use std::net::{Ipv4Addr, UdpSocket};

pub fn wake_on_lan<'a>(wol: &'a Wol) -> std::io::Result<()> {
    // Create magic packet
    // 6 bytes of 0xff followed by 16 repetitions of the target MAC address
    let mut magic_packet = vec![0xff; 6];
    let mac = wol.get_octets().repeat(16);

    // build magic packet
    magic_packet.extend(mac);

    if magic_packet.len() != 102 {
        panic!("Magic packet is not 102 bytes");
    }

    // Send magic packet to broadcast address on port 9
    // Port 9 is the default port for Wake-on-Lan
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    socket.send_to(&magic_packet, (Ipv4Addr::new(255, 255, 255, 255), 9))?;

    println!("Sent Wake-on-Lan packet to {}", wol.name);
    Ok(())
}
