//! Mock implementations for testing
//!
//! This module provides comprehensive mock adapters and utilities for testing
//! blockchain interactions without requiring actual chain connections.
//!
//! # Features
//!
//! - Transaction status simulation
//! - Balance tracking
//! - Block number progression
//! - Network delay simulation
//! - Contract deployment mocking
//! - Event emission
//! - Failure injection for testing error paths
//!
//! # Example
//!
//! ```
//! use apex_sdk_core::mocks::MockChainAdapterBuilder;
//! use apex_sdk_types::{Address, TransactionStatus};
//!
//! # tokio_test::block_on(async {
//! let adapter = MockChainAdapterBuilder::new("TestChain")
//!     .with_balance(Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"), 1000)
//!     .with_block_number(100)
//!     .with_network_delay_ms(10)
//!     .build();
//!
//! let balance = adapter.get_balance(&Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")).await.unwrap();
//! assert_eq!(balance, 1000);
//! # });
//! ```

use crate::ChainAdapter;
use apex_sdk_types::{Address, Event, TransactionStatus};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Mock transaction record
#[derive(Debug, Clone)]
pub struct MockTransaction {
    /// Transaction hash
    pub hash: String,
    /// From address
    pub from: Address,
    /// To address
    pub to: Address,
    /// Amount transferred
    pub amount: u128,
    /// Transaction status
    pub status: TransactionStatus,
    /// Block number (if confirmed)
    pub block_number: Option<u64>,
}

/// Mock chain adapter for testing
#[derive(Clone)]
pub struct MockChainAdapter {
    chain_name: String,
    tx_statuses: Arc<Mutex<HashMap<String, TransactionStatus>>>,
    transactions: Arc<Mutex<Vec<MockTransaction>>>,
    balances: Arc<Mutex<HashMap<String, u128>>>,
    valid_addresses: Arc<Mutex<Vec<Address>>>,
    should_fail: Arc<Mutex<bool>>,
    failure_rate: Arc<Mutex<f32>>,
    call_count: Arc<Mutex<u32>>,
    block_number: Arc<Mutex<u64>>,
    network_delay_ms: Arc<Mutex<u64>>,
    deployed_contracts: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    events: Arc<Mutex<Vec<Event>>>,
    nonce_counter: Arc<Mutex<HashMap<String, u64>>>,
}

