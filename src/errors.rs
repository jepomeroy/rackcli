use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnmpError {
    #[error("Failed to create SNMP session: {0}")]
    SessionError(String),

    #[error("SNMP operation failed: {0}")]
    OperationError(String),

    #[error("Invalid SNMP value: {0}")]
    InvalidValue(String),

    #[error("SNMP OID not found: {0}")]
    OidNotFound(String),
}
