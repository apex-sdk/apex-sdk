//! Error types for the Apex SDK.

use thiserror::Error;

/// Result type alias for Apex SDK operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the Apex SDK.
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Invalid address error
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// Unsupported chain error
    #[error("Unsupported chain: {0}")]
    UnsupportedChain(String),

    /// Generic error
    #[error("Error: {0}")]
    Other(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    #[test]
    fn test_config_error_display() {
        let error = Error::Config("test config error".to_string());
        assert_eq!(error.to_string(), "Configuration error: test config error");
    }

    #[test]
    fn test_connection_error_display() {
        let error = Error::Connection("test connection error".to_string());
        assert_eq!(error.to_string(), "Connection error: test connection error");
    }

    #[test]
    fn test_transaction_error_display() {
        let error = Error::Transaction("test transaction error".to_string());
        assert_eq!(
            error.to_string(),
            "Transaction error: test transaction error"
        );
    }

    #[test]
    fn test_serialization_error_display() {
        let error = Error::Serialization("test serialization error".to_string());
        assert_eq!(
            error.to_string(),
            "Serialization error: test serialization error"
        );
    }

    #[test]
    fn test_invalid_address_error_display() {
        let error = Error::InvalidAddress("test invalid address".to_string());
        assert_eq!(error.to_string(), "Invalid address: test invalid address");
    }

    #[test]
    fn test_unsupported_chain_error_display() {
        let error = Error::UnsupportedChain("test unsupported chain".to_string());
        assert_eq!(
            error.to_string(),
            "Unsupported chain: test unsupported chain"
        );
    }

    #[test]
    fn test_other_error_display() {
        let error = Error::Other("test other error".to_string());
        assert_eq!(error.to_string(), "Error: test other error");
    }

    #[test]
    fn test_from_anyhow_error() {
        let anyhow_err = anyhow::anyhow!("test anyhow error");
        let error: Error = anyhow_err.into();
        assert!(matches!(error, Error::Other(_)));
        assert_eq!(error.to_string(), "Error: test anyhow error");
    }
}