impl MockChainAdapter {
    /// Create a new mock adapter
    pub fn new(chain_name: impl Into<String>) -> Self {
        Self {
            chain_name: chain_name.into(),
            tx_statuses: Arc::new(Mutex::new(HashMap::new())),
            transactions: Arc::new(Mutex::new(Vec::new())),
            balances: Arc::new(Mutex::new(HashMap::new())),
            valid_addresses: Arc::new(Mutex::new(Vec::new())),
            should_fail: Arc::new(Mutex::new(false)),
            failure_rate: Arc::new(Mutex::new(0.0)),
            call_count: Arc::new(Mutex::new(0)),
            block_number: Arc::new(Mutex::new(0)),
            network_delay_ms: Arc::new(Mutex::new(0)),
            deployed_contracts: Arc::new(Mutex::new(HashMap::new())),
            events: Arc::new(Mutex::new(Vec::new())),
            nonce_counter: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a transaction status to mock
    pub fn add_tx_status(&self, tx_hash: String, status: TransactionStatus) {
        self.tx_statuses.lock().unwrap().insert(tx_hash, status);
    }

    /// Add a transaction record
    pub fn add_transaction(&self, tx: MockTransaction) {
        self.transactions.lock().unwrap().push(tx);
    }

    /// Set balance for an address
    pub fn set_balance(&self, address: Address, balance: u128) {
        self.balances
            .lock()
            .unwrap()
            .insert(address.as_str().to_string(), balance);
    }

    /// Get balance for an address
    pub async fn get_balance(&self, address: &Address) -> Result<u128, String> {
        self.apply_network_delay().await;
        self.increment_call_count();

        if self.should_fail() {
            return Err("Mock failure triggered".to_string());
        }

        Ok(*self
            .balances
            .lock()
            .unwrap()
            .get(address.as_str())
            .unwrap_or(&0))
    }

    /// Transfer balance between addresses
    pub async fn transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: u128,
    ) -> Result<String, String> {
        self.apply_network_delay().await;
        self.increment_call_count();

        if self.should_fail() {
            return Err("Mock failure triggered".to_string());
        }

        let mut balances = self.balances.lock().unwrap();

        // Get and copy balances before modifying
        let from_balance = *balances.get(from.as_str()).unwrap_or(&0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        let to_balance = *balances.get(to.as_str()).unwrap_or(&0);

        // Now update balances
        balances.insert(from.as_str().to_string(), from_balance - amount);
        balances.insert(to.as_str().to_string(), to_balance + amount);

        // Generate transaction hash
        let tx_hash = format!("0x{:x}", self.get_next_nonce(from.as_str()));

        // Record transaction
        let tx = MockTransaction {
            hash: tx_hash.clone(),
            from: from.clone(),
            to: to.clone(),
            amount,
            status: TransactionStatus::Confirmed {
                block_hash: format!("0xblock{}", self.get_block_number()),
                block_number: Some(self.get_block_number()),
            },
            block_number: Some(self.get_block_number()),
        };

        self.add_transaction(tx);
        Ok(tx_hash)
    }

    /// Add a valid address
    pub fn add_valid_address(&self, address: Address) {
        self.valid_addresses.lock().unwrap().push(address);
    }

    /// Set whether operations should fail
    pub fn set_should_fail(&self, should_fail: bool) {
        *self.should_fail.lock().unwrap() = should_fail;
    }

    /// Set failure rate (0.0 to 1.0)
    pub fn set_failure_rate(&self, rate: f32) {
        *self.failure_rate.lock().unwrap() = rate.clamp(0.0, 1.0);
    }

    /// Check if operation should fail based on failure rate
    fn should_fail(&self) -> bool {
        if *self.should_fail.lock().unwrap() {
            return true;
        }

        let rate = *self.failure_rate.lock().unwrap();
        if rate > 0.0 {
            use rand::Rng;
            let mut rng = rand::rng();
            return rng.random::<f32>() < rate;
        }

        false
    }

    /// Get the number of times methods were called
    pub fn get_call_count(&self) -> u32 {
        *self.call_count.lock().unwrap()
    }

    /// Reset call count
    pub fn reset_call_count(&self) {
        *self.call_count.lock().unwrap() = 0;
    }

    fn increment_call_count(&self) {
        *self.call_count.lock().unwrap() += 1;
    }

    /// Set current block number
    pub fn set_block_number(&self, block_number: u64) {
        *self.block_number.lock().unwrap() = block_number;
    }

    /// Get current block number
    pub fn get_block_number(&self) -> u64 {
        *self.block_number.lock().unwrap()
    }

    /// Increment block number (simulate block progression)
    pub fn mine_block(&self) -> u64 {
        let mut block = self.block_number.lock().unwrap();
        *block += 1;
        *block
    }

    /// Set network delay in milliseconds
    pub fn set_network_delay_ms(&self, delay_ms: u64) {
        *self.network_delay_ms.lock().unwrap() = delay_ms;
    }

    /// Apply network delay simulation
    async fn apply_network_delay(&self) {
        let delay = *self.network_delay_ms.lock().unwrap();
        if delay > 0 {
            tokio::time::sleep(Duration::from_millis(delay)).await;
        }
    }

    /// Deploy a contract (mock)
    pub async fn deploy_contract(
        &self,
        bytecode: Vec<u8>,
        deployer: &Address,
    ) -> Result<Address, String> {
        self.apply_network_delay().await;
        self.increment_call_count();

        if self.should_fail() {
            return Err("Mock failure triggered".to_string());
        }

        // Generate contract address
        let nonce = self.get_next_nonce(deployer.as_str());
        let contract_addr = Address::evm(format!("0xcontract{:040x}", nonce % 0xffffffffffffffff));

        self.deployed_contracts
            .lock()
            .unwrap()
            .insert(contract_addr.as_str().to_string(), bytecode);

        Ok(contract_addr)
    }

    /// Get deployed contract bytecode
    pub fn get_contract_code(&self, address: &Address) -> Option<Vec<u8>> {
        self.deployed_contracts
            .lock()
            .unwrap()
            .get(address.as_str())
            .cloned()
    }

    /// Emit an event
    pub fn emit_event(&self, event: Event) {
        self.events.lock().unwrap().push(event);
    }

    /// Get all emitted events
    pub fn get_events(&self) -> Vec<Event> {
        self.events.lock().unwrap().clone()
    }

    /// Get events filtered by name
    pub fn get_events_by_name(&self, name: &str) -> Vec<Event> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.name == name)
            .cloned()
            .collect()
    }

