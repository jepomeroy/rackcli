use crate::device::Device;
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::snmp::Snmp;
use crate::switch_oid::SwitchOidBuilder;

use std::net::{IpAddr, SocketAddr};

#[derive(Serialize, Deserialize)]
pub struct Switch {
    pub name: String,
    ip: String,
    brand: String,
    version: SNMPVersion,
    ports: u64,
    community: String,
    auth: SNMPAuth,
    auth_user: String,
    auth_pass: String,
    encryption: SNMPEncryption,
    encryption_pass: String,
}
#[derive(Clone)]
pub struct SwitchResult {
    pub port: u64,
    pub status: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum SNMPVersion {
    V2,
    V3,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum SNMPAuth {
    Md5,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum SNMPEncryption {
    None,
    Des,
    Aes128,
    Aes192,
    Aes256,
}

impl Device for Switch {
    fn disable(&self) -> std::io::Result<()> {
        let off = SwitchOidBuilder::new()
            .get_off(self.brand.clone())
            .expect("Invalid brand");

        self.set(off)
    }

    fn enable(&self) -> std::io::Result<()> {
        let on = SwitchOidBuilder::new()
            .get_on(self.brand.clone())
            .expect("Invalid brand");

        self.set(on)
    }

    fn status(&self) -> std::io::Result<()> {
        let ports = self.get_ports();

        let client = Snmp::new();
        let results = client.get(self, ports);

        match results {
            Ok(results) => {
                println!("Status for {}:", self.name);
                for result in results {
                    println!("\t{}", result);
                }
            }
            Err(e) => {
                println!("Error getting status for {}: {}", self.name, e);
            }
        }

        Ok(())
    }

    fn update(&mut self) {
        let sob = SwitchOidBuilder::new();

        let mut username = String::new();
        let mut password = String::new();
        let mut community = String::new();
        let mut auth = SNMPAuth::Md5;
        let mut encryption = SNMPEncryption::None;
        let mut encryption_pass = String::new();

        let ip = dialoguer::Input::<String>::new()
            .with_prompt("IP")
            .default(self.ip.clone())
            .interact()
            .unwrap();

        let ports = dialoguer::Input::<u64>::new()
            .with_prompt("Ports")
            .default(self.ports)
            .interact()
            .unwrap();

        let brand = sob.get_oid_name(
            dialoguer::Select::new()
                .with_prompt("Brand")
                .default(
                    sob.get_oid_names()
                        .iter()
                        .position(|x| x == &self.brand)
                        .unwrap(),
                )
                .items(sob.get_oid_names().as_slice())
                .interact()
                .unwrap(),
        );

        let version = match dialoguer::Select::new()
            .with_prompt("SNMP Version")
            .default(self.version as usize)
            .item("v2")
            .item("v3")
            .interact()
            .unwrap()
        {
            0 => SNMPVersion::V2,
            1 => SNMPVersion::V3,
            _ => unreachable!(),
        };

        if version == SNMPVersion::V2 {
            community = dialoguer::Input::<String>::new()
                .with_prompt("Community")
                .default(self.community.clone())
                .interact()
                .unwrap();
        } else {
            auth = match dialoguer::Select::new()
                .with_prompt("SNMP Authentication")
                .default(self.auth as usize)
                .item("MD5")
                .item("SHA1")
                .item("SHA224")
                .item("SHA256")
                .item("SHA384")
                .item("SHA512")
                .interact()
                .unwrap()
            {
                0 => SNMPAuth::Md5,
                1 => SNMPAuth::Sha1,
                2 => SNMPAuth::Sha224,
                3 => SNMPAuth::Sha256,
                4 => SNMPAuth::Sha384,
                5 => SNMPAuth::Sha512,
                _ => unreachable!(),
            };

            username = dialoguer::Input::<String>::new()
                .with_prompt("Username")
                .default(self.auth_user.clone())
                .interact()
                .unwrap();

            password = dialoguer::Password::new()
                .with_prompt("Password (blank to prompt each time)")
                .allow_empty_password(true)
                .with_confirmation("Confirm Password", "Passwords do not match")
                .interact()
                .unwrap();

            encryption = match dialoguer::Select::new()
                .with_prompt("SNMP Encryption")
                .default(self.encryption as usize)
                .item("None")
                .item("DES")
                .item("AES128")
                .item("AES192")
                .item("AES256")
                .interact()
                .unwrap()
            {
                0 => SNMPEncryption::None,
                1 => SNMPEncryption::Des,
                2 => SNMPEncryption::Aes128,
                3 => SNMPEncryption::Aes192,
                4 => SNMPEncryption::Aes256,
                _ => unreachable!(),
            };

            if encryption != SNMPEncryption::None {
                encryption_pass = dialoguer::Password::new()
                    .with_prompt("Encryption Password")
                    .with_confirmation("Confirm Password", "Passwords do not match")
                    .interact()
                    .unwrap();
            }
        }

        self.ip = ip;
        self.ports = ports;
        self.brand = brand;
        self.community = community;
        self.auth = auth;
        self.auth_user = username;
        self.auth_pass = password;
        self.encryption = encryption;
        self.encryption_pass = encryption_pass;
    }
}

impl Switch {
    pub fn create(switch_names: Vec<String>) -> Self {
        let sob = SwitchOidBuilder::new();
        let mut username = String::new();
        let mut password = String::new();
        let mut community = String::new();
        let mut auth = SNMPAuth::Md5;
        let mut encryption = SNMPEncryption::None;
        let mut encryption_pass = String::new();

        let name = dialoguer::Input::<String>::new()
            .with_prompt("Name")
            .validate_with(|input: &String| -> Result<(), &str> {
                if switch_names.contains(input) {
                    Err("Name already exists")
                } else {
                    Ok(())
                }
            })
            .interact()
            .unwrap();

        let ip = dialoguer::Input::<String>::new()
            .with_prompt("IP")
            .interact()
            .unwrap();

        let ports = dialoguer::Input::<u64>::new()
            .with_prompt("Ports")
            .interact()
            .unwrap();

        let brand = sob.get_oid_name(
            dialoguer::Select::new()
                .with_prompt("Brand")
                .items(sob.get_oid_names().as_slice())
                .interact()
                .unwrap(),
        );

        let version = match dialoguer::Select::new()
            .with_prompt("SNMP Version")
            .default(0)
            .item("v2")
            .item("v3")
            .interact()
            .unwrap()
        {
            0 => SNMPVersion::V2,
            1 => SNMPVersion::V3,
            _ => unreachable!(),
        };

        if version == SNMPVersion::V2 {
            community = dialoguer::Input::<String>::new()
                .with_prompt("Community")
                .interact()
                .unwrap();
        } else {
            auth = match dialoguer::Select::new()
                .with_prompt("SNMP Authentication")
                .default(0)
                .item("MD5")
                .item("SHA1")
                .item("SHA224")
                .item("SHA256")
                .item("SHA384")
                .item("SHA512")
                .interact()
                .unwrap()
            {
                0 => SNMPAuth::Md5,
                1 => SNMPAuth::Sha1,
                2 => SNMPAuth::Sha224,
                3 => SNMPAuth::Sha256,
                4 => SNMPAuth::Sha384,
                5 => SNMPAuth::Sha512,
                _ => unreachable!(),
            };

            username = dialoguer::Input::<String>::new()
                .with_prompt("Username")
                .interact()
                .unwrap();

            password = dialoguer::Password::new()
                .with_prompt("Password (blank to prompt each time)")
                .allow_empty_password(true)
                .with_confirmation("Confirm Password", "Passwords do not match")
                .interact()
                .unwrap();

            encryption = match dialoguer::Select::new()
                .with_prompt("SNMP Encryption")
                .default(0)
                .item("None")
                .item("DES")
                .item("AES128")
                .item("AES192")
                .item("AES256")
                .interact()
                .unwrap()
            {
                0 => SNMPEncryption::None,
                1 => SNMPEncryption::Des,
                2 => SNMPEncryption::Aes128,
                3 => SNMPEncryption::Aes192,
                4 => SNMPEncryption::Aes256,
                _ => unreachable!(),
            };

            if encryption != SNMPEncryption::None {
                encryption_pass = dialoguer::Password::new()
                    .with_prompt("Encryption Password")
                    .with_confirmation("Confirm Password", "Passwords do not match")
                    .interact()
                    .unwrap();
            }
        }

        Self {
            name,
            ip,
            ports,
            brand,
            version,
            community,
            auth,
            auth_user: username,
            auth_pass: password,
            encryption,
            encryption_pass,
        }
    }

    pub(crate) fn get_auth_protocol(&self) -> SNMPAuth {
        self.auth
    }

    pub(crate) fn get_auth_password(&self) -> &[u8] {
        &self.auth_pass.as_bytes()
    }

    pub(crate) fn get_community(&self) -> &str {
        &self.community
    }

    pub(crate) fn get_socket_addr(&self) -> SocketAddr {
        let ip_addr: IpAddr = self.ip.parse().expect("Invalid IP address");
        SocketAddr::new(ip_addr, 161)
    }

    pub(crate) fn get_oid(&self) -> Vec<u64> {
        let sob = SwitchOidBuilder::new();
        let oid = sob
            .get_switch_oid(self.brand.clone())
            .expect("Invalid brand");

        // Split the oid by '.' and convert each part to a u32
        oid.split('.').map(|x| x.parse::<u64>().unwrap()).collect()
    }

    pub(crate) fn get_ports(&self) -> Vec<u64> {
        let ports_input = dialoguer::Input::<String>::new()
            .with_prompt("List of ports (ex: 1-6,8,10-12)")
            .default(format!("1-{}", self.ports))
            .interact()
            .unwrap();

        Switch::parse_ports(ports_input).expect("Invalid port range")
    }

    pub(crate) fn get_privacy_protocol(&self) -> SNMPEncryption {
        self.encryption
    }

    pub(crate) fn get_privacy_password(&self) -> &[u8] {
        &self.encryption_pass.as_bytes()
    }
    pub(crate) fn get_username(&self) -> &[u8] {
        &self.auth_user.as_bytes()
    }

    pub(crate) fn get_version(&self) -> SNMPVersion {
        self.version
    }

    pub(crate) fn parse_ports(ports_input: String) -> Result<Vec<u64>, String> {
        let mut ports = Vec::new();

        for port in ports_input.split(',') {
            if port.contains(' ') {
                return Err(format!("Invalid port range: {}", ports_input));
            }

            if port.contains('-') {
                let range: Vec<&str> = port.split('-').collect();

                if range.len() != 2 {
                    return Err(format!("Invalid port range: {}", ports_input));
                }

                let start = range[0]
                    .parse::<u64>()
                    .map_err(|_| format!("Invalid port range: {}", ports_input))?;

                let end = range[1]
                    .parse::<u64>()
                    .map_err(|_| format!("Invalid port range: {}", ports_input))?;

                for i in start..=end {
                    ports.push(i);
                }
            } else {
                ports.push(
                    port.parse::<u64>()
                        .map_err(|_| format!("Invalid port range: {}", ports_input))?,
                );
            }
        }

        ports.sort();
        ports.dedup();
        Ok(ports)
    }

    fn set(&self, value: i64) -> std::io::Result<()> {
        let ports = self.get_ports();

        let client = Snmp::new();
        let results = client.set(self, ports, value);

        match results {
            Ok(results) => {
                println!("Status for {}:", self.name);
                for result in results {
                    println!("\t{}", result);
                }
            }
            Err(e) => {
                println!("Error getting status for {}: {}", self.name, e);
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for SwitchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.port < 10 {
            if self.status == "on" {
                write!(f, "Port: {}  - {}", self.port, self.status.green())
            } else {
                write!(f, "Port: {}  - {}", self.port, self.status.red())
            }
        } else {
            if self.status == "on" {
                write!(f, "Port: {} - {}", self.port, self.status.green())
            } else {
                write!(f, "Port: {} - {}", self.port, self.status.red())
            }
        }
    }
}

impl std::fmt::Display for SNMPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SNMPVersion::V2 => write!(f, "v2"),
            SNMPVersion::V3 => write!(f, "v3"),
        }
    }
}

impl std::fmt::Display for SNMPAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SNMPAuth::Md5 => write!(f, "Md5"),
            SNMPAuth::Sha1 => write!(f, "SHA1"),
            SNMPAuth::Sha224 => write!(f, "SHA224"),
            SNMPAuth::Sha256 => write!(f, "SHA256"),
            SNMPAuth::Sha384 => write!(f, "SHA384"),
            SNMPAuth::Sha512 => write!(f, "SHA512"),
        }
    }
}

