//! Transaction building and execution

use crate::error::{Error, Result};
use apex_sdk_types::{Address, Chain, TransactionStatus};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// Transaction builder for creating cross-chain transactions
pub struct TransactionBuilder {
    from: Option<Address>,
    to: Option<Address>,
    amount: Option<u128>,
    source_chain: Option<Chain>,
    destination_chain: Option<Chain>,
    data: Option<Vec<u8>>,
    gas_limit: Option<u64>,
    nonce: Option<u64>,
}

impl TransactionBuilder {
    /// Create a new transaction builder
    pub fn new() -> Self {
        Self {
            from: None,
            to: None,
            amount: None,
            source_chain: None,
            destination_chain: None,
            data: None,
            gas_limit: None,
            nonce: None,
        }
    }

    /// Set the sender address (Substrate)
    ///
    /// **Security Note**: This method does not validate the address format.
    /// Use `from_substrate_account_checked()` for validated input, or call `.build_validated()`
    /// instead of `.build()` to validate all addresses before building the transaction.
    pub fn from_substrate_account(mut self, address: impl Into<String>) -> Self {
        self.from = Some(Address::substrate(address));
        self
    }

    /// Set the sender address (Substrate) with immediate validation
    ///
    /// This validates the SS58 format and checksum immediately.
    /// Returns an error if the address is invalid.
    pub fn from_substrate_account_checked(mut self, address: impl Into<String>) -> Result<Self> {
        self.from = Some(
            Address::substrate_checked(address)
                .map_err(|e| Error::InvalidAddress(e.to_string()))?,
        );
        Ok(self)
    }

    /// Set the sender address (EVM)
    ///
    /// **Security Note**: This method does not validate the address format.
    /// Use `from_evm_address_checked()` for validated input, or call `.build_validated()`
    /// instead of `.build()` to validate all addresses before building the transaction.
    pub fn from_evm_address(mut self, address: impl Into<String>) -> Self {
        self.from = Some(Address::evm(address));
        self
    }

    /// Set the sender address (EVM) with immediate validation
    ///
    /// This validates the EVM format and EIP-55 checksum immediately.
    /// Returns an error if the address is invalid.
    pub fn from_evm_address_checked(mut self, address: impl Into<String>) -> Result<Self> {
        self.from =
            Some(Address::evm_checked(address).map_err(|e| Error::InvalidAddress(e.to_string()))?);
        Ok(self)
    }

    /// Set the sender address
    pub fn from(mut self, address: Address) -> Self {
        self.from = Some(address);
        self
    }

    /// Set the recipient address (EVM)
    ///
    /// **Security Note**: This method does not validate the address format.
    /// Use `to_evm_address_checked()` for validated input, or call `.build_validated()`
    /// instead of `.build()` to validate all addresses before building the transaction.
    pub fn to_evm_address(mut self, address: impl Into<String>) -> Self {
        self.to = Some(Address::evm(address));
        self
    }

    /// Set the recipient address (EVM) with immediate validation
    ///
    /// This validates the EVM format and EIP-55 checksum immediately.
    /// Returns an error if the address is invalid.
    pub fn to_evm_address_checked(mut self, address: impl Into<String>) -> Result<Self> {
        self.to =
            Some(Address::evm_checked(address).map_err(|e| Error::InvalidAddress(e.to_string()))?);
        Ok(self)
    }

    /// Set the recipient address (Substrate)
    ///
    /// **Security Note**: This method does not validate the address format.
    /// Use `to_substrate_account_checked()` for validated input, or call `.build_validated()`
    /// instead of `.build()` to validate all addresses before building the transaction.
    pub fn to_substrate_account(mut self, address: impl Into<String>) -> Self {
        self.to = Some(Address::substrate(address));
        self
    }

    /// Set the recipient address (Substrate) with immediate validation
    ///
    /// This validates the SS58 format and checksum immediately.
    /// Returns an error if the address is invalid.
    pub fn to_substrate_account_checked(mut self, address: impl Into<String>) -> Result<Self> {
        self.to = Some(
            Address::substrate_checked(address)
                .map_err(|e| Error::InvalidAddress(e.to_string()))?,
        );
        Ok(self)
    }

