//! Error types for SNMP operations.

use thiserror::Error;

/// Errors that can occur during SNMP sessions and operations.
#[derive(Error, Debug)]
pub enum SnmpError {
    /// Failure establishing or initialising an SNMP session.
    #[error("Failed to create SNMP session: {0}")]
    SessionError(String),
    /// An SNMP get or set operation returned an error.
    #[error("SNMP operation failed: {0}")]
    OperationError(String),
}
