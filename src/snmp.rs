use std::time::Duration;

use crate::errors::SnmpError;
use crate::snmpv2::SnmpV2Client;
use crate::snmpv3::SnmpV3Client;
use crate::switch::{SNMPVersion, Switch, SwitchResult};
use snmp2::Oid;
use tokio::task::JoinSet;

pub trait SnmpClient {
    async fn get(self, oid: Oid, port: u64) -> Result<SwitchResult, SnmpError>;
    async fn set(self, oid: Oid, value: i64, port: u64) -> Result<SwitchResult, SnmpError>;
}

pub struct Snmp {}

impl Snmp {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get(&self, switch: &Switch) -> Result<Vec<SwitchResult>, SnmpError> {
        let mut req_set = JoinSet::new();

        match switch.get_version() {
            SNMPVersion::V2 => {
                let mut switch_results = Vec::new();
                for port in switch.get_ports().iter() {
                    let oid = Snmp::make_oid(switch.get_oid(), *port);
                    let v2 = SnmpV2Client::new(
                        switch.get_socket_addr(),
                        switch.get_community().as_bytes(),
                        Some(Duration::from_secs(5)),
                    )?;

                    req_set.spawn(v2.get(oid, *port));
                }

                // Print the results
                while let Some(result_result) = req_set.join_next().await {
                    match result_result {
                        Ok(Ok(switch_result)) => switch_results.push(switch_result),
                        Ok(Err(e)) => println!("Error: {}", e),
                        Err(e) => println!("Task error: {}", e),
                    }
                }

                Ok(switch_results)
            }
            SNMPVersion::V3 => {
                let mut switch_results = Vec::new();
                let auth_password = switch.get_or_prompt_auth_password();
                let privacy_password = switch.get_or_prompt_privacy_password();
                for port in switch.get_ports().iter() {
                    let oid = Snmp::make_oid(switch.get_oid(), *port);
                    let v3 = SnmpV3Client::new(
                        switch.get_socket_addr(),
                        switch.get_username(),
                        &auth_password,
                        switch.get_auth_protocol(),
                        switch.get_privacy_protocol(),
                        &privacy_password,
                        Some(Duration::from_secs(5)),
                    )?;

                    req_set.spawn(v3.get(oid, *port));
                }

                // Print the results
                while let Some(result_result) = req_set.join_next().await {
                    match result_result {
                        Ok(Ok(switch_result)) => switch_results.push(switch_result),
                        Ok(Err(e)) => println!("Error: {}", e),
                        Err(e) => println!("Task error: {}", e),
                    }
                }

                Ok(switch_results)
            }
        }
    }

    pub async fn set(&self, switch: &Switch, value: i64) -> Result<Vec<SwitchResult>, SnmpError> {
        let mut req_set = JoinSet::new();

        match switch.get_version() {
            SNMPVersion::V2 => {
                let mut switch_results = Vec::new();
                for port in switch.get_ports().iter() {
                    let oid = Snmp::make_oid(switch.get_oid(), *port);
                    let v2 = SnmpV2Client::new(
                        switch.get_socket_addr(),
                        switch.get_community().as_bytes(),
                        Some(Duration::from_secs(5)),
                    )?;

                    req_set.spawn(v2.set(oid, value, *port));
                }

                // Print the results
                while let Some(result_result) = req_set.join_next().await {
                    match result_result {
                        Ok(Ok(switch_result)) => switch_results.push(switch_result),
                        Ok(Err(e)) => println!("Error: {}", e),
                        Err(e) => println!("Task error: {}", e),
                    }
                }

                Ok(switch_results)
            }
            SNMPVersion::V3 => {
                let mut switch_results = Vec::new();
                let auth_password = switch.get_or_prompt_auth_password();
                let privacy_password = switch.get_or_prompt_privacy_password();
                for port in switch.get_ports().iter() {
                    let oid = Snmp::make_oid(switch.get_oid(), *port);
                    let v3 = SnmpV3Client::new(
                        switch.get_socket_addr(),
                        switch.get_username(),
                        &auth_password,
                        switch.get_auth_protocol(),
                        switch.get_privacy_protocol(),
                        &privacy_password,
                        Some(Duration::from_secs(5)),
                    )?;

                    req_set.spawn(v3.set(oid, value, *port));
                }

                // Print the results
                while let Some(result_result) = req_set.join_next().await {
                    match result_result {
                        Ok(Ok(switch_result)) => switch_results.push(switch_result),
                        Ok(Err(e)) => println!("Error: {}", e),
                        Err(e) => println!("Task error: {}", e),
                    }
                }

                Ok(switch_results)
            }
        }
    }

    fn make_oid(oid_vec: Vec<u64>, port: u64) -> Oid<'static> {
        let mut new_vec = oid_vec.to_vec();
        new_vec.push(port);

        Oid::from(new_vec.as_slice()).expect("Invalid OID")
    }
}
