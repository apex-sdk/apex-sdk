use crate::{Error, Metrics, PolkadotConfig, Result};
use apex_sdk_core::ConfirmationStrategy;
use apex_sdk_types::TransactionStatus;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use subxt::OnlineClient;
use tokio::sync::{mpsc, oneshot, RwLock};
use tracing::{debug, error, info, warn};

/// Maximum time to keep a transaction in the watch list (5 minutes)
const MAX_WATCH_DURATION: Duration = Duration::from_secs(300);

/// Handle for a transaction being watched
struct TxWatchHandle {
    submitted_at: Instant,
    strategy: ConfirmationStrategy,
    sender: oneshot::Sender<TransactionStatus>,
    first_seen_block: Option<u64>,
}

/// Manages subscription-based transaction monitoring
pub struct TransactionMonitor {
    watch_tx: mpsc::UnboundedSender<(
        String,
        ConfirmationStrategy,
        oneshot::Sender<TransactionStatus>,
    )>,
}

impl TransactionMonitor {
    /// Create a new transaction monitor and start the subscription loop
    pub async fn new(client: OnlineClient<PolkadotConfig>, metrics: Arc<Metrics>) -> Result<Self> {
        let pending_txs = Arc::new(RwLock::new(HashMap::new()));
        let (watch_tx, watch_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            if let Err(e) = Self::run_monitor(client, pending_txs, metrics, watch_rx).await {
                error!("Transaction monitor error: {}", e);
            }
        });

