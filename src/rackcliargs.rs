//! Command-line argument definitions, parsed by clap.

use clap::{Args, Parser, Subcommand};

/// Top-level CLI arguments.
#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct RackCliArgs {
    #[clap(subcommand)]
    pub device_type: DeviceType,
}

/// The top-level device category subcommand.
#[derive(Subcommand, Debug)]
pub enum DeviceType {
    /// Add, Delete, List, Update, Enable, Disable, or get Status for Switch devices
    Switch(SwitchCmd),
    /// Add, Delete, List, Update, or Enable Wake-On-Lan devices
    Wol(WolCmd),
    /// List all devices
    List,
}

/// Switch subcommand with a required action.
#[derive(Args, Debug)]
pub struct SwitchCmd {
    #[clap(subcommand)]
    pub command: SwitchSubCommand,
}

/// Actions available for PoE switch devices.
#[derive(Subcommand, Debug)]
pub enum SwitchSubCommand {
    /// Add a new Switch device
    Add,
    /// Delete a Switch device
    Delete,
    /// List all Switch devices
    List,
    /// Update a Switch device
    Update,
    /// Enable Switch device
    Enable,
    /// Disable Switch device
    Disable,
    /// Get status of Switch device
    Status,
}

/// WoL subcommand with a required action.
#[derive(Args, Debug)]
pub struct WolCmd {
    #[clap(subcommand)]
    pub command: WolSubCommand,
}

/// Actions available for Wake-on-LAN devices.
#[derive(Subcommand, Debug)]
pub enum WolSubCommand {
    /// Add a new Wake-On-Lan device
    Add,
    /// Delete a Wake-On-Lan device
    Delete,
    /// List all Wake-On-Lan devices
    List,
    /// Update a Wake-On-Lan device
    Update,
    /// Enable Wake-On-Lan device
    Enable,
}