    /// Set the recipient address
    pub fn to(mut self, address: Address) -> Self {
        self.to = Some(address);
        self
    }

    /// Set the transfer amount
    pub fn amount(mut self, amount: u128) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the source chain
    pub fn on_chain(mut self, chain: Chain) -> Self {
        self.source_chain = Some(chain);
        self
    }

    /// Set transaction data/payload
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Set gas limit
    pub fn with_gas_limit(mut self, limit: u64) -> Self {
        self.gas_limit = Some(limit);
        self
    }

    /// Set transaction nonce (for uniqueness and replay protection)
    pub fn with_nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    /// Build the transaction
    pub fn build(self) -> Result<Transaction> {
        let from = self
            .from
            .ok_or_else(|| Error::transaction("Sender address required"))?;
        let to = self
            .to
            .ok_or_else(|| Error::transaction("Recipient address required"))?;
        let amount = self
            .amount
            .ok_or_else(|| Error::transaction("Amount required"))?;

        // Determine source and destination chains based on addresses if not specified
        let source_chain = self.source_chain.unwrap_or(match &from {
            Address::Substrate(_) => Chain::Polkadot,
            Address::Evm(_) => Chain::Ethereum,
        });

        let destination_chain = self.destination_chain.unwrap_or(match &to {
            Address::Substrate(_) => Chain::Polkadot,
            Address::Evm(_) => Chain::Ethereum,
        });

        Ok(Transaction {
            from,
            to,
            amount,
            source_chain,
            destination_chain,
            data: self.data,
            gas_limit: self.gas_limit,
            nonce: self.nonce,
        })
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a blockchain transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Sender address
    pub from: Address,
    /// Recipient address
    pub to: Address,
    /// Amount to transfer
    pub amount: u128,
    /// Source blockchain
    pub source_chain: Chain,
    /// Destination blockchain
    pub destination_chain: Chain,
    /// Transaction data/payload
    pub data: Option<Vec<u8>>,
    /// Gas limit
    pub gas_limit: Option<u64>,
    /// Nonce for transaction uniqueness (prevents replay attacks)
    #[serde(default)]
    pub nonce: Option<u64>,
}

impl Transaction {
    /// Check if this is a cross-chain transaction
    pub fn is_cross_chain(&self) -> bool {
        self.source_chain != self.destination_chain
    }

    /// Get transaction hash using Keccak256
    ///
    /// This implementation ensures deterministic hashing by:
    /// 1. Using canonical name representation instead of Debug formatting
    /// 2. Explicitly marking presence/absence of optional fields
    /// 3. Including all fields in a fixed order
    ///
    /// # Determinism Guarantee
    ///
    /// The same transaction parameters will always produce the same hash,
    /// regardless of the order of construction or serialization format changes.
    pub fn hash(&self) -> String {
        let mut hasher = Keccak256::new();

        // Hash transaction data in deterministic order with explicit field markers

        // Field 1: from address
        hasher.update(b"from:");
        hasher.update(self.from.as_str().as_bytes());

        // Field 2: to address
        hasher.update(b"to:");
        hasher.update(self.to.as_str().as_bytes());

        // Field 3: amount (always present)
        hasher.update(b"amount:");
        hasher.update(self.amount.to_le_bytes());

        // Field 4: source chain (use canonical name)
        hasher.update(b"source_chain:");
        hasher.update(self.source_chain.name().as_bytes());

        // Field 5: destination chain (use canonical name)
        hasher.update(b"destination_chain:");
        hasher.update(self.destination_chain.name().as_bytes());

        // Field 6: data (optional - mark presence)
        hasher.update(b"data:");
        if let Some(ref data) = self.data {
            hasher.update(b"some:");
            hasher.update((data.len() as u64).to_le_bytes());
            hasher.update(data);
        } else {
            hasher.update(b"none");
        }

        // Field 7: gas_limit (optional - mark presence)
        hasher.update(b"gas_limit:");
        if let Some(gas_limit) = self.gas_limit {
            hasher.update(b"some:");
            hasher.update(gas_limit.to_le_bytes());
        } else {
            hasher.update(b"none");
        }

        // Field 8: nonce (optional - mark presence)
        hasher.update(b"nonce:");
        if let Some(nonce) = self.nonce {
            hasher.update(b"some:");
            hasher.update(nonce.to_le_bytes());
        } else {
            hasher.update(b"none");
        }

        let result = hasher.finalize();
        format!("0x{}", hex::encode(result))
    }
}

/// Transaction execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    /// Transaction hash on source chain
    pub source_tx_hash: String,
    /// Transaction hash on destination chain (for cross-chain)
    pub destination_tx_hash: Option<String>,
    /// Transaction status
    pub status: TransactionStatus,
    /// Block number where transaction was included
    pub block_number: Option<u64>,
    /// Gas used
    pub gas_used: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_builder_new() {
        let builder = TransactionBuilder::new();
        assert!(builder.from.is_none());
        assert!(builder.to.is_none());
        assert!(builder.amount.is_none());
    }

