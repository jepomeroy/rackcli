mod config;
mod device;
mod errors;
mod keyring;
mod rackcliargs;
mod snmp;
mod snmpv2;
mod snmpv3;
mod switch;
mod switch_oid;
mod utils;
mod wol;

use clap::Parser;
use config::read_config;
use openssl::provider::Provider;
use rackcliargs::RackCliArgs;
use switch::Switch;
use wol::Wol;

// Add commands
fn add_switch() {
    let mut config = read_config();
    let switch = Switch::create(config.get_switch_names());
    config.add_switch(switch);
    config.write_config();
}

fn add_wol_device() {
    let mut config = read_config();
    let wol = Wol::create(config.get_wol_names());
    config.add_wol(wol);
    config.write_config();
}

// Delete commands
fn delete_switch() {
    let mut config = read_config();
    config.delete_switch();
    config.write_config();
}

fn delete_wol_device() {
    let mut config = read_config();
    config.delete_wol();
    config.write_config();
}

// List commands
fn list_config() {
    let config = read_config();
    config.print_config();
}

fn list_switches() {
    let config = read_config();
    config.print_switches();
}

fn list_wols() {
    let config = read_config();
    config.print_wols();
}

// Update commands
fn update_switch() {
    let mut config = read_config();
    config.update_switch();
    config.write_config();
}

fn update_wol_device() {
    let mut config = read_config();
    config.update_wol();
    config.write_config();
}

// Enable commands
async fn enable_switch() {
    let mut config = read_config();
    config.enable_switch().await;
}

async fn enable_wol_device() {
    let mut config = read_config();
    config.enable_wol().await;
}

// Disable commands
async fn disable_switch() {
    let mut config = read_config();
    config.disable_switch().await;
}

// Status commands
async fn status_switch() {
    let mut config = read_config();
    config.get_switch_status().await;
}

#[tokio::main]
async fn main() {
    // Load OpenSSL legacy provider to enable DES and other legacy ciphers
    let _legacy = Provider::load(None, "legacy").expect("Failed to load OpenSSL legacy provider");
    let _default =
        Provider::load(None, "default").expect("Failed to load OpenSSL default provider");

    let args = RackCliArgs::parse();

    match args.device_type {
        rackcliargs::DeviceType::List => list_config(),
        rackcliargs::DeviceType::Switch(switch) => match switch.command {
            rackcliargs::SwitchSubCommand::Add => add_switch(),
            rackcliargs::SwitchSubCommand::Delete => delete_switch(),
            rackcliargs::SwitchSubCommand::List => list_switches(),
            rackcliargs::SwitchSubCommand::Update => update_switch(),
            // Async calls
            rackcliargs::SwitchSubCommand::Enable => enable_switch().await,
            rackcliargs::SwitchSubCommand::Disable => disable_switch().await,
            rackcliargs::SwitchSubCommand::Status => status_switch().await,
        },
        rackcliargs::DeviceType::Wol(wol) => match wol.command {
            rackcliargs::WolSubCommand::Add => add_wol_device(),
            rackcliargs::WolSubCommand::Delete => delete_wol_device(),
            rackcliargs::WolSubCommand::List => list_wols(),
            rackcliargs::WolSubCommand::Update => update_wol_device(),
            // Async calls
            rackcliargs::WolSubCommand::Enable => enable_wol_device().await,
        },
    }
}
