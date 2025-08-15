use std::time::Duration;

use crate::errors::SnmpError;
use crate::snmpv2::SnmpV2Client;
use crate::snmpv3::SnmpV3Client;
use crate::switch::{SNMPVersion, Switch, SwitchResult};
use futures::executor::block_on;
use snmp2::Oid;
use tokio::spawn;

pub trait SnmpClient {
    async fn get(self, oid: Oid, port: u64) -> Result<SwitchResult, SnmpError>;
    async fn set(self, oid: Oid, value: i64, port: u64) -> Result<SwitchResult, SnmpError>;
}

pub struct Snmp {}

impl Snmp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, switch: &Switch, ports: Vec<u64>) -> Result<Vec<SwitchResult>, SnmpError> {
        match switch.get_version() {
            SNMPVersion::V2 => {
                let mut switch_results = Vec::new();
                let mut futures = Vec::new();
                for port in ports.iter() {
                    let oid = Snmp::make_oid(switch.get_oid(), *port);
                    let v2 = SnmpV2Client::new(
                        switch.get_socket_addr(),
                        switch.get_community().as_bytes(),
                        Some(Duration::from_secs(5)),
                    )?;

                    futures.push(spawn(v2.get(oid, *port)));
                }

                let mut results = Vec::new();
                for future in futures {
                    results.push(block_on(future));
                }

                // Print the results
                for result_result in results {
                    match result_result {
                        Ok(result) => {
                            if let Ok(switch_result) = result {
                                switch_results.push(switch_result.clone());
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }

                Ok(switch_results)
            }
            SNMPVersion::V3 => {
                let mut switch_results = Vec::new();
                let mut futures = Vec::new();
                for port in ports.iter() {
                    let oid = Snmp::make_oid(switch.get_oid(), *port);
                    let v3 = SnmpV3Client::new(
                        switch.get_socket_addr(),
                        switch.get_username(),
                        switch.get_auth_password(),
                        switch.get_auth_protocol(),
                        switch.get_privacy_protocol(),
                        switch.get_privacy_password(),
                        Some(Duration::from_secs(5)),
                    )?;

                    futures.push(spawn(v3.get(oid, *port)));
                }

                let mut results = Vec::new();
                for future in futures {
                    results.push(block_on(future));
                }

                // Print the results
                for result_result in results {
                    match result_result {
                        Ok(result) => {
                            if let Ok(switch_result) = result {
                                switch_results.push(switch_result.clone());
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }

                Ok(switch_results)
            }
        }
    }

    pub fn set(
        &self,
        switch: &Switch,
        ports: Vec<u64>,
        value: i64,
    ) -> Result<Vec<SwitchResult>, SnmpError> {
        match switch.get_version() {
            SNMPVersion::V2 => {
                let mut switch_results = Vec::new();
                let mut futures = Vec::new();
                for port in ports.iter() {
                    let oid = Snmp::make_oid(switch.get_oid(), *port);
                    let v2 = SnmpV2Client::new(
                        switch.get_socket_addr(),
                        switch.get_community().as_bytes(),
                        Some(Duration::from_secs(5)),
                    )?;

                    futures.push(spawn(v2.set(oid, value, *port)));
                }

                let mut results = Vec::new();
                for future in futures {
                    results.push(block_on(future));
                }

                // Print the results
                for result_result in results {
                    match result_result {
                        Ok(result) => {
                            if let Ok(switch_result) = result {
                                switch_results.push(switch_result.clone());
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }

                Ok(switch_results)
            }
            SNMPVersion::V3 => {
                let mut switch_results = Vec::new();
                let mut futures = Vec::new();
                for port in ports.iter() {
                    let oid = Snmp::make_oid(switch.get_oid(), *port);
                    let v3 = SnmpV3Client::new(
                        switch.get_socket_addr(),
                        switch.get_username(),
                        switch.get_auth_password(),
                        switch.get_auth_protocol(),
                        switch.get_privacy_protocol(),
                        switch.get_privacy_password(),
                        Some(Duration::from_secs(5)),
                    )?;

                    futures.push(spawn(v3.set(oid, value, *port)));
                }

                let mut results = Vec::new();
                for future in futures {
                    results.push(block_on(future));
                }

                // Print the results
                for result_result in results {
                    match result_result {
                        Ok(result) => {
                            if let Ok(switch_result) = result {
                                switch_results.push(switch_result.clone());
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }

                Ok(switch_results)
            }
        }
    }

    fn make_oid(oid_vec: Vec<u64>, port: u64) -> Oid<'static> {
        let mut new_vec = oid_vec.clone();
        new_vec.push(port);

        Oid::from(new_vec.as_slice()).expect("Invalid OID")
    }
}
