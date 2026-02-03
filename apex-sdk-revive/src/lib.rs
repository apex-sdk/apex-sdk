//! # Apex SDK Revive Adapter
//!
//! Native Rust adapter for `pallet-revive` (PolkaVM/Solidity) on Polkadot System Chains.

use apex_sdk_core::SdkError;
use thiserror::Error;

pub mod adapter;
pub mod contract;

pub use adapter::ReviveAdapter;
pub use contract::{Contract, ContractManager};

/// Revive adapter error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Contract error: {0}")]
    Contract(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Subxt error: {0}")]
    Subxt(#[from] subxt::Error),
}

impl From<Error> for SdkError {
    fn from(err: Error) -> Self {
        match err {
            Error::Connection(msg) => SdkError::NetworkError(msg),
            Error::Transaction(msg) => SdkError::TransactionError(msg),
            Error::Contract(msg) => SdkError::TransactionError(msg),
            Error::Storage(msg) => SdkError::ProviderError(msg),
            Error::Other(msg) => SdkError::ProviderError(msg),
            Error::Subxt(e) => SdkError::ProviderError(e.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
