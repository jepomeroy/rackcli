// use clent::CLient;

use std::net::SocketAddr;

use csnmp::{ObjectIdentifier, ObjectValue, Snmp2cClient};

use crate::{snmp::SnmpClient, switch::SwitchResult};

pub struct SnmpV2Client {}

impl SnmpClient for SnmpV2Client {
    async fn get(
        self,
        socket_addr: SocketAddr,
        community: Vec<u8>,
        oid: ObjectIdentifier,
        port: u32,
    ) -> Result<SwitchResult, String> {
        let client_result = Snmp2cClient::new(socket_addr, community, None, None, 0).await;

        match client_result {
            Ok(client) => {
                let result = client.get(oid).await;
                match result {
                    Ok(val) => val
                        .as_i32()
                        .map(|v| {
                            if v == 1 {
                                Ok(SwitchResult {
                                    port,
                                    status: "on".to_string(),
                                })
                            } else if v == 2 {
                                Ok(SwitchResult {
                                    port,
                                    status: "off".to_string(),
                                })
                            } else {
                                Err(format!("Invalid value: {}", v))
                            }
                        })
                        .unwrap_or_else(|| Err(format!("Invalid value: {:?}", val))),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    }

    async fn set(
        self,
        socket_addr: SocketAddr,
        community: Vec<u8>,
        oid: ObjectIdentifier,
        value: i32,
        port: u32,
    ) -> Result<SwitchResult, String> {
        let client_result = Snmp2cClient::new(socket_addr, community, None, None, 0).await;

        match client_result {
            Ok(client) => {
                let object_value = ObjectValue::Integer(value);

                let result = client.set(oid, object_value).await;
                match result {
                    Ok(val) => val
                        .as_i32()
                        .map(|v| {
                            if v == 1 {
                                Ok(SwitchResult {
                                    port,
                                    status: "on".to_string(),
                                })
                            } else if v == 2 {
                                Ok(SwitchResult {
                                    port,
                                    status: "off".to_string(),
                                })
                            } else {
                                Err(format!("Invalid value: {}", v))
                            }
                        })
                        .unwrap_or_else(|| Err(format!("Invalid value: {:?}", val))),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    }
}
