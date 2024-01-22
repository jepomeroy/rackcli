use crate::device::Device;
use crate::switch::Switch;
use crate::wol::Wol;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub switches: Vec<Switch>,
    pub wols: Vec<Wol>,
}

pub fn read_config() -> Config {
    match Config::get_config_path() {
        Ok(config_path) => match fs::read_to_string(config_path) {
            Ok(toml_content) => {
                let data: Config = toml::from_str(&toml_content).unwrap();
                data
            }
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
    pub fn new() -> Self {
        Self {
            switches: vec![],
            wols: vec![],
        }
    }

    fn get_config_path() -> Result<PathBuf, String> {
        let base_dirs =
            ProjectDirs::from("com", "epomeroy", "rackcli").expect("Configuration path to exist");
        fs::create_dir_all(base_dirs.config_dir()).expect("Create directories");
        Ok(base_dirs.config_dir().join("config.toml"))
    }

    pub fn print_config(&self) {
        self.print_switches();
        self.print_wols();
    }

    pub fn write_config(&self) {
        match Config::get_config_path() {
            Ok(config_path) => {
                let toml_content = toml::to_string(&self);

                match toml_content {
                    Ok(s) => std::fs::write(config_path, s).unwrap(),
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    ////////////////////////
    /// Switch functions
    ////////////////////////
    pub fn add_switch(&mut self, switch: Switch) {
        self.switches.push(switch);
    }

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

        match dialoguer::Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete {}?",
                switch_names[switch_name]
            ))
            .interact()
        {
            Ok(_) => {
                self.switches.remove(switch_name);
            }
            Err(_) => return,
        }
    }

    pub fn disable_switch(&self) {
        if let Some(switch_index) = self.select_switch("Switch to disable".to_string()) {
            let _ = self.switches[switch_index].disable();
        }
    }

    pub fn enable_switch(&self) {
        if let Some(switch_index) = self.select_switch("Switch to enable".to_string()) {
            let _ = self.switches[switch_index].enable();
        }
    }

    pub fn get_switch_names(&self) -> Vec<String> {
        let mut switch_names: Vec<String> = self
            .switches
            .iter()
            .map(|switch| switch.name.clone())
            .collect();

        switch_names.sort();

        switch_names
    }

    pub fn get_switch_status(&self) {
        if let Some(switch_index) = self.select_switch("Switch to get status".to_string()) {
            let _ = self.switches[switch_index].status();
        }
    }

    pub fn update_switch(&mut self) {
        if let Some(switch_index) = self.select_switch("Switch to update".to_string()) {
            self.switches[switch_index].update();
        }
    }

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

    ////////////////////////
    /// Wol functions
    ////////////////////////
    pub fn add_wol(&mut self, wol: Wol) {
        self.wols.push(wol);
    }

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

        match dialoguer::Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete {}?",
                wol_names[wol_name]
            ))
            .interact()
        {
            Ok(_) => {
                self.wols.remove(wol_name);
            }
            Err(_) => return,
        }
    }

    pub fn enable_wol(&self) {
        if let Some(wol_index) = self.select_wol("Wol device to enable".to_string()) {
            let _ = self.wols[wol_index].enable();
        }
    }

    pub fn get_wol_names(&self) -> Vec<String> {
        let mut wol_names: Vec<String> = self.wols.iter().map(|wol| wol.name.clone()).collect();
        wol_names.sort();

        wol_names
    }

    pub fn update_wol(&mut self) {
        if let Some(wol_index) = self.select_wol("Wol device to update".to_string()) {
            self.wols[wol_index].update();
        }
    }

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