    #[test]
    fn test_transaction_builder_default() {
        let builder = TransactionBuilder::default();
        assert!(builder.from.is_none());
        assert!(builder.to.is_none());
    }

    #[test]
    fn test_transaction_builder_evm_to_evm() {
        let tx = TransactionBuilder::new()
            .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .to_evm_address("0x1234567890123456789012345678901234567890")
            .amount(1000)
            .build();

        assert!(tx.is_ok());
        let tx = tx.unwrap();
        assert_eq!(tx.amount, 1000);
        assert!(!tx.is_cross_chain());
        assert_eq!(tx.source_chain, Chain::Ethereum);
        assert_eq!(tx.destination_chain, Chain::Ethereum);
    }

    #[test]
    fn test_transaction_builder_substrate_to_evm() {
        let tx = TransactionBuilder::new()
            .from_substrate_account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
            .to_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .amount(500)
            .build();

        assert!(tx.is_ok());
        let tx = tx.unwrap();
        assert!(tx.is_cross_chain());
        assert_eq!(tx.source_chain, Chain::Polkadot);
        assert_eq!(tx.destination_chain, Chain::Ethereum);
    }

    #[test]
    fn test_transaction_builder_substrate_to_substrate() {
        let tx = TransactionBuilder::new()
            .from_substrate_account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
            .to_substrate_account("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")
            .amount(2000)
            .build();

        assert!(tx.is_ok());
        let tx = tx.unwrap();
        assert!(!tx.is_cross_chain());
        assert_eq!(tx.source_chain, Chain::Polkadot);
        assert_eq!(tx.destination_chain, Chain::Polkadot);
    }

    #[test]
    fn test_transaction_builder_with_explicit_chain() {
        let tx = TransactionBuilder::new()
            .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .to_evm_address("0x1234567890123456789012345678901234567890")
            .amount(1000)
            .on_chain(Chain::Polygon)
            .build();

        assert!(tx.is_ok());
        let tx = tx.unwrap();
        assert_eq!(tx.source_chain, Chain::Polygon);
    }

    #[test]
    fn test_transaction_builder_missing_from() {
        let result = TransactionBuilder::new()
            .to_evm_address("0x1234567890123456789012345678901234567890")
            .amount(100)
            .build();

        assert!(result.is_err());
        match result {
            Err(Error::Transaction(msg, _)) => {
                assert!(msg.contains("Sender address required"));
            }
            _ => panic!("Expected Transaction error"),
        }
    }

    #[test]
    fn test_transaction_builder_missing_to() {
        let result = TransactionBuilder::new()
            .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .amount(100)
            .build();

        assert!(result.is_err());
        match result {
            Err(Error::Transaction(msg, _)) => {
                assert!(msg.contains("Recipient address required"));
            }
            _ => panic!("Expected Transaction error"),
        }
    }

    #[test]
    fn test_transaction_builder_missing_amount() {
        let result = TransactionBuilder::new()
            .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .to_evm_address("0x1234567890123456789012345678901234567890")
            .build();

        assert!(result.is_err());
        match result {
            Err(Error::Transaction(msg, _)) => {
                assert!(msg.contains("Amount required"));
            }
            _ => panic!("Expected Transaction error"),
        }
    }

