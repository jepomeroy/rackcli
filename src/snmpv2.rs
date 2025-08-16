use crate::utils::get_status;
use crate::{errors::SnmpError, snmp::SnmpClient, switch::SwitchResult};
use snmp2::{Oid, SyncSession, Value};
use std::{net::SocketAddr, time::Duration};

pub struct SnmpV2Client {
    session: SyncSession,
}

impl SnmpV2Client {
    pub fn new(
        socket_addr: SocketAddr,
        community: &[u8],
        timeout: Option<Duration>,
    ) -> Result<Self, SnmpError> {
        match SyncSession::new_v2c(socket_addr, community, timeout, 0) {
            Ok(session) => Ok(Self { session }),
            Err(e) => Err(SnmpError::SessionError(e.to_string())),
        }
    }
}

impl SnmpClient for SnmpV2Client {
    async fn get(mut self, oid: Oid<'_>, port: u64) -> Result<SwitchResult, SnmpError> {
        let response = self.session.get(&oid);

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
