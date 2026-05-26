//! Application configuration serialised as TOML at `~/.config/rackcli/config.toml`.

use crate::device::Device;
use crate::switch::Switch;
use crate::wol::Wol;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

/// Top-level application configuration containing all managed switches and WoL devices.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub switches: Vec<Switch>,
    pub wols: Vec<Wol>,
}

/// Loads configuration from disk.
///
/// Creates and writes an empty config when the file does not yet exist.
/// Falls back to an empty in-memory config (without overwriting the file) when the TOML is
/// malformed, so a corrupt config is never silently destroyed.
pub fn read_config() -> Config {
    match Config::get_config_path() {
        Ok(config_path) => match fs::read_to_string(config_path) {
            Ok(toml_content) => match toml::from_str::<Config>(&toml_content) {
                Ok(mut data) => {
                    data.switches.iter_mut().for_each(|switch| {
                        switch.get_keys();
                    });
                    data
                }
                Err(e) => {
                    println!("Error parsing config file: {}", e);
                    Config::new()
                }
            },
            Err(_) => {
                println!("No config file found, creating one");
                let config = Config::new();
                config.write_config();
                config
            }
        },
        Err(_) => {
            println!("No config file found, creating one");
            let config = Config::new();
            config.write_config();
            config
        }
    }
}

impl Config {
    /// Returns an empty configuration with no switches or WoL devices.
    pub fn new() -> Self {
        Self {
            switches: vec![],
            wols: vec![],
        }
    }

    /// Returns the path to the config file, creating the parent directory if needed.
    fn get_config_path() -> Result<PathBuf, String> {
        let base_dirs = ProjectDirs::from("com", "jepomeroy", "rackcli")
            .ok_or_else(|| "Could not determine config directory (is $HOME set?)".to_string())?;
        fs::create_dir_all(base_dirs.config_dir()).expect("Create directories");
        Ok(base_dirs.config_dir().join("config.toml"))
    }

    /// Prints all switches and WoL devices to stdout.
    pub fn print_config(&self) {
        self.print_switches();
        self.print_wols();
    }