    #[test]
    fn test_transaction_with_data() {
        let tx = TransactionBuilder::new()
            .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .to_evm_address("0x1234567890123456789012345678901234567890")
            .amount(1000)
            .with_data(vec![1, 2, 3, 4])
            .with_gas_limit(21000)
            .build();

        assert!(tx.is_ok());
        let tx = tx.unwrap();
        assert_eq!(tx.data, Some(vec![1, 2, 3, 4]));
        assert_eq!(tx.gas_limit, Some(21000));
    }

    #[test]
    fn test_transaction_with_empty_data() {
        let tx = TransactionBuilder::new()
            .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
            .to_evm_address("0x1234567890123456789012345678901234567890")
            .amount(1000)
            .with_data(vec![])
            .build();

        assert!(tx.is_ok());
        let tx = tx.unwrap();
        assert_eq!(tx.data, Some(vec![]));
    }

    #[test]
    fn test_transaction_is_cross_chain() {
        let tx = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Polkadot,
            data: None,
            gas_limit: None,
            nonce: None,
        };

        assert!(tx.is_cross_chain());
    }

    #[test]
    fn test_transaction_is_not_cross_chain() {
        let tx = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: None,
            gas_limit: None,
            nonce: None,
        };

        assert!(!tx.is_cross_chain());
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: None,
            gas_limit: None,
            nonce: None,
        };

        let hash = tx.hash();
        assert!(hash.starts_with("0x"));
        assert_eq!(hash.len(), 66); // 0x + 64 hex chars
    }

    #[test]
    fn test_transaction_hash_determinism() {
        // Create identical transactions
        let tx1 = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: Some(vec![1, 2, 3, 4]),
            gas_limit: Some(21000),
            nonce: Some(42),
        };

        let tx2 = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: Some(vec![1, 2, 3, 4]),
            gas_limit: Some(21000),
            nonce: Some(42),
        };

        // Same parameters should produce same hash
        assert_eq!(tx1.hash(), tx2.hash());
    }

    #[test]
    fn test_transaction_hash_changes_with_nonce() {
        let tx1 = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: None,
            gas_limit: None,
            nonce: Some(1),
        };

        let tx2 = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: None,
            gas_limit: None,
            nonce: Some(2),
        };

        // Different nonce should produce different hash
        assert_ne!(tx1.hash(), tx2.hash());
    }

    #[test]
    fn test_transaction_hash_none_vs_some_data() {
        let tx_none = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: None,
            gas_limit: None,
            nonce: None,
        };

        let tx_empty = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: Some(vec![]),
            gas_limit: None,
            nonce: None,
        };

        // None and Some(empty) should produce different hashes
        assert_ne!(tx_none.hash(), tx_empty.hash());
    }

    #[test]
    fn test_transaction_clone() {
        let tx = Transaction {
            from: Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"),
            to: Address::evm("0x1234567890123456789012345678901234567890"),
            amount: 1000,
            source_chain: Chain::Ethereum,
            destination_chain: Chain::Ethereum,
            data: Some(vec![1, 2, 3]),
            gas_limit: Some(21000),
            nonce: Some(5),
        };

        let cloned = tx.clone();
        assert_eq!(tx.amount, cloned.amount);
        assert_eq!(tx.data, cloned.data);
        assert_eq!(tx.gas_limit, cloned.gas_limit);
        assert_eq!(tx.nonce, cloned.nonce);
    }

    #[test]
    fn test_transaction_result_serialization() {
        let result = TransactionResult {
            source_tx_hash: "0xabc123".to_string(),
            destination_tx_hash: Some("0xdef456".to_string()),
            status: TransactionStatus::Confirmed {
                block_hash: "0xblock123".to_string(),
                block_number: Some(12345),
            },
            block_number: Some(12345),
            gas_used: Some(21000),
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: TransactionResult = serde_json::from_str(&json).unwrap();

        assert_eq!(result.source_tx_hash, deserialized.source_tx_hash);
        assert_eq!(result.destination_tx_hash, deserialized.destination_tx_hash);
        assert_eq!(result.block_number, deserialized.block_number);
        assert_eq!(result.gas_used, deserialized.gas_used);
    }
}
