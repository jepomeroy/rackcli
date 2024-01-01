use dialoguer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Switch {
    pub name: String,
    ip: String,
    ports: u8,
    username: String,
    password: String,
}

impl Switch {
    pub fn create(switch_names: Vec<String>) -> Self {
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

        let ports = dialoguer::Input::<u8>::new()
            .with_prompt("Ports")
            .interact()
            .unwrap();

        let username = dialoguer::Input::<String>::new()
            .with_prompt("Username")
            .interact()
            .unwrap();

        let password = dialoguer::Password::new()
            .with_prompt("Password")
            .with_confirmation("Confirm Password", "Passwords do not match")
            .interact()
            .unwrap();

        Self {
            name,
            ip,
            ports,
            username,
            password,
        }
    }

    pub fn update(&mut self) {
        let ip = dialoguer::Input::<String>::new()
            .with_prompt("IP")
            .default(self.ip.clone())
            .interact()
            .unwrap();

        let ports = dialoguer::Input::<u8>::new()
            .with_prompt("Ports")
            .default(self.ports)
            .interact()
            .unwrap();

        let username = dialoguer::Input::<String>::new()
            .with_prompt("Username")
            .default(self.username.clone())
            .interact()
            .unwrap();

        let password = dialoguer::Password::new()
            .with_prompt("Password")
            .with_confirmation("Confirm Password", "Passwords do not match")
            .interact()
            .unwrap();

        self.ip = ip;
        self.ports = ports;
        self.username = username;
        self.password = password;
    }
}

impl std::fmt::Display for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  Name: {}\n  Addr: {}\n  Ports: {}\n  Username: {}\n",
            self.name, self.ip, self.ports, self.username
        )
    }
}
