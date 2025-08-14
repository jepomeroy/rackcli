use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnmpError {
    #[error("Failed to create SNMP session: {0}")]
    SessionError(String),

    #[error("SNMP operation failed: {0}")]
    OperationError(String),
}
