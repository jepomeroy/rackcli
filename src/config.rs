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

    pub fn add_switch(&mut self, switch: Switch) {
        self.switches.push(switch);
    }

    pub fn add_wol(&mut self, wol: Wol) {
        self.wols.push(wol);
    }

    pub fn delete_switch(&mut self) {
        if self.switches.is_empty() {
            println!("No Switches configured");
            return;
        }

        let switch_names: Vec<String> = self
            .switches
            .iter()
            .map(|switch| switch.name.clone())
            .collect();

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

    pub fn delete_wol(&mut self) {
        if self.wols.is_empty() {
            println!("No Wake-on-Lan devices configured");
            return;
        }

        let wol_names: Vec<String> = self.wols.iter().map(|wol| wol.name.clone()).collect();

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
        if self.wols.is_empty() {
            println!("No Wake-on-Lan devices configured");
            return;
        }

        let wol_names: Vec<String> = self.wols.iter().map(|wol| wol.name.clone()).collect();

        let wol_name = dialoguer::Select::new()
            .with_prompt("Wol device to enable")
            .default(0)
            .items(&wol_names[..])
            .interact()
            .unwrap();

        let wol = &self.wols[wol_name];
        let _ = wol.wake_on_lan();
    }

    pub fn get_switch_names(&self) -> Vec<String> {
        self.switches
            .iter()
            .map(|switch| switch.name.clone())
            .collect()
    }

    pub fn get_wol_names(&self) -> Vec<String> {
        self.wols.iter().map(|wol| wol.name.clone()).collect()
    }

    pub fn update_switch(&mut self) {
        if self.switches.is_empty() {
            println!("No Switches configured");
            return;
        }

        let switch_names: Vec<String> = self
            .switches
            .iter()
            .map(|switch| switch.name.clone())
            .collect();

        let switch_name = dialoguer::Select::new()
            .with_prompt("Switch to update")
            .default(0)
            .items(&switch_names[..])
            .interact()
            .unwrap();

        self.switches[switch_name].update();
    }

    pub fn update_wol(&mut self) {
        if self.wols.is_empty() {
            println!("No Wake-on-Lan devices configured");
            return;
        }

        let wol_names: Vec<String> = self.wols.iter().map(|wol| wol.name.clone()).collect();

        let wol_name = dialoguer::Select::new()
            .with_prompt("Wol device to update")
            .default(0)
            .items(&wol_names[..])
            .interact()
            .unwrap();

        self.wols[wol_name].update();
    }

    pub fn print_config(&self) {
        self.print_switches();
        self.print_wols();
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

    fn get_config_path() -> Result<PathBuf, String> {
        let base_dirs = ProjectDirs::from("com", "epomeroy", "rackcli").expect("Foo");
        fs::create_dir_all(base_dirs.config_dir()).expect("Create directories");
        Ok(base_dirs.config_dir().join("config.toml"))
    }
}