    /// Clear all events
    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }

    /// Get transaction history
    pub fn get_transactions(&self) -> Vec<MockTransaction> {
        self.transactions.lock().unwrap().clone()
    }

    /// Get transactions for an address
    pub fn get_transactions_for_address(&self, address: &Address) -> Vec<MockTransaction> {
        self.transactions
            .lock()
            .unwrap()
            .iter()
            .filter(|tx| &tx.from == address || &tx.to == address)
            .cloned()
            .collect()
    }

    /// Get next nonce for an address
    fn get_next_nonce(&self, address: &str) -> u64 {
        let mut nonces = self.nonce_counter.lock().unwrap();
        let nonce = nonces.entry(address.to_string()).or_insert(0);
        *nonce += 1;
        *nonce
    }

    /// Reset the mock adapter to initial state
    pub fn reset(&self) {
        self.tx_statuses.lock().unwrap().clear();
        self.transactions.lock().unwrap().clear();
        self.balances.lock().unwrap().clear();
        self.valid_addresses.lock().unwrap().clear();
        *self.should_fail.lock().unwrap() = false;
        *self.failure_rate.lock().unwrap() = 0.0;
        *self.call_count.lock().unwrap() = 0;
        *self.block_number.lock().unwrap() = 0;
        *self.network_delay_ms.lock().unwrap() = 0;
        self.deployed_contracts.lock().unwrap().clear();
        self.events.lock().unwrap().clear();
        self.nonce_counter.lock().unwrap().clear();
    }
}

#[async_trait]
impl ChainAdapter for MockChainAdapter {
    async fn get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus, String> {
        self.apply_network_delay().await;
        self.increment_call_count();

        if self.should_fail() {
            return Err("Mock failure triggered".to_string());
        }

        self.tx_statuses
            .lock()
            .unwrap()
            .get(tx_hash)
            .cloned()
            .ok_or_else(|| format!("Transaction not found: {}", tx_hash))
    }

    fn validate_address(&self, address: &Address) -> bool {
        // If no valid addresses are specified, all addresses are valid
        let valid_addrs = self.valid_addresses.lock().unwrap();
        if valid_addrs.is_empty() {
            return true;
        }

        valid_addrs.iter().any(|addr| match (addr, address) {
            (Address::Substrate(a), Address::Substrate(b)) => a == b,
            (Address::Evm(a), Address::Evm(b)) => a.eq_ignore_ascii_case(b),
            _ => false,
        })
    }

    fn chain_name(&self) -> &str {
        &self.chain_name
    }
}

/// Builder for creating mock chain adapters with fluent API
pub struct MockChainAdapterBuilder {
    adapter: MockChainAdapter,
}

impl MockChainAdapterBuilder {
    /// Create a new builder
    pub fn new(chain_name: impl Into<String>) -> Self {
        Self {
            adapter: MockChainAdapter::new(chain_name),
        }
    }

    /// Add a transaction status
    pub fn with_tx_status(self, tx_hash: String, status: TransactionStatus) -> Self {
        self.adapter.add_tx_status(tx_hash, status);
        self
    }

    /// Add a transaction record
    pub fn with_transaction(self, tx: MockTransaction) -> Self {
        self.adapter.add_transaction(tx);
        self
    }

    /// Set balance for an address
    pub fn with_balance(self, address: Address, balance: u128) -> Self {
        self.adapter.set_balance(address, balance);
        self
    }

    /// Add a valid address
    pub fn with_valid_address(self, address: Address) -> Self {
        self.adapter.add_valid_address(address);
        self
    }

    /// Set failure mode
    pub fn with_failure(self, should_fail: bool) -> Self {
        self.adapter.set_should_fail(should_fail);
        self
    }

    /// Set failure rate (0.0 to 1.0)
    pub fn with_failure_rate(self, rate: f32) -> Self {
        self.adapter.set_failure_rate(rate);
        self
    }

    /// Set block number
    pub fn with_block_number(self, block_number: u64) -> Self {
        self.adapter.set_block_number(block_number);
        self
    }

    /// Set network delay in milliseconds
    pub fn with_network_delay_ms(self, delay_ms: u64) -> Self {
        self.adapter.set_network_delay_ms(delay_ms);
        self
    }

    /// Add a deployed contract
    pub fn with_contract(self, address: Address, bytecode: Vec<u8>) -> Self {
        self.adapter
            .deployed_contracts
            .lock()
            .unwrap()
            .insert(address.as_str().to_string(), bytecode);
        self
    }

    /// Add an event
    pub fn with_event(self, event: Event) -> Self {
        self.adapter.emit_event(event);
        self
    }

