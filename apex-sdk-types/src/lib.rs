//! # Apex SDK Types
//!
//! Common types and data structures used across the Apex SDK.
//!
//! This crate provides fundamental types for representing blockchain entities
//! across different chain types (Substrate, EVM, Hybrid).
//!
//! ## Core Types
//!
//! - **Chain**: Enumeration of supported blockchain networks
//! - **ChainType**: Classification of chains (Substrate, EVM, Hybrid)
//! - **Address**: Generic address type supporting multiple formats
//! - **TransactionStatus**: Unified transaction status representation
//! - **CrossChainTransaction**: Cross-chain transaction information
//!
//! ## Example
//!
//! ```rust
//! use apex_sdk_types::{Chain, ChainType, Address};
//!
//! // Create addresses for different chains
//! let eth_addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
//! let dot_addr = Address::substrate("15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5");
//!
//! // Check chain types
//! assert_eq!(Chain::Ethereum.chain_type(), ChainType::Evm);
//! assert_eq!(Chain::Polkadot.chain_type(), ChainType::Substrate);
//! assert_eq!(Chain::Moonbeam.chain_type(), ChainType::Hybrid);
//! ```

use serde::{Deserialize, Serialize};

/// Blockchain types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType {
    /// Substrate-based chain
    Substrate,
    /// EVM-based chain
    Evm,
    /// Hybrid chain (both Substrate and EVM)
    Hybrid,
}

/// Supported blockchain networks
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chain {
    // Substrate Relay Chains
    /// Polkadot relay chain
    Polkadot,
    /// Kusama relay chain
    Kusama,

    // Substrate Parachains
    /// Moonbeam (Polkadot parachain with EVM)
    Moonbeam,
    /// Astar (Polkadot parachain with EVM)
    Astar,
    /// Acala DeFi Hub
    Acala,
    /// Phala Privacy Cloud
    Phala,
    /// Bifrost Liquid Staking
    Bifrost,
    /// Westend testnet (legacy)
    Westend,
    /// Paseo testnet (default Polkadot testnet)
    Paseo,

    // EVM Layer 1
    /// Ethereum mainnet
    Ethereum,
    /// Binance Smart Chain
    BinanceSmartChain,
    /// Polygon
    Polygon,
    /// Avalanche C-Chain
    Avalanche,

    // EVM Layer 2
    /// Arbitrum One
    Arbitrum,
    /// Optimism
    Optimism,
    /// zkSync Era
    ZkSync,
    /// Base (Coinbase L2)
    Base,
}

impl Chain {
    /// Get the chain type
    pub fn chain_type(&self) -> ChainType {
        match self {
            // Pure Substrate chains
            Chain::Polkadot
            | Chain::Kusama
            | Chain::Acala
            | Chain::Phala
            | Chain::Bifrost
            | Chain::Westend
            | Chain::Paseo => ChainType::Substrate,

            // Pure EVM chains
            Chain::Ethereum
            | Chain::BinanceSmartChain
            | Chain::Polygon
            | Chain::Avalanche
            | Chain::Arbitrum
            | Chain::Optimism
            | Chain::ZkSync
            | Chain::Base => ChainType::Evm,

            // Hybrid chains (Substrate + EVM)
            Chain::Moonbeam | Chain::Astar => ChainType::Hybrid,
        }
    }

    /// Get the chain name
    pub fn name(&self) -> &str {
        match self {
            // Substrate
            Chain::Polkadot => "Polkadot",
            Chain::Kusama => "Kusama",
            Chain::Acala => "Acala",
            Chain::Phala => "Phala",
            Chain::Bifrost => "Bifrost",
            Chain::Westend => "Westend",
            Chain::Paseo => "Paseo",

            // EVM L1
            Chain::Ethereum => "Ethereum",
            Chain::BinanceSmartChain => "Binance Smart Chain",
            Chain::Polygon => "Polygon",
            Chain::Avalanche => "Avalanche",

            // EVM L2
            Chain::Arbitrum => "Arbitrum",
            Chain::Optimism => "Optimism",
            Chain::ZkSync => "zkSync",
            Chain::Base => "Base",

            // Hybrid
            Chain::Moonbeam => "Moonbeam",
            Chain::Astar => "Astar",
        }
    }

