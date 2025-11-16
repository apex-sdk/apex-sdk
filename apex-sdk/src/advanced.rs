//! Advanced features for Apex SDK
//!
//! This module provides advanced blockchain interaction features including:
//! - Event subscriptions and monitoring
//! - Parallel transaction execution
//! - Transaction batching
//! - Block monitoring

use crate::error::{Error, Result};
use apex_sdk_types::{Event, EventFilter};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// Event subscription manager
#[derive(Clone)]
pub struct EventSubscription {
    filter: EventFilter,
    sender: broadcast::Sender<Event>,
    active: Arc<RwLock<bool>>,
}

impl EventSubscription {
    /// Create a new event subscription
    pub fn new(filter: EventFilter) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            filter,
            sender,
            active: Arc::new(RwLock::new(true)),
        }
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    /// Emit an event
    pub async fn emit(&self, event: Event) -> Result<()> {
        if !*self.active.read().await {
            return Err(Error::Other("Subscription is not active".to_string()));
        }

        if self.matches_filter(&event) {
            self.sender
                .send(event)
                .map_err(|e| Error::Other(format!("Failed to send event: {}", e)))?;
        }

        Ok(())
    }

    /// Check if event matches filter
    fn matches_filter(&self, event: &Event) -> bool {
        // Filter by event names
        if let Some(ref names) = self.filter.event_names {
            if !names.contains(&event.name) {
                return false;
            }
        }

        // Filter by block range
        if let Some(block_number) = event.block_number {
            if let Some(from_block) = self.filter.from_block {
                if block_number < from_block {
                    return false;
                }
            }
            if let Some(to_block) = self.filter.to_block {
                if block_number > to_block {
                    return false;
                }
            }
        }

        true
    }

    /// Stop the subscription
    pub async fn stop(&self) {
        *self.active.write().await = false;
    }

    /// Check if subscription is active
    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }
}

/// Block subscription for monitoring new blocks
#[derive(Clone)]
pub struct BlockSubscription {
    sender: broadcast::Sender<BlockInfo>,
    active: Arc<RwLock<bool>>,
}

/// Block information
#[derive(Debug, Clone)]
pub struct BlockInfo {
    /// Block number
    pub number: u64,
    /// Block hash
    pub hash: String,
    /// Parent hash
    pub parent_hash: String,
    /// Timestamp
    pub timestamp: u64,
    /// Number of transactions
    pub tx_count: u32,
}

impl BlockSubscription {
    /// Create a new block subscription
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            sender,
            active: Arc::new(RwLock::new(true)),
        }
    }

    /// Subscribe to new blocks
    pub fn subscribe(&self) -> broadcast::Receiver<BlockInfo> {
        self.sender.subscribe()
    }

    /// Emit a new block
    pub async fn emit(&self, block: BlockInfo) -> Result<()> {
        if !*self.active.read().await {
            return Err(Error::Other("Subscription is not active".to_string()));
        }

        self.sender
            .send(block)
            .map_err(|e| Error::Other(format!("Failed to send block: {}", e)))?;

        Ok(())
    }

    /// Stop the subscription
    pub async fn stop(&self) {
        *self.active.write().await = false;
    }

    /// Check if subscription is active
    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }
}

impl Default for BlockSubscription {
    fn default() -> Self {
        Self::new()
    }
}

/// Parallel transaction executor
pub struct ParallelExecutor {
    max_concurrent: usize,
}

impl ParallelExecutor {
    /// Create a new parallel executor
    pub fn new(max_concurrent: usize) -> Self {
        Self { max_concurrent }
    }

    /// Execute transactions in parallel
    pub async fn execute<F, Fut, T>(&self, transactions: Vec<F>) -> Vec<Result<T>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send + 'static,
        T: Send + 'static,
    {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.max_concurrent));
        let mut handles = vec![];

        for tx in transactions {
            let permit = semaphore.clone();
            let handle = tokio::spawn(async move {
                let _permit = permit.acquire().await.unwrap();
                tx().await
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(Error::Other(format!("Task failed: {}", e)))),
            }
        }

        results
    }

    /// Execute transactions in parallel with timeout
    pub async fn execute_with_timeout<F, Fut, T>(
        &self,
        transactions: Vec<F>,
        timeout: std::time::Duration,
    ) -> Vec<Result<T>>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<T>> + Send + 'static,
        T: Send + 'static,
    {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.max_concurrent));
        let mut handles = vec![];

        for tx in transactions {
            let permit = semaphore.clone();
            let handle = tokio::spawn(async move {
                let _permit = permit.acquire().await.unwrap();
                match tokio::time::timeout(timeout, tx()).await {
                    Ok(result) => result,
                    Err(_) => Err(Error::transaction("Transaction timeout")),
                }
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(Error::Other(format!("Task failed: {}", e)))),
            }
        }

        results
    }
}