    /// Build the mock adapter
    pub fn build(self) -> MockChainAdapter {
        self.adapter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_adapter_basic() {
        let adapter = MockChainAdapter::new("TestChain");
        assert_eq!(adapter.chain_name(), "TestChain");
    }

    #[tokio::test]
    async fn test_mock_adapter_tx_status() {
        let adapter = MockChainAdapter::new("TestChain");
        adapter.add_tx_status(
            "0x123".to_string(),
            TransactionStatus::Confirmed {
                block_hash: "0xabc".to_string(),
                block_number: Some(100),
            },
        );

        let status = adapter.get_transaction_status("0x123").await.unwrap();
        match status {
            TransactionStatus::Confirmed { block_number, .. } => {
                assert_eq!(block_number, Some(100));
            }
            _ => panic!("Wrong status type"),
        }
    }

    #[tokio::test]
    async fn test_mock_adapter_address_validation() {
        let adapter = MockChainAdapter::new("TestChain");
        let addr = Address::Evm("0x1234567890123456789012345678901234567890".to_string());
        adapter.add_valid_address(addr.clone());

        assert!(adapter.validate_address(&addr));
        assert!(!adapter.validate_address(&Address::Evm("0xinvalid".to_string())));
    }

    #[tokio::test]
    async fn test_mock_adapter_failure_mode() {
        let adapter = MockChainAdapter::new("TestChain");
        adapter.set_should_fail(true);

        let result = adapter.get_transaction_status("0x123").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_adapter_call_count() {
        let adapter = MockChainAdapter::new("TestChain");
        adapter.add_tx_status("0x123".to_string(), TransactionStatus::Pending);

        let _ = adapter.get_transaction_status("0x123").await;
        let _ = adapter.get_transaction_status("0x123").await;

        assert_eq!(adapter.get_call_count(), 2);

        adapter.reset_call_count();
        assert_eq!(adapter.get_call_count(), 0);
    }

    #[tokio::test]
    async fn test_mock_adapter_builder() {
        let adapter = MockChainAdapterBuilder::new("TestChain")
            .with_tx_status("0x123".to_string(), TransactionStatus::Pending)
            .with_valid_address(Address::Evm(
                "0x1234567890123456789012345678901234567890".to_string(),
            ))
            .build();

        assert_eq!(adapter.chain_name(), "TestChain");
        assert!(adapter.get_transaction_status("0x123").await.is_ok());
    }

    #[tokio::test]
    async fn test_balance_tracking() {
        let adapter = MockChainAdapter::new("TestChain");
        let addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");

        // Initial balance is zero
        assert_eq!(adapter.get_balance(&addr).await.unwrap(), 0);

        // Set balance
        adapter.set_balance(addr.clone(), 1000);
        assert_eq!(adapter.get_balance(&addr).await.unwrap(), 1000);
    }

    #[tokio::test]
    async fn test_transfer() {
        let adapter = MockChainAdapter::new("TestChain");
        let from = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        let to = Address::evm("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");

        adapter.set_balance(from.clone(), 1000);

        // Transfer
        let tx_hash = adapter.transfer(&from, &to, 300).await.unwrap();
        assert!(!tx_hash.is_empty());

        // Check balances
        assert_eq!(adapter.get_balance(&from).await.unwrap(), 700);
        assert_eq!(adapter.get_balance(&to).await.unwrap(), 300);

        // Check transaction history
        let txs = adapter.get_transactions();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].amount, 300);
    }

    #[tokio::test]
    async fn test_transfer_insufficient_balance() {
        let adapter = MockChainAdapter::new("TestChain");
        let from = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        let to = Address::evm("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");

        adapter.set_balance(from.clone(), 100);

        let result = adapter.transfer(&from, &to, 200).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }

    #[tokio::test]
    async fn test_block_number() {
        let adapter = MockChainAdapter::new("TestChain");

        assert_eq!(adapter.get_block_number(), 0);

        adapter.set_block_number(100);
        assert_eq!(adapter.get_block_number(), 100);

        let block = adapter.mine_block();
        assert_eq!(block, 101);
        assert_eq!(adapter.get_block_number(), 101);
    }

    #[tokio::test]
    async fn test_network_delay() {
        let adapter = MockChainAdapter::new("TestChain");
        adapter.set_network_delay_ms(50);
        adapter.add_tx_status("0x123".to_string(), TransactionStatus::Pending);

        let start = std::time::Instant::now();
        let _ = adapter.get_transaction_status("0x123").await;
        let elapsed = start.elapsed();

        assert!(elapsed.as_millis() >= 50);
    }