impl std::fmt::Display for SNMPEncryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SNMPEncryption::None => write!(f, "None"),
            SNMPEncryption::Des => write!(f, "DES"),
            SNMPEncryption::Aes128 => write!(f, "AES128"),
            SNMPEncryption::Aes192 => write!(f, "AES192"),
            SNMPEncryption::Aes256 => write!(f, "AES256"),
        }
    }
}

impl std::fmt::Display for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.version == SNMPVersion::V2 {
            return write!(
                f,
                "  Name: {}\n  Addr: {}\n  Brand: {}\n  Ports: {}\n  Version: {}\n  Community: {}\n",
                self.name, self.ip, self.brand, self.ports, self.version, self.community
            );
        } else {
            return write!(
                f,
                "  Name: {}\n  Addr: {}\n  Brand: {}\n  Ports: {}\n  Version: {}\n  Username: {}\n  Auth: {}\n  Encryption: {}\n",
                self.name, self.ip, self.brand, self.ports, self.version, self.auth_user, self.auth, self.encryption
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Valid input tests
    #[test]
    fn test_parse_ports_single() {
        let ports = Switch::parse_ports("1".to_string());
        assert_eq!(ports, Ok(vec![1]));
    }

    #[test]
    fn test_parse_ports_range() {
        let ports = Switch::parse_ports("1-6".to_string());
        assert_eq!(ports, Ok(vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn test_parse_mixed_input() {
        let ports = Switch::parse_ports("1-6,8,10-12".to_string());
        assert_eq!(ports, Ok(vec![1, 2, 3, 4, 5, 6, 8, 10, 11, 12]));
    }

    #[test]
    fn test_parse_duplicate_input() {
        let ports = Switch::parse_ports("1-6,8,10-12,1,2,3,4,5,6,8,10,11,12".to_string());
        assert_eq!(ports, Ok(vec![1, 2, 3, 4, 5, 6, 8, 10, 11, 12]));
    }

    // Invalid input tests
    #[test]
    fn test_parse_ports_invalid_range() {
        let ports = Switch::parse_ports("1-6-8".to_string());
        assert_eq!(ports, Err("Invalid port range: 1-6-8".to_string()));
    }

    #[test]
    fn test_parse_ports_invalid_range_format() {
        let ports = Switch::parse_ports("1-A".to_string());
        assert_eq!(ports, Err("Invalid port range: 1-A".to_string()));
    }

    #[test]
    fn test_parse_ports_invalid_format() {
        let ports = Switch::parse_ports("1 2 3".to_string());
        assert_eq!(ports, Err("Invalid port range: 1 2 3".to_string()));
    }

    #[test]
    fn test_parse_ports_invalid_port() {
        let ports = Switch::parse_ports("1-6,a".to_string());
        assert_eq!(ports, Err("Invalid port range: 1-6,a".to_string()));
    }
}