impl Default for ParallelExecutor {
    fn default() -> Self {
        Self::new(10)
    }
}

/// Transaction batch builder
#[derive(Default)]
pub struct TransactionBatch {
    transactions: Vec<Vec<u8>>,
}

impl TransactionBatch {
    /// Create a new transaction batch
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a transaction to the batch
    pub fn add(&mut self, tx: Vec<u8>) -> &mut Self {
        self.transactions.push(tx);
        self
    }

    /// Add multiple transactions
    pub fn add_many(&mut self, txs: Vec<Vec<u8>>) -> &mut Self {
        self.transactions.extend(txs);
        self
    }

    /// Get the number of transactions
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Check if batch is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Clear the batch
    pub fn clear(&mut self) {
        self.transactions.clear();
    }

    /// Get all transactions
    pub fn transactions(&self) -> &[Vec<u8>] {
        &self.transactions
    }

    /// Take all transactions and clear the batch
    pub fn take(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_event_subscription() {
        let filter = EventFilter {
            event_names: Some(vec!["Transfer".to_string()]),
            addresses: None,
            from_block: None,
            to_block: None,
        };

        let subscription = EventSubscription::new(filter);
        let mut receiver = subscription.subscribe();

        let event = Event {
            name: "Transfer".to_string(),
            data: serde_json::json!({"from": "0x123", "to": "0x456", "amount": 100}),
            block_number: Some(100),
            tx_hash: Some("0xabc".to_string()),
            index: Some(0),
        };

        subscription.emit(event.clone()).await.unwrap();

        let received = receiver.recv().await.unwrap();
        assert_eq!(received.name, "Transfer");
        assert_eq!(received.block_number, Some(100));
    }

    #[tokio::test]
    async fn test_block_subscription() {
        let subscription = BlockSubscription::new();
        let mut receiver = subscription.subscribe();

        let block = BlockInfo {
            number: 100,
            hash: "0xabc".to_string(),
            parent_hash: "0x123".to_string(),
            timestamp: 1234567890,
            tx_count: 10,
        };

        subscription.emit(block.clone()).await.unwrap();

        let received = receiver.recv().await.unwrap();
        assert_eq!(received.number, 100);
        assert_eq!(received.hash, "0xabc");
    }

    #[tokio::test]
    async fn test_parallel_executor() {
        let executor = ParallelExecutor::new(5);

        let transactions: Vec<_> = (0..10)
            .map(|i| {
                move || async move {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    Ok::<_, Error>(i * 2)
                }
            })
            .collect();

        let results = executor.execute(transactions).await;

        assert_eq!(results.len(), 10);
        for (i, result) in results.iter().enumerate() {
            assert_eq!(result.as_ref().unwrap(), &(i * 2));
        }
    }

    #[tokio::test]
    async fn test_parallel_executor_with_timeout() {
        let executor = ParallelExecutor::new(5);

        let transactions: Vec<_> = (0..5)
            .map(|i| {
                move || async move {
                    if i == 2 {
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    } else {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                    Ok::<_, Error>(i)
                }
            })
            .collect();

        let results = executor
            .execute_with_timeout(transactions, Duration::from_millis(100))
            .await;

        assert_eq!(results.len(), 5);
        assert!(results[2].is_err()); // Should timeout
    }

    #[test]
    fn test_transaction_batch() {
        let mut batch = TransactionBatch::new();

        batch.add(vec![1, 2, 3]);
        batch.add(vec![4, 5, 6]);

        assert_eq!(batch.len(), 2);
        assert!(!batch.is_empty());

        let txs = batch.take();
        assert_eq!(txs.len(), 2);
        assert!(batch.is_empty());
    }

    #[tokio::test]
    async fn test_subscription_stop() {
        let subscription = BlockSubscription::new();

        assert!(subscription.is_active().await);

        subscription.stop().await;

        assert!(!subscription.is_active().await);

        let block = BlockInfo {
            number: 100,
            hash: "0xabc".to_string(),
            parent_hash: "0x123".to_string(),
            timestamp: 1234567890,
            tx_count: 10,
        };

        assert!(subscription.emit(block).await.is_err());
    }
}
