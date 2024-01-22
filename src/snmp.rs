// use clent::CLient;
use crate::switch::{SNMPAuth, SNMPEncryption, SNMPVersion, Switch};

use snmpclient::client::{AuthProtocol, PrivacyProtocol, SNMPv3Client};
use snmpclient::params::Params;

const SNMP_PORT_NUM: u32 = 161;

fn get_host(host: String) -> String {
    if host.find(':').is_none() {
        format!("{}:{}", host, SNMP_PORT_NUM)
    } else {
        host
    }
}

pub fn snmp_get(switch: &Switch) -> Result<String, String> {
    match switch.get_version() {
        SNMPVersion::V2 => {
            let params = Params::new_params_v2c(
                switch.get_host(),
                switch.get_username(),
                switch.get_community(),
            );
        }
        SNMPVersion::V3 => {
            let auth_protocol = match switch.get_auth_protocol() {
                SNMPAuth::MD5 => Some(AuthProtocol::MD5),
                SNMPAuth::SHA => Some(AuthProtocol::SHA),
                _ => None,
            };

            let privacy_protocol = match switch.get_privacy_protocol() {
                SNMPEncryption::DES => Some(PrivacyProtocol::DES),
                SNMPEncryption::AES => Some(PrivacyProtocol::AES),
                _ => None,
            };

            let params = Params::new_params_v3(
                switch.get_host(),
                switch.get_username(),
                auth_protocol,
                switch.get_auth_password(),
                privacy_protocol,
                switch.get_privacy_password(),
            );
            let client = SNMPv3Client::new(params);
        }
    }

    Ok("bar".to_string())
}

// pub fn snmp_set() -> Result<String, Error> {
//     request::snmp_get(PduType::SetRequest, oids, &mut client, &mut session)?;

//     Ok("foo")
// }

// Tests
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_host_with_port() {
        let result = get_host("192.168.1.1:161".to_string());

        assert_eq!(result, "192.168.1.1:161".to_string());
    }

    #[test]
    fn test_host_with_out_port() {
        let result = get_host("192.168.1.1".to_string());

        assert_eq!(result, "192.168.1.1:161".to_string());
    }
}
