use dialoguer;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Wol {
    pub name: String,
    mac: String,
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

    pub fn update(&mut self) {
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

    pub fn get_octets(&self) -> Vec<u8> {
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