    /// Serialises the configuration to TOML and writes it to disk with mode `0600`.
    pub fn write_config(&self) {
        match Config::get_config_path() {
            Ok(config_path) => {
                let toml_content = toml::to_string(&self);

                match toml_content {
                    Ok(s) => {
                        match fs::OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .mode(0o600)
                            .open(&config_path)
                        {
                            Ok(mut file) => {
                                if let Err(e) = file.write_all(s.as_bytes()) {
                                    println!("Error writing config file: {}", e);
                                }
                            }
                            Err(e) => println!("Error opening config file: {}", e),
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    //
    // Switch functions
    //
    /// Stores credentials in the keyring and appends the switch to the config.
    pub fn add_switch(&mut self, switch: Switch) {
        switch.set_keys();
        self.switches.push(switch);
    }

    /// Interactively selects a switch to remove, prompts for confirmation, then removes it and
    /// cleans up its keyring entries.
    pub fn delete_switch(&mut self) {
        if self.switches.is_empty() {
            println!("No Switches configured");
            return;
        }

        let switch_names = self.get_switch_names();

        let switch_name = dialoguer::Select::new()
            .with_prompt("Switch to delete")
            .default(0)
            .items(&switch_names[..])
            .interact()
            .unwrap();

        if dialoguer::Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete {}?",
                switch_names[switch_name].clone()
            ))
            .interact()
            .unwrap_or(false)
        {
            let removed_switch = self.switches.remove(switch_name);
            removed_switch.remove_keys();
        }
    }

    /// Interactively selects a switch and disables PoE on its ports via SNMP.
    pub async fn disable_switch(&mut self) {
        if let Some(switch_index) = self.select_switch("Switch to disable".to_string()) {
            let _ = self.switches[switch_index].disable().await;
        }
    }

    /// Interactively selects a switch and enables PoE on its ports via SNMP.
    pub async fn enable_switch(&mut self) {
        if let Some(switch_index) = self.select_switch("Switch to enable".to_string()) {
            let _ = self.switches[switch_index].enable().await;
        }
    }

    /// Returns the names of all configured switches in their stored order.
    pub fn get_switch_names(&self) -> Vec<String> {
        let switch_names: Vec<String> = self
            .switches
            .iter()
            .map(|switch| switch.name.clone())
            .collect();

        // switch_names.sort();

        switch_names
    }

    /// Interactively selects a switch and prints the PoE status of its ports.
    pub async fn get_switch_status(&mut self) {
        if let Some(switch_index) = self.select_switch("Switch to get status".to_string()) {
            self.switches[switch_index].status().await;
        }
    }

    /// Interactively selects a switch, edits its fields, then refreshes keyring entries.
    pub fn update_switch(&mut self) {
        if let Some(switch_index) = self.select_switch("Switch to update".to_string()) {
            self.switches[switch_index].update();
            self.switches[switch_index].set_keys();
        }
    }

    /// Prints all configured switches to stdout.
    pub fn print_switches(&self) {
        println!("Switches:");

        if self.switches.is_empty() {
            println!("  No Switches configured\n");
        } else {
            for switch in &self.switches {
                println!("{}", switch);
            }
        }
    }

    /// Presents an interactive selection prompt and returns the chosen switch's index, or `None`
    /// if no switches are configured.
    fn select_switch(&self, prompt: String) -> Option<usize> {
        if self.switches.is_empty() {
            println!("No Switches configured");
            return None;
        }

        let switch_names = self.get_switch_names();

        let switch_index = dialoguer::Select::new()
            .with_prompt(prompt)
            .default(0)
            .items(&switch_names[..])
            .interact()
            .unwrap();

        Some(switch_index)
    }

    //
    // Wol functions
    //
    /// Appends a WoL device to the config.
    pub fn add_wol(&mut self, wol: Wol) {
        self.wols.push(wol);
    }

    /// Interactively selects a WoL device to remove and prompts for confirmation before deleting.
    pub fn delete_wol(&mut self) {
        if self.wols.is_empty() {
            println!("No Wake-on-Lan devices configured");
            return;
        }

        let wol_names = self.get_wol_names();

        let wol_name = dialoguer::Select::new()
            .with_prompt("Wol device to delete")
            .default(0)
            .items(&wol_names[..])
            .interact()
            .unwrap();

        if dialoguer::Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete {}?",
                wol_names[wol_name]
            ))
            .interact()
            .unwrap_or(false)
        {
            self.wols.remove(wol_name);
        }
    }

    /// Interactively selects a WoL device and sends its magic packet.
    pub async fn enable_wol(&mut self) {
        if let Some(wol_index) = self.select_wol("Wol device to enable".to_string()) {
            let _ = self.wols[wol_index].enable().await;
        }
    }

    /// Returns the names of all configured WoL devices, sorted alphabetically.
    pub fn get_wol_names(&self) -> Vec<String> {
        let mut wol_names: Vec<String> = self.wols.iter().map(|wol| wol.name.clone()).collect();
        wol_names.sort();

        wol_names
    }

    /// Interactively selects a WoL device and edits its fields.
    pub fn update_wol(&mut self) {
        if let Some(wol_index) = self.select_wol("Wol device to update".to_string()) {
            self.wols[wol_index].update();
        }
    }

    /// Prints all configured WoL devices to stdout.
    pub fn print_wols(&self) {
        println!("Wols:");

        if self.wols.is_empty() {
            println!("  No Wake-on-Lan devices configured");
        } else {
            for wol in &self.wols {
                println!("{}", wol);
            }
        }
    }

    /// Presents an interactive selection prompt and returns the chosen WoL device's index, or
    /// `None` if no WoL devices are configured.
    fn select_wol(&self, prompt: String) -> Option<usize> {
        if self.wols.is_empty() {
            println!("No Wake-on-Lan devices configured");
            return None;
        }

        let wol_names = self.get_wol_names();

        let wol_index = dialoguer::Select::new()
            .with_prompt(prompt)
            .default(0)
            .items(&wol_names[..])
            .interact()
            .unwrap();

        Some(wol_index)
    }
}
