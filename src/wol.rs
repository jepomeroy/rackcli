use crate::device::Device;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    net::{Ipv4Addr, UdpSocket},
    num::ParseIntError,
    sync::LazyLock,
};

static MAC_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([0-9A-Fa-f]{2}[:]){5}([0-9A-Fa-f]{2})$").unwrap()
});

#[derive(Serialize, Deserialize)]
pub struct Wol {
    pub name: String,
    mac: String,
}

impl Device for Wol {
    async fn disable(&mut self) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Disable not implemented for Wol",
        ))
    }

    fn update(&mut self) {
        let mac = dialoguer::Input::<String>::new()
            .with_prompt("MAC")
            .validate_with(|input: &String| -> Result<(), &str> {
                if !MAC_RE.is_match(input) {
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

    async fn enable(&mut self) -> std::io::Result<()> {
        // Create magic packet
        // 6 bytes of 0xff followed by 16 repetitions of the target MAC address
        let mut magic_packet = vec![0xff; 6];
        let mac = match self.get_octets() {
            Ok(m) => m.repeat(16),
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MAC address: {}", e),
                ))
            }
        };

        // build magic packet
        magic_packet.extend(mac);

        if magic_packet.len() != 102 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Magic packet is not 102 bytes",
            ));
        }

        // Send magic packet to broadcast address on port 9
        // Port 9 is the default port for Wake-on-Lan
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;

        socket.send_to(&magic_packet, (Ipv4Addr::new(255, 255, 255, 255), 9))?;

        println!("Sent Wake-on-Lan packet to {}", self.name);
        Ok(())
    }

    async fn status(&mut self) {
        println!("Not implemented");
    }
}

impl Wol {
    pub fn create(wol_names: Vec<String>) -> Self {
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
                if !MAC_RE.is_match(input) {
                    Err("Invalid MAC address. Format should be XX:XX:XX:XX:XX:XX where X is a hexadecimal digit")
                } else {
                    Ok(())
                }
            })
            .interact()
            .unwrap();

        Self { name, mac }
    }

    pub fn get_octets(&self) -> Result<Vec<u8>, ParseIntError> {
        let mut octets = Vec::<u8>::new();
        for octet in self.mac.split(":") {
            octets.push(u8::from_str_radix(octet, 16)?);
        }

        Ok(octets)
    }
}

impl std::fmt::Display for Wol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  Name: {}\n  MAC: {}\n", self.name, self.mac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wol(mac: &str) -> Wol {
        Wol { name: "test".to_string(), mac: mac.to_string() }
    }

    #[test]
    fn test_get_octets_valid() {
        assert_eq!(wol("AA:BB:CC:DD:EE:FF").get_octets().unwrap(), vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn test_get_octets_lowercase() {
        assert_eq!(wol("aa:bb:cc:dd:ee:ff").get_octets().unwrap(), vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn test_get_octets_zeros() {
        assert_eq!(wol("00:00:00:00:00:00").get_octets().unwrap(), vec![0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_get_octets_broadcast() {
        assert_eq!(wol("FF:FF:FF:FF:FF:FF").get_octets().unwrap(), vec![0xFF; 6]);
    }

    #[test]
    fn test_get_octets_invalid_hex() {
        assert!(wol("ZZ:BB:CC:DD:EE:FF").get_octets().is_err());
    }

    #[test]
    fn test_get_octets_length() {
        assert_eq!(wol("01:23:45:67:89:AB").get_octets().unwrap().len(), 6);
    }
}
