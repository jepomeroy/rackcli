//! Core trait for rack devices managed by this CLI.

/// A rack device that supports remote control and interactive configuration.
pub trait Device {
    /// Deactivates the device (e.g., cuts PoE power to a switch port).
    ///
    /// Returns an error if the operation is unsupported or fails.
    async fn disable(&mut self) -> std::io::Result<()>;

    /// Activates the device (e.g., restores PoE power or sends a Wake-on-LAN packet).
    async fn enable(&mut self) -> std::io::Result<()>;

    /// Prints the current operational status of the device to stdout.
    async fn status(&mut self);

    /// Interactively prompts the user to edit this device's stored configuration fields.
    fn update(&mut self);
}
