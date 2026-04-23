use std::fmt::Display;

use keyring::Entry;

const APP_NAME: &str = "rackcli";

pub(crate) enum KeyRingType {
    Auth,
    Community,
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

pub(crate) fn get_key(switch_name: &str, key_type: KeyRingType) -> Result<String, keyring::Error> {
    let entry = Entry::new(APP_NAME, &format!("{}/{}", switch_name, key_type))?;
    entry.get_password()
}

pub(crate) fn set_key(
    switch_name: &str,
    value: &str,
    key_type: KeyRingType,
) -> Result<(), keyring::Error> {
    let entry = Entry::new(APP_NAME, &format!("{}/{}", switch_name, key_type))?;
    entry.set_password(value)
}

pub(crate) fn remove_key(switch_name: &str, key_type: KeyRingType) -> Result<(), keyring::Error> {
    let entry = Entry::new(APP_NAME, &format!("{}/{}", switch_name, key_type))?;
    entry.delete_credential()
}
