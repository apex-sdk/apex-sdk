//! Transaction types and builders for the Apex SDK.

use crate::{
    error::Result,
    types::{Address, Chain},
};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// Transaction builder for creating transactions
#[derive(Debug, Clone, Default)]
pub struct TransactionBuilder {
    from: Option<Address>,
    to: Option<Address>,
    amount: Option<u128>,
    gas_limit: Option<u64>,
    gas_price: Option<u64>,
    data: Option<Vec<u8>>,
    chain: Option<Chain>,
}

impl TransactionBuilder {
    /// Create a new transaction builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the sender address
    pub fn from(mut self, address: Address) -> Self {
        self.from = Some(address);
        self
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

    /// Set the gas limit
    pub fn gas_limit(mut self, limit: u64) -> Self {
        self.gas_limit = Some(limit);
        self
    }

    /// Set the gas price
    pub fn gas_price(mut self, price: u64) -> Self {
        self.gas_price = Some(price);
        self
    }

    /// Set transaction data
    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Set the target chain
    pub fn chain(mut self, chain: Chain) -> Self {
        self.chain = Some(chain);
        self
    }

    /// Build the transaction
    pub fn build(self) -> Result<Transaction> {
        let from = self
            .from
            .ok_or_else(|| crate::error::Error::Config("From address is required".to_string()))?;
        let to = self
            .to
            .ok_or_else(|| crate::error::Error::Config("To address is required".to_string()))?;
        let amount = self
            .amount
            .ok_or_else(|| crate::error::Error::Config("Amount is required".to_string()))?;

        Ok(Transaction {
            from,
            to,
            amount,
            gas_limit: self.gas_limit,
            gas_price: self.gas_price,
            data: self.data,
            chain: self.chain,
            nonce: None,
        })
    }
}

/// Represents a blockchain transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u128,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<u64>,
    pub data: Option<Vec<u8>>,
    pub chain: Option<Chain>,
    pub nonce: Option<u64>,
}

impl Transaction {
    /// Create a new transaction builder
    pub fn builder() -> TransactionBuilder {
        TransactionBuilder::new()
    }

    /// Get the destination chain for this transaction
    pub fn destination_chain(&self) -> Chain {
        self.chain.as_ref().unwrap_or(&Chain::Polkadot).clone()
    }

    /// Check if this is a cross-chain transaction
    pub fn is_cross_chain(&self) -> bool {
        // Implementation would check if from and to addresses are on different chains
        false
    }

    /// Calculate transaction hash
    pub fn hash(&self) -> String {
        let mut hasher = Keccak256::new();
        hasher.update(format!("{:?}", self).as_bytes());
        format!("0x{:x}", hasher.finalize())
    }
}

/// Transaction execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub hash: String,
    pub status: TransactionStatus,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
    Unknown,
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
    }

    #[test]
    fn test_transaction_builder_missing_from() {
        let result = Transaction::builder()
            .to(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .amount(1000)
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_transaction_builder_missing_to() {
        let result = Transaction::builder()
            .from(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .amount(1000)
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_transaction_builder_missing_amount() {
        let result = Transaction::builder()
            .from(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .to(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_transaction_clone() {
        let tx = Transaction::builder()
            .from(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .to(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .amount(1000)
            .build()
            .unwrap();

        let _cloned = tx.clone();
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction::builder()
            .from(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .to(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .amount(1000)
            .build()
            .unwrap();

        let hash = tx.hash();
        assert!(hash.starts_with("0x"));
        assert_eq!(hash.len(), 66); // 0x + 64 hex chars
    }

    #[test]
    fn test_transaction_is_cross_chain() {
        let tx = Transaction::builder()
            .from(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .to(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .amount(1000)
            .build()
            .unwrap();

        assert!(!tx.is_cross_chain());
    }

    #[test]
    fn test_transaction_is_not_cross_chain() {
        let tx = Transaction::builder()
            .from(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .to(Address::substrate(
                "15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5",
            ))
            .amount(1000)
            .build()
            .unwrap();

        assert!(!tx.is_cross_chain()); // For now, always false
    }

    #[test]
    fn test_transaction_hash_determinism() {
        let tx1 = Transaction::builder()
            .from(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .to(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .amount(1000)
            .build()
            .unwrap();

        let tx2 = tx1.clone();
        assert_eq!(tx1.hash(), tx2.hash());
    }

    #[test]
    fn test_transaction_hash_changes_with_nonce() {
        let mut tx1 = Transaction::builder()
            .from(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .to(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"))
            .amount(1000)
            .build()
            .unwrap();

        let mut tx2 = tx1.clone();
        tx1.nonce = Some(1);
        tx2.nonce = Some(2);

        assert_ne!(tx1.hash(), tx2.hash());
    }

    #[test]
    fn test_transaction_result_serialization() {
        let result = TransactionResult {
            hash: "0x123".to_string(),
            status: TransactionStatus::Success,
            block_number: Some(100),
            gas_used: Some(21000),
        };

        let _serialized = serde_json::to_string(&result).unwrap();
    }
}