    /// Get default RPC endpoint for the chain
    pub fn default_endpoint(&self) -> &str {
        match self {
            // Substrate
            Chain::Polkadot => "wss://polkadot.api.onfinality.io/public-ws",
            Chain::Kusama => "wss://kusama.api.onfinality.io/public-ws",
            Chain::Acala => "wss://acala.api.onfinality.io/public-ws",
            Chain::Phala => "wss://phala.api.onfinality.io/public-ws",
            Chain::Bifrost => "wss://bifrost-polkadot.api.onfinality.io/public-ws",
            Chain::Westend => "wss://westend-rpc.polkadot.io",
            Chain::Paseo => "wss://paseo-rpc.polkadot.io",

            // EVM L1
            Chain::Ethereum => "https://eth.llamarpc.com",
            Chain::BinanceSmartChain => "https://bsc.publicnode.com",
            Chain::Polygon => "https://polygon-rpc.com",
            Chain::Avalanche => "https://api.avax.network/ext/bc/C/rpc",

            // EVM L2
            Chain::Arbitrum => "https://arb1.arbitrum.io/rpc",
            Chain::Optimism => "https://mainnet.optimism.io",
            Chain::ZkSync => "https://mainnet.era.zksync.io",
            Chain::Base => "https://mainnet.base.org",

            // Hybrid
            Chain::Moonbeam => "wss://moonbeam.api.onfinality.io/public-ws",
            Chain::Astar => "wss://astar.api.onfinality.io/public-ws",
        }
    }

    /// Get multiple RPC endpoints for reliability and failover
    pub fn rpc_endpoints(&self) -> Vec<&str> {
        match self {
            // Substrate
            Chain::Polkadot => vec![
                "wss://polkadot.api.onfinality.io/public-ws",
                "wss://rpc.ibp.network/polkadot",
                "wss://polkadot.dotters.network",
            ],
            Chain::Kusama => vec![
                "wss://kusama.api.onfinality.io/public-ws",
                "wss://rpc.ibp.network/kusama",
                "wss://kusama.dotters.network",
            ],
            Chain::Westend => vec![
                "wss://westend-rpc.polkadot.io",
                "wss://rpc.ibp.network/westend",
                "wss://westend.dotters.network",
            ],
            Chain::Paseo => vec![
                "wss://paseo-rpc.polkadot.io",
                "wss://rpc.ibp.network/paseo",
                "wss://paseo.dotters.network",
            ],
            // For other chains, return the single default endpoint
            _ => vec![self.default_endpoint()],
        }
    }

    /// Check if chain is a Layer 2 solution
    pub fn is_layer2(&self) -> bool {
        matches!(
            self,
            Chain::Arbitrum | Chain::Optimism | Chain::ZkSync | Chain::Base
        )
    }

    /// Check if chain supports smart contracts
    pub fn supports_smart_contracts(&self) -> bool {
        match self.chain_type() {
            ChainType::Evm => true,
            ChainType::Hybrid => true,
            ChainType::Substrate => matches!(
                self,
                Chain::Acala | Chain::Phala | Chain::Moonbeam | Chain::Astar
            ),
        }
    }
}

/// Generic address type for different chains
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Address {
    /// Substrate SS58 address
    Substrate(String),
    /// EVM hex address (0x...)
    Evm(String),
}

impl Address {
    /// Create a Substrate address
    pub fn substrate(addr: impl Into<String>) -> Self {
        Address::Substrate(addr.into())
    }

    /// Create an EVM address
    pub fn evm(addr: impl Into<String>) -> Self {
        Address::Evm(addr.into())
    }

    /// Get the address as a string
    pub fn as_str(&self) -> &str {
        match self {
            Address::Substrate(s) | Address::Evm(s) => s,
        }
    }
}

