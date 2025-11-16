//! Error types for Apex SDK

use thiserror::Error;

/// Result type alias for Apex SDK operations
pub type Result<T> = std::result::Result<T, Error>;

/// Apex SDK error types
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration error
    #[error("Configuration error: {0}\n\n💡 Tip: {1}")]
    Config(String, String),

    /// Connection error
    #[error("Connection error: {0}\n\n💡 Tip: {1}")]
    Connection(String, String),

    /// Transaction error
    #[error("Transaction error: {0}\n\n💡 Tip: {1}")]
    Transaction(String, String),

    /// Chain not supported
    #[error("Chain not supported: {0}\n\n💡 Supported chains: Polkadot, Kusama, Westend (Substrate) | Ethereum, BSC, Polygon (EVM)\n💡 Use `apex chain list` to see all supported chains")]
    UnsupportedChain(String),

    /// Invalid address format
    #[error("Invalid address format: {0}\n\n💡 Expected formats:\n  • Substrate (SS58): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\n  • Ethereum (Hex): 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb")]
    InvalidAddress(String),

    /// Substrate adapter error
    #[error("Substrate adapter error: {0}")]
    Substrate(#[from] apex_sdk_substrate::Error),

    /// EVM adapter error
    #[error("EVM adapter error: {0}")]
    Evm(#[from] apex_sdk_evm::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a configuration error with a helpful tip
    pub fn config(msg: impl Into<String>) -> Self {
        let msg_str = msg.into();
        let tip = if msg_str.contains("adapter") || msg_str.contains("configured") {
            "Use .with_substrate() or .with_evm() when building the SDK:\n    \
             ApexSDK::builder().with_substrate(Chain::Polkadot, \"wss://rpc.polkadot.io\").build()?"
        } else {
            "Check your configuration and ensure all required fields are set"
        };
        Error::Config(msg_str, tip.to_string())
    }

    /// Create a connection error with a helpful tip
    pub fn connection(msg: impl Into<String>) -> Self {
        let msg_str = msg.into();
        let tip = if msg_str.contains("timeout") || msg_str.contains("timed out") {
            "The endpoint may be slow or unreachable. Try:\n    \
             1. Use a different RPC endpoint\n    \
             2. Increase timeout with .with_timeout(Duration::from_secs(30))\n    \
             3. Check your internet connection"
        } else if msg_str.contains("refused") || msg_str.contains("failed to connect") {
            "Cannot reach the endpoint. Verify:\n    \
             1. The endpoint URL is correct\n    \
             2. Your firewall allows outbound connections\n    \
             3. The node is online and accessible"
        } else {
            "Check the endpoint URL and network connection"
        };
        Error::Connection(msg_str, tip.to_string())
    }

    /// Create a transaction error with a helpful tip
    pub fn transaction(msg: impl Into<String>) -> Self {
        let msg_str = msg.into();
        let tip = if msg_str.contains("insufficient") || msg_str.contains("balance") {
            "Your account doesn't have enough balance. Ensure:\n    \
             1. The account has sufficient funds for transaction + fees\n    \
             2. You're using the correct account\n    \
             3. The chain is the right one for your tokens"
        } else if msg_str.contains("nonce") {
            "Transaction nonce error. Try:\n    \
             1. Wait for pending transactions to complete\n    \
             2. Use .with_nonce() to specify nonce manually\n    \
             3. Check for stuck transactions"
        } else {
            "Review the transaction parameters and account state"
        };
        Error::Transaction(msg_str, tip.to_string())
    }
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
    fn test_config_error_display() {
        let error = Error::config("test config error");
        assert!(error
            .to_string()
            .contains("Configuration error: test config error"));
        assert!(error.to_string().contains("💡 Tip:"));
    }

    #[test]
    fn test_connection_error_display() {
        let error = Error::connection("failed to connect");
        assert!(error
            .to_string()
            .contains("Connection error: failed to connect"));
        assert!(error.to_string().contains("💡 Tip:"));
    }

    #[test]
    fn test_transaction_error_display() {
        let error = Error::transaction("invalid transaction");
        assert!(error
            .to_string()
            .contains("Transaction error: invalid transaction"));
        assert!(error.to_string().contains("💡 Tip:"));
    }

    #[test]
    fn test_unsupported_chain_error_display() {
        let error = Error::UnsupportedChain("Unknown".to_string());
        assert!(error.to_string().contains("Chain not supported: Unknown"));
        assert!(error.to_string().contains("💡 Supported chains:"));
    }

    #[test]
    fn test_invalid_address_error_display() {
        let error = Error::InvalidAddress("0xinvalid".to_string());
        assert!(error
            .to_string()
            .contains("Invalid address format: 0xinvalid"));
        assert!(error.to_string().contains("💡 Expected formats:"));
    }

    #[test]
    fn test_serialization_error_display() {
        let error = Error::Serialization("JSON parse error".to_string());
        assert_eq!(error.to_string(), "Serialization error: JSON parse error");
    }

    #[test]
    fn test_other_error_display() {
        let error = Error::Other("generic error".to_string());
        assert_eq!(error.to_string(), "generic error");
    }

    #[test]
    fn test_from_anyhow_error() {
        let anyhow_err = anyhow::anyhow!("test error");
        let error: Error = anyhow_err.into();
        assert!(matches!(error, Error::Other(_)));
        assert_eq!(error.to_string(), "test error");
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<Error>();
        assert_sync::<Error>();
    }
}
