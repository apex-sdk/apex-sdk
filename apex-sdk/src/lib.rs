//! # Apex SDK
//!
//! The industry's first unified Rust SDK for Substrate & EVM blockchain development.
//!
//! ## Features
//!
//! - **Unified Interface**: Single API for both Substrate and EVM blockchains
//! - **Compile-Time Type Safety**: Catch errors before deployment
//! - **Native Performance**: Rust-based implementation
//! - **Cross-Chain Ready**: Built-in cross-chain communication support
//!
//! ## Example
//!
//! ```rust,no_run
//! use apex_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let sdk = ApexSDK::builder()
//!         .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
//!         .with_revive_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
//!         .build()
//!         .await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod advanced;
pub mod builder;
pub mod error;
pub mod error_recovery;
pub mod performance;
pub mod sdk;
pub mod transaction;

pub use apex_sdk_core as core;
pub use apex_sdk_revive as revive;
pub use apex_sdk_substrate as substrate;
pub use apex_sdk_types as types;

pub use advanced::{
    BlockInfo, BlockSubscription, EventSubscription, ParallelExecutor, TransactionBatch,
};
pub use builder::ApexSDKBuilder;
pub use error::{Error, Result};
pub use error_recovery::{with_retry, CircuitBreaker, RetryConfig};
pub use performance::{
    batch_execute, parallel_execute, AsyncMemo, BatchConfig, ConnectionPool, RateLimiter,
};
pub use sdk::{ApexSDK, ConfirmationStrategy, SdkConfig};
pub use transaction::{Transaction, TransactionBuilder, TransactionResult};

/// Prelude module for common imports
pub mod prelude {
    pub use crate::{
        builder::ApexSDKBuilder,
        error::{Error, Result},
        sdk::{ApexSDK, ConfirmationStrategy, SdkConfig},
        transaction::{Transaction, TransactionBuilder, TransactionResult},
        types::{Address, Chain, ChainType},
    };

    #[cfg(feature = "substrate")]
    pub use crate::substrate::{SubstrateAdapter, Wallet as SubstrateWallet};

    #[cfg(feature = "revive")]
    pub use crate::revive::ReviveAdapter;
}
