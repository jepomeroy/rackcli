use crate::device::Device;
use dialoguer;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, UdpSocket};

#[derive(Serialize, Deserialize)]
pub struct Wol {
    pub name: String,
    mac: String,
}

impl Device for Wol {
    fn disable(&self) -> std::io::Result<()> {
        unimplemented!();
    }

    fn update(&mut self) {
        let re = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();
        let mac = dialoguer::Input::<String>::new()
            .with_prompt("MAC")
            .validate_with(|input: &String| -> Result<(), &str> {
                if !re.is_match(input) {
                    Err("Invalid MAC address")
                } else {
                    Ok(())
                }
            })
            .default(self.mac.clone())
            .interact()
            .unwrap();

        self.mac = mac;
    }

    fn enable(&self) -> std::io::Result<()> {
        // Create magic packet
        // 6 bytes of 0xff followed by 16 repetitions of the target MAC address
        let mut magic_packet = vec![0xff; 6];
        let mac = self.get_octets().repeat(16);

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

        println!("Sent Wake-on-Lan packet to {}", self.name);
        Ok(())
    }

    fn status(&self) -> std::io::Result<()> {
        unimplemented!();
    }
}

impl Wol {
    pub fn create(wol_names: Vec<String>) -> Self {
        let re = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$").unwrap();

        let name = dialoguer::Input::<String>::new()
            .with_prompt("Name")
            .validate_with(|input: &String| -> Result<(), &str> {
                if wol_names.contains(input) {
                    Err("Name already exists")
                } else {
                    Ok(())
                }
            })
            .interact()
            .unwrap();

        let mac = dialoguer::Input::<String>::new()
            .with_prompt("MAC")
            .validate_with(|input: &String| -> Result<(), &str> {
                if !re.is_match(input) {
                    Err("Invalid MAC address")
                } else {
                    Ok(())
                }
            })
            .interact()
            .unwrap();

        Self { name, mac }
    }

    fn get_octets(&self) -> Vec<u8> {
        let mut octets = Vec::<u8>::new();
        for octet in self.mac.split(":") {
            octets.push(u8::from_str_radix(octet, 16).unwrap());
        }

        octets
    }
}

impl std::fmt::Display for Wol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  Name: {}\n  MAC: {}\n", self.name, self.mac)
    }
}
