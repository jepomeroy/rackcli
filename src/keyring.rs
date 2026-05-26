//! System keyring helpers for storing per-switch SNMP credentials.

use std::fmt::Display;

use keyring::Entry;

const APP_NAME: &str = "rackcli";

/// Identifies which credential to read or write for a switch.
pub(crate) enum KeyRingType {
    /// SNMPv3 authentication password.
    Auth,
    /// SNMPv2c community string.
    Community,
    /// SNMPv3 privacy (encryption) password.
    Encrypt,
}

impl Display for KeyRingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyRingType::Auth => write!(f, "auth"),
            KeyRingType::Community => write!(f, "community"),
            KeyRingType::Encrypt => write!(f, "encrypt"),
        }
    }
}

/// Retrieves the credential stored under `{switch_name}/{key_type}` from the system keyring.
pub(crate) fn get_key(switch_name: &str, key_type: KeyRingType) -> Result<String, keyring::Error> {
    let entry = Entry::new(APP_NAME, &format!("{}/{}", switch_name, key_type))?;
    entry.get_password()
}

/// Stores or overwrites the credential under `{switch_name}/{key_type}` in the system keyring.
pub(crate) fn set_key(
    switch_name: &str,
    value: &str,
    key_type: KeyRingType,
) -> Result<(), keyring::Error> {
    let entry = Entry::new(APP_NAME, &format!("{}/{}", switch_name, key_type))?;
    entry.set_password(value)
}

/// Deletes the credential stored under `{switch_name}/{key_type}` from the system keyring.
pub(crate) fn remove_key(switch_name: &str, key_type: KeyRingType) -> Result<(), keyring::Error> {
    let entry = Entry::new(APP_NAME, &format!("{}/{}", switch_name, key_type))?;
    entry.delete_credential()
}