/// Transaction status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Transaction is pending.
    ///
    /// The transaction has been created but has not yet been broadcasted to the network.
    /// This status typically indicates that the transaction is awaiting submission or signing.
    Pending,
    /// Transaction is in memory pool (mempool).
    ///
    /// The transaction has been broadcasted to the network and is waiting to be included in a block.
    /// This status indicates that the transaction is known to the network but not yet confirmed.
    InMempool,
    /// Transaction is confirmed
    Confirmed {
        /// Block hash
        block_hash: String,
        /// Block number where transaction was included
        block_number: Option<u64>,
    },
    /// Transaction is finalized (for Substrate chains)
    Finalized {
        /// Block hash
        block_hash: String,
        /// Block number
        block_number: u64,
    },
    /// Transaction failed
    Failed {
        /// Error message
        error: String,
    },
    /// Transaction status unknown
    Unknown,
}

/// Represents a blockchain event emitted by a smart contract or runtime.
///
/// The `Event` struct captures details about an event, including its name, associated data,
/// the block and transaction in which it occurred, and its index within the block.
///
/// # Fields
/// - `name`: The name of the event (e.g., `"Transfer"`, `"Approval"`).
/// - `data`: The event payload as a JSON value. This typically contains event parameters.
/// - `block_number`: The block number in which the event was emitted, if available.
/// - `tx_hash`: The transaction hash associated with the event, if available.
/// - `index`: The index of the event within the block, if available.
///
/// # Example
/// ```
/// use apex_sdk_types::Event;
/// use serde_json::json;
///
/// let event = Event {
///     name: "Transfer".to_string(),
///     data: json!({
///         "from": "0x123...",
///         "to": "0x456...",
///         "value": 1000
///     }),
///     block_number: Some(123456),
///     tx_hash: Some("0xabc...".to_string()),
///     index: Some(0),
/// };
/// assert_eq!(event.name, "Transfer");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// The name of the event (e.g., "Transfer", "Approval").
    pub name: String,
    /// The event payload as a JSON value, typically containing event parameters.
    pub data: serde_json::Value,
    /// The block number in which the event was emitted, if available.
    pub block_number: Option<u64>,
    /// The transaction hash associated with the event, if available.
    pub tx_hash: Option<String>,
    /// The index of the event within the block, if available.
    pub index: Option<u32>,
}

/// Filter criteria for subscribing to blockchain events.
///
/// This struct allows you to specify which events to receive by name, contract address,
/// and block range. All fields are optional; if a field is `None`, it will not be used
/// as a filter criterion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    /// List of event names to filter for.
    ///
    /// If specified, only events with names matching one of the strings in this list
    /// will be included. If `None`, all event names are included.
    pub event_names: Option<Vec<String>>,
    /// List of contract addresses to filter for.
    ///
    /// If specified, only events emitted by contracts with addresses in this list
    /// will be included. If `None`, events from all addresses are included.
    pub addresses: Option<Vec<Address>>,
    /// The starting block number (inclusive) for filtering events.
    ///
    /// If specified, only events from blocks with number greater than or equal to this
    /// value will be included. If `None`, events from all blocks are included.
    pub from_block: Option<u64>,
    /// The ending block number (inclusive) for filtering events.
    ///
    /// If specified, only events from blocks with number less than or equal to this
    /// value will be included. If `None`, events up to the latest block are included.
    pub to_block: Option<u64>,
}

/// Cross-chain transaction info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainTransaction {
    /// Transaction ID
    pub id: String,
    /// Source chain
    pub source_chain: Chain,
    /// Destination chain
    pub destination_chain: Chain,
    /// Source transaction hash
    pub source_tx_hash: Option<String>,
    /// Destination transaction hash
    pub destination_tx_hash: Option<String>,
    /// Transaction status
    pub status: TransactionStatus,
    /// Timestamp
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_type() {
        assert_eq!(Chain::Polkadot.chain_type(), ChainType::Substrate);
        assert_eq!(Chain::Ethereum.chain_type(), ChainType::Evm);
        assert_eq!(Chain::Moonbeam.chain_type(), ChainType::Hybrid);
    }

    #[test]
    fn test_address_creation() {
        let sub_addr = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
        assert!(matches!(sub_addr, Address::Substrate(_)));

        let evm_addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        assert!(matches!(evm_addr, Address::Evm(_)));
    }
}
