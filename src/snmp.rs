use std::net::SocketAddr;

use crate::snmpv2::SnmpV2Client;
use crate::switch::{SNMPVersion, Switch, SwitchResult};
use csnmp::ObjectIdentifier;
use futures::executor::block_on;
use tokio::spawn;

pub trait SnmpClient {
    async fn get(
        self,
        socket_addr: SocketAddr,
        community: Vec<u8>,
        oid: ObjectIdentifier,
        port: u32,
    ) -> Result<SwitchResult, String>;
    async fn set(
        self,
        socket_addr: SocketAddr,
        community: Vec<u8>,
        oid: ObjectIdentifier,
        value: i32,
        port: u32,
    ) -> Result<SwitchResult, String>;
}

pub struct Snmp {}

impl Snmp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, switch: &Switch, ports: Vec<u32>) -> Result<Vec<SwitchResult>, String> {
        match switch.get_version() {
            SNMPVersion::V2 => {
                let mut switch_results = Vec::new();
                let mut futures = Vec::new();
                for port in ports.iter() {
                    let socket_addr = switch.get_socket_addr();
                    let oid = Snmp::make_oid(&switch, *port);
                    let v2 = SnmpV2Client {};

                    futures.push(spawn(v2.get(
                        socket_addr,
                        switch.get_community().into(),
                        oid,
                        *port,
                    )));
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
            _ => todo!(),
        }
    }

    pub fn set(
        &self,
        switch: &Switch,
        ports: Vec<u32>,
        value: i32,
    ) -> Result<Vec<SwitchResult>, String> {
        match switch.get_version() {
            SNMPVersion::V2 => {
                let mut switch_results = Vec::new();
                let mut futures = Vec::new();
                for port in ports.iter() {
                    let socket_addr = switch.get_socket_addr();
                    let oid = Snmp::make_oid(&switch, *port);
                    let v2 = SnmpV2Client {};

                    futures.push(spawn(v2.set(
                        socket_addr,
                        switch.get_community().into(),
                        oid,
                        value,
                        *port,
                    )));
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
            _ => todo!(),
        }
    }

    fn make_oid(switch: &Switch, port: u32) -> ObjectIdentifier {
        let mut octets_vec = switch.get_oid();
        octets_vec.push(port);

        ObjectIdentifier::try_from(octets_vec.as_slice()).expect("Invalid OID")
    }
}
