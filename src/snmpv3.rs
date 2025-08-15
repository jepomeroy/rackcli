use crate::switch::{SNMPAuth, SNMPEncryption};
use crate::utils::get_status;
use crate::{errors::SnmpError, snmp::SnmpClient, switch::SwitchResult};
use snmp2::v3::{Auth::AuthPriv, AuthProtocol, Cipher, Security};
use snmp2::{Oid, SyncSession, Value};
use std::{net::SocketAddr, time::Duration};

pub struct SnmpV3Client {
    session: SyncSession,
}

impl SnmpV3Client {
    pub fn new(
        socket_addr: SocketAddr,
        username: &[u8],
        password: &[u8],
        auth_protocol: SNMPAuth,
        encryption: SNMPEncryption,
        encryption_key: &[u8],
        timeout: Option<Duration>,
    ) -> Result<Self, SnmpError> {
        if password.is_empty() {
            return Err(SnmpError::SessionError(
                "Authentication password cannot be empty for SNMP v3".to_string(),
            ));
        }
        let auth = match auth_protocol {
            SNMPAuth::Md5 => AuthProtocol::Md5,
            SNMPAuth::Sha1 => AuthProtocol::Sha1,
            SNMPAuth::Sha224 => AuthProtocol::Sha224,
            SNMPAuth::Sha256 => AuthProtocol::Sha256,
            SNMPAuth::Sha384 => AuthProtocol::Sha384,
            SNMPAuth::Sha512 => AuthProtocol::Sha512,
        };

        let enc = match encryption {
            SNMPEncryption::Des => Cipher::Des,
            SNMPEncryption::Aes128 => Cipher::Aes128,
            SNMPEncryption::Aes192 => Cipher::Aes192,
            SNMPEncryption::Aes256 => Cipher::Aes256,
            _ => Cipher::Des, // Default to DES if no encryption is specified
        };

        let security = if encryption == SNMPEncryption::None {
            Security::new(username, password).with_auth_protocol(auth)
        } else {
            Security::new(username, password)
                .with_auth_protocol(auth)
                .with_auth(AuthPriv {
                    cipher: enc,
                    privacy_password: encryption_key.to_vec(),
                })
        };

        match SyncSession::new_v3(socket_addr, timeout, 0, security) {
            Ok(mut session) => {
                // First attempt to discover the engine ID
                match session.init() {
                    Ok(_) => Ok(Self { session }),
                    Err(e) => Err(SnmpError::SessionError(format!(
                        "Session init failed: {}",
                        e
                    ))),
                }
            }
            Err(e) => Err(SnmpError::SessionError(format!(
                "Session creation failed: {}",
                e
            ))),
        }
    }
}

impl SnmpClient for SnmpV3Client {
    async fn get(mut self, oid: Oid<'_>, port: u64) -> Result<SwitchResult, SnmpError> {
        let response = self.session.get(&oid);

        let response = match response {
            Err(snmp2::Error::AuthUpdated) => {
                // Authentication keys have been updated, retry the GET request
                self.session.get(&oid)
            }
            other => other,
        };

        match response {
            Ok(mut pdu) => {
                if let Some((_, value)) = pdu.varbinds.next() {
                    let status = get_status(value);

                    return Ok(SwitchResult { port, status });
                }

                Err(SnmpError::OperationError(
                    "No value found in response".to_string(),
                ))
            }
            Err(e) => Err(SnmpError::OperationError(e.to_string())),
        }
    }

    async fn set(mut self, oid: Oid<'_>, value: i64, port: u64) -> Result<SwitchResult, SnmpError> {
        let port_value = Value::Integer(value);
        let response = self.session.set(&[(&oid, port_value)]);

        let response = match response {
            Err(snmp2::Error::AuthUpdated) => {
                // Authentication keys have been updated, retry the SET request
                let retry_value = Value::Integer(value);
                self.session.set(&[(&oid, retry_value)])
            }
            other => other,
        };

        match response {
            Ok(mut pdu) => {
                if let Some((_, value)) = pdu.varbinds.next() {
                    let status = get_status(value);

                    return Ok(SwitchResult { port, status });
                }

                Err(SnmpError::OperationError(
                    "No value found in response".to_string(),
                ))
            }

            Err(e) => Err(SnmpError::OperationError(e.to_string())),
        }
    }
}