        Ok(Self { watch_tx })
    }

    /// Watch a transaction with the given confirmation strategy
    /// Returns a receiver that will be notified when the transaction reaches the required confirmations
    pub async fn watch_transaction(
        &self,
        tx_hash: String,
        strategy: ConfirmationStrategy,
    ) -> oneshot::Receiver<TransactionStatus> {
        let (tx, rx) = oneshot::channel();

        if let Err(e) = self.watch_tx.send((tx_hash.clone(), strategy, tx)) {
            error!("Failed to add transaction to watch list: {}", e);
        } else {
            debug!("Added transaction to watch list: {}", tx_hash);
        }

        rx
    }

    /// Main monitoring loop that subscribes to finalized blocks
    async fn run_monitor(
        client: OnlineClient<PolkadotConfig>,
        pending_txs: Arc<RwLock<HashMap<String, TxWatchHandle>>>,
        metrics: Arc<Metrics>,
        mut watch_rx: mpsc::UnboundedReceiver<(
            String,
            ConfirmationStrategy,
            oneshot::Sender<TransactionStatus>,
        )>,
    ) -> Result<()> {
        info!("Starting transaction monitor subscription loop");

        loop {
            // Subscribe to finalized blocks
            match client.blocks().subscribe_finalized().await {
                Ok(mut subscription) => {
                    info!("Successfully subscribed to finalized blocks");

                    loop {
                        tokio::select! {
                            // Handle new transactions to watch
                            Some((tx_hash, strategy, sender)) = watch_rx.recv() => {
                                let handle = TxWatchHandle {
                                    submitted_at: Instant::now(),
                                    strategy,
                                    sender,
                                    first_seen_block: None,
                                };
                                pending_txs.write().await.insert(tx_hash, handle);
                                debug!("Now watching {} transactions", pending_txs.read().await.len());
                            }

                            // Handle finalized blocks
                            block_result = subscription.next() => {
                                match block_result {
                                    Some(Ok(block)) => {
                                        if let Err(e) = Self::process_finalized_block(
                                            &pending_txs,
                                            &metrics,
                                            block
                                        ).await {
                                            error!("Error processing finalized block: {}", e);
                                        }
                                    }
                                    Some(Err(e)) => {
                                        error!("Error receiving finalized block: {}", e);
                                        break;
                                    }
                                    None => {
                                        warn!("Finalized blocks subscription ended, reconnecting...");
                                        break;
                                    }
                                }
                            }

                            // Periodic cleanup of expired transactions
                            _ = tokio::time::sleep(Duration::from_secs(30)) => {
                                Self::cleanup_expired_transactions(&pending_txs).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to subscribe to finalized blocks: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    /// Process a finalized block and check for watched transactions
    async fn process_finalized_block(
        pending_txs: &Arc<RwLock<HashMap<String, TxWatchHandle>>>,
        metrics: &Arc<Metrics>,
        block: subxt::blocks::Block<PolkadotConfig, OnlineClient<PolkadotConfig>>,
    ) -> Result<()> {
        let block_number = block.number() as u64;
        let block_hash = block.hash();

        debug!("Processing finalized block #{}", block_number);

        // Get all extrinsics in this block
        let extrinsics = block
            .extrinsics()
            .await
            .map_err(|e| Error::Transaction(format!("Failed to get extrinsics: {}", e)))?;

        let pending = pending_txs.read().await;
        if pending.is_empty() {
            return Ok(());
        }

        let mut block_tx_hashes = HashMap::new();
        for ext_details in extrinsics.iter() {
            let ext_bytes = ext_details.bytes();
            let computed_hash = sp_core::blake2_256(ext_bytes);
            let tx_hash = format!("0x{}", hex::encode(computed_hash));
            let (success, error_msg) = match ext_details.events().await {
                Ok(events) => {
                    let mut success = false;
                    let mut error_msg = None;

                    for event in events.iter().flatten() {
                        if event.pallet_name() == "System" {
                            if event.variant_name() == "ExtrinsicSuccess" {
                                success = true;
                            } else if event.variant_name() == "ExtrinsicFailed" {
                                error_msg =
                                    Some(format!("Extrinsic failed at block {}", block_number));
                            }
                        }
                    }
                    (success, error_msg)
                }
                Err(_) => (false, Some("Failed to fetch events".to_string())),
            };

            block_tx_hashes.insert(tx_hash, (success, error_msg));
        }

        drop(pending);

        let mut to_complete = Vec::new();
        {
            let mut pending = pending_txs.write().await;

            for (tx_hash, handle) in pending.iter_mut() {
                if let Some((_success, _error_msg)) = block_tx_hashes.get(tx_hash) {
                    if handle.first_seen_block.is_none() {
                        handle.first_seen_block = Some(block_number);
                        info!(
                            "Transaction {} found in finalized block #{}",
                            tx_hash, block_number
                        );
                    }
                }

                if let Some(first_seen) = handle.first_seen_block {
                    let confirmations = block_number.saturating_sub(first_seen);

                    let is_complete = match &handle.strategy {
                        ConfirmationStrategy::Immediate => true,
                        ConfirmationStrategy::Finalized { .. } => true,
                        ConfirmationStrategy::BlockConfirmations {
                            confirmations: required,
                            ..
                        } => confirmations >= (*required as u64),
                    };

                    if is_complete {
                        let status =
                            if let Some((success, error_msg)) = block_tx_hashes.get(tx_hash) {
                                if *success {
                                    TransactionStatus::finalized(
                                        tx_hash.clone(),
                                        first_seen,
                                        format!("0x{}", hex::encode(block_hash.0)),
                                        None,
                                        None,
                                        Some(confirmations as u32),
                                    )
                                } else {
                                    TransactionStatus::failed(
                                        tx_hash.clone(),
                                        error_msg
                                            .clone()
                                            .unwrap_or_else(|| "Unknown error".to_string()),
                                    )
                                }
                            } else {
                                TransactionStatus::finalized(
                                    tx_hash.clone(),
                                    first_seen,
                                    format!("0x{}", hex::encode(block_hash.0)),
                                    None,
                                    None,
                                    Some(confirmations as u32),
                                )
                            };

                        to_complete.push((tx_hash.clone(), status));
                        metrics.record_transaction_success();
                    }
                }
            }

            for (tx_hash, status) in to_complete {
                if let Some(handle) = pending.remove(&tx_hash) {
                    let _ = handle.sender.send(status);
                    debug!("Completed watching transaction: {}", tx_hash);
                }
            }
        }

        Ok(())
    }

    /// Remove transactions that have exceeded the maximum watch duration
    async fn cleanup_expired_transactions(
        pending_txs: &Arc<RwLock<HashMap<String, TxWatchHandle>>>,
    ) {
        let now = Instant::now();
        let mut pending = pending_txs.write().await;
        let mut expired = Vec::new();

        for (tx_hash, handle) in pending.iter() {
            if now.duration_since(handle.submitted_at) > MAX_WATCH_DURATION {
                expired.push(tx_hash.clone());
            }
        }

        for tx_hash in expired {
            if let Some(handle) = pending.remove(&tx_hash) {
                let timeout_secs = match &handle.strategy {
                    ConfirmationStrategy::BlockConfirmations { timeout_secs, .. } => *timeout_secs,
                    ConfirmationStrategy::Finalized { timeout_secs } => *timeout_secs,
                    ConfirmationStrategy::Immediate => 0,
                };

                let status = TransactionStatus::failed(
                    tx_hash.clone(),
                    format!("Timeout after {} seconds", timeout_secs),
                );
                let _ = handle.sender.send(status);
                warn!(
                    "Transaction {} expired after {:?}",
                    tx_hash, MAX_WATCH_DURATION
                );
            }
        }

        if !pending.is_empty() {
            debug!("Active transactions being monitored: {}", pending.len());
        }
    }
}