    #[tokio::test]
    async fn test_contract_deployment() {
        let adapter = MockChainAdapter::new("TestChain");
        let deployer = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        let bytecode = vec![0x60, 0x80, 0x60, 0x40];

        let contract_addr = adapter
            .deploy_contract(bytecode.clone(), &deployer)
            .await
            .unwrap();

        // Verify contract was deployed
        let code = adapter.get_contract_code(&contract_addr).unwrap();
        assert_eq!(code, bytecode);
    }

    #[tokio::test]
    async fn test_event_emission() {
        let adapter = MockChainAdapter::new("TestChain");

        // Emit events
        adapter.emit_event(Event {
            name: "Transfer".to_string(),
            data: serde_json::json!({"from": "0x123", "to": "0x456", "value": 100}),
            block_number: Some(1),
            tx_hash: Some("0xabc".to_string()),
            index: Some(0),
        });

        adapter.emit_event(Event {
            name: "Approval".to_string(),
            data: serde_json::json!({"owner": "0x123", "spender": "0x789"}),
            block_number: Some(1),
            tx_hash: Some("0xdef".to_string()),
            index: Some(1),
        });

        // Get all events
        let events = adapter.get_events();
        assert_eq!(events.len(), 2);

        // Get events by name
        let transfers = adapter.get_events_by_name("Transfer");
        assert_eq!(transfers.len(), 1);
        assert_eq!(transfers[0].name, "Transfer");

        // Clear events
        adapter.clear_events();
        assert_eq!(adapter.get_events().len(), 0);
    }

    #[tokio::test]
    async fn test_transaction_history() {
        let adapter = MockChainAdapter::new("TestChain");
        let addr1 = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
        let addr2 = Address::evm("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
        let addr3 = Address::evm("0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359");

        adapter.set_balance(addr1.clone(), 1000);
        adapter.set_balance(addr2.clone(), 500);

        // Create some transactions
        adapter.transfer(&addr1, &addr2, 100).await.unwrap();
        adapter.transfer(&addr2, &addr3, 50).await.unwrap();

        // Get all transactions
        let all_txs = adapter.get_transactions();
        assert_eq!(all_txs.len(), 2);

        // Get transactions for specific address
        let addr1_txs = adapter.get_transactions_for_address(&addr1);
        assert_eq!(addr1_txs.len(), 1);

        let addr2_txs = adapter.get_transactions_for_address(&addr2);
        assert_eq!(addr2_txs.len(), 2); // Both as sender and receiver
    }

    #[tokio::test]
    async fn test_failure_rate() {
        let adapter = MockChainAdapter::new("TestChain");
        adapter.set_failure_rate(1.0); // 100% failure rate
        adapter.set_balance(Address::evm("0x123"), 1000);

        // Should always fail with 100% failure rate
        let result = adapter.get_balance(&Address::evm("0x123")).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset() {
        let adapter = MockChainAdapter::new("TestChain");
        let addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");

        // Add some data
        adapter.set_balance(addr.clone(), 1000);
        adapter.set_block_number(100);
        adapter.add_tx_status("0x123".to_string(), TransactionStatus::Pending);
        adapter.emit_event(Event {
            name: "Test".to_string(),
            data: serde_json::json!({}),
            block_number: None,
            tx_hash: None,
            index: None,
        });

        // Reset
        adapter.reset();

        // Verify everything is cleared
        assert_eq!(adapter.get_balance(&addr).await.unwrap(), 0);
        assert_eq!(adapter.get_block_number(), 0);
        assert!(adapter.get_transaction_status("0x123").await.is_err());
        assert_eq!(adapter.get_events().len(), 0);
    }

    #[tokio::test]
    async fn test_builder_comprehensive() {
        let addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");

        let adapter = MockChainAdapterBuilder::new("TestChain")
            .with_balance(addr.clone(), 5000)
            .with_block_number(500)
            .with_network_delay_ms(10)
            .with_tx_status("0xabc".to_string(), TransactionStatus::Pending)
            .with_event(Event {
                name: "TestEvent".to_string(),
                data: serde_json::json!({"test": true}),
                block_number: Some(500),
                tx_hash: None,
                index: None,
            })
            .build();

        assert_eq!(adapter.get_balance(&addr).await.unwrap(), 5000);
        assert_eq!(adapter.get_block_number(), 500);
        assert_eq!(adapter.get_events().len(), 1);
        assert!(adapter.get_transaction_status("0xabc").await.is_ok());
    }
}
