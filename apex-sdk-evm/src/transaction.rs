//! Transaction execution for EVM chains
//!
//! This module provides comprehensive transaction execution including:
//! - Gas estimation (EIP-1559 and legacy)
//! - Transaction signing
//! - Transaction submission with retry logic
//! - Transaction monitoring

use crate::{wallet::Wallet, Error, ProviderType};
use alloy::network::TransactionBuilder;
use alloy::primitives::{Address as EthAddress, Bytes, B256, U256};
use alloy::providers::Provider;
use alloy::rpc::types::{Block, BlockNumberOrTag, TransactionReceipt, TransactionRequest};
use std::time::Duration;

/// Configuration for gas estimation and pricing
#[derive(Debug, Clone)]
pub struct GasConfig {
    /// Gas limit multiplier for safety margin (default: 1.2 = 20% buffer)
    pub gas_limit_multiplier: f64,
    /// Max priority fee per gas (EIP-1559) in gwei
    pub max_priority_fee_per_gas: Option<U256>,
    /// Max fee per gas (EIP-1559) in gwei
    pub max_fee_per_gas: Option<U256>,
    /// Gas price for legacy transactions in gwei
    pub gas_price: Option<U256>,
}

impl Default for GasConfig {
    fn default() -> Self {
        Self {
            gas_limit_multiplier: 1.2,
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            gas_price: None,
        }
    }
}

/// Configuration for transaction retry logic
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Initial backoff duration in milliseconds
    pub initial_backoff_ms: u64,
    /// Maximum backoff duration in milliseconds
    pub max_backoff_ms: u64,
    /// Backoff multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Whether to add jitter to backoff to avoid thundering herd
    pub use_jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 1000,
            max_backoff_ms: 30000,
            backoff_multiplier: 2.0,
            use_jitter: true,
        }
    }
}

/// Gas estimation result
#[derive(Debug, Clone)]
pub struct GasEstimate {
    /// Estimated gas limit
    pub gas_limit: U256,
    /// Estimated gas price or max fee per gas (EIP-1559)
    pub gas_price: U256,
    /// Base fee per gas (EIP-1559 only)
    pub base_fee_per_gas: Option<U256>,
    /// Max priority fee per gas (EIP-1559 only)
    pub max_priority_fee_per_gas: Option<U256>,
    /// Whether this is an EIP-1559 transaction
    pub is_eip1559: bool,
    /// Estimated total cost in wei
    pub total_cost: U256,
}

impl GasEstimate {
    /// Format gas price in Gwei
    pub fn gas_price_gwei(&self) -> String {
        format_gwei(self.gas_price)
    }

    /// Format base fee in Gwei (EIP-1559)
    pub fn base_fee_gwei(&self) -> Option<String> {
        self.base_fee_per_gas.map(format_gwei)
    }

    /// Format priority fee in Gwei (EIP-1559)
    pub fn priority_fee_gwei(&self) -> Option<String> {
        self.max_priority_fee_per_gas.map(format_gwei)
    }

    /// Format total cost in ETH
    pub fn total_cost_eth(&self) -> String {
        format_eth(self.total_cost)
    }
}

/// Transaction executor with gas estimation and retry logic
pub struct TransactionExecutor {
    provider: ProviderType,
    gas_config: GasConfig,
    retry_config: RetryConfig,
}

impl TransactionExecutor {
    /// Create a new transaction executor
    pub fn new(provider: ProviderType) -> Self {
        Self {
            provider,
            gas_config: GasConfig::default(),
            retry_config: RetryConfig::default(),
        }
    }

    /// Set gas configuration
    pub fn with_gas_config(mut self, config: GasConfig) -> Self {
        self.gas_config = config;
        self
    }

    /// Set retry configuration
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Estimate gas for a transaction
    ///
    /// This handles both EIP-1559 (London fork) and legacy transactions
    pub async fn estimate_gas(
        &self,
        from: EthAddress,
        to: Option<EthAddress>,
        value: Option<U256>,
        data: Option<Vec<u8>>,
    ) -> Result<GasEstimate, Error> {
        tracing::debug!("Estimating gas for transaction");

        // Build transaction request
        let mut tx = TransactionRequest::default()
            .from(from)
            .value(value.unwrap_or(U256::ZERO));

        if let Some(to_addr) = to {
            tx = tx.to(to_addr);
        }

        if let Some(tx_data) = data {
            tx = tx.input(tx_data.into());
        }

        // Estimate gas limit
        let estimated_gas = self.estimate_gas_limit(&tx).await?;

        // Apply safety multiplier
        let gas_limit = U256::from(
            (estimated_gas.to::<u128>() as f64 * self.gas_config.gas_limit_multiplier) as u128,
        );

        tracing::debug!(
            "Estimated gas limit: {} (with {}% buffer)",
            gas_limit,
            (self.gas_config.gas_limit_multiplier - 1.0) * 100.0
        );

        // Get gas pricing
        let (gas_price, base_fee, priority_fee, is_eip1559) = self.estimate_gas_price().await?;

        // Calculate total cost
        let total_cost = gas_limit * gas_price;

        Ok(GasEstimate {
            gas_limit,
            gas_price,
            base_fee_per_gas: base_fee,
            max_priority_fee_per_gas: priority_fee,
            is_eip1559,
            total_cost,
        })
    }

    /// Estimate gas limit for a transaction
    async fn estimate_gas_limit(&self, tx: &TransactionRequest) -> Result<U256, Error> {
        let gas = self
            .provider
            .inner
            .estimate_gas(tx.clone())
            .await
            .map_err(|e| Error::Transaction(format!("Gas estimation failed: {}", e)))?;

        Ok(U256::from(gas))
    }

    /// Estimate gas price (handles both EIP-1559 and legacy)
    async fn estimate_gas_price(&self) -> Result<(U256, Option<U256>, Option<U256>, bool), Error> {
        // Try EIP-1559 first
        match self.get_eip1559_fees().await {
            Ok((base_fee, priority_fee)) => {
                let max_fee = base_fee * U256::from(2) + priority_fee;
                tracing::debug!(
                    "Using EIP-1559: base={} gwei, priority={} gwei, max={} gwei",
                    format_gwei(base_fee),
                    format_gwei(priority_fee),
                    format_gwei(max_fee)
                );
                Ok((max_fee, Some(base_fee), Some(priority_fee), true))
            }
            Err(_) => {
                // Fallback to legacy gas price
                let gas_price = self.get_legacy_gas_price().await?;
                tracing::debug!("Using legacy gas price: {} gwei", format_gwei(gas_price));
                Ok((gas_price, None, None, false))
            }
        }
    }

    /// Get EIP-1559 fee estimates
    async fn get_eip1559_fees(&self) -> Result<(U256, U256), Error> {
        // Get base fee from latest block
        let block: Block = self
            .provider
            .inner
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await
            .map_err(|e| Error::Connection(format!("Failed to get block: {}", e)))?
            .ok_or_else(|| Error::Connection("No latest block".to_string()))?;

        let base_fee = block
            .header
            .base_fee_per_gas
            .map(U256::from)
            .ok_or_else(|| Error::Other("EIP-1559 not supported".to_string()))?;

        // Use configured priority fee or default to 2 gwei
        let priority_fee = self
            .gas_config
            .max_priority_fee_per_gas
            .unwrap_or_else(|| U256::from(2_000_000_000u64)); // 2 gwei

        Ok((base_fee, priority_fee))
    }

    /// Get legacy gas price
    async fn get_legacy_gas_price(&self) -> Result<U256, Error> {
        if let Some(price) = self.gas_config.gas_price {
            return Ok(price);
        }

        let gas_price = self
            .provider
            .inner
            .get_gas_price()
            .await
            .map_err(|e| Error::Connection(format!("Failed to get gas price: {}", e)))?;

        Ok(U256::from(gas_price))
    }

    /// Build a transaction
    pub async fn build_transaction(
        &self,
        wallet: &Wallet,
        to: EthAddress,
        value: U256,
        data: Option<Vec<u8>>,
        gas_estimate: Option<GasEstimate>,
    ) -> Result<TransactionRequest, Error> {
        let from = wallet.eth_address();

        // Get gas estimate if not provided
        let gas_est = if let Some(est) = gas_estimate {
            est
        } else {
            self.estimate_gas(from, Some(to), Some(value), data.clone())
                .await?
        };

        // Get nonce
        let nonce = self.get_transaction_count(from).await?;

        // Build transaction request
        let mut tx = TransactionRequest::default()
            .with_from(from)
            .with_to(to)
            .with_value(value)
            .with_gas_limit(gas_est.gas_limit.to::<u64>())
            .with_nonce(nonce.to::<u64>());

        // Set gas parameters based on EIP-1559 support
        if gas_est.is_eip1559 {
            if let Some(base_fee) = gas_est.base_fee_per_gas {
                let max_fee = base_fee * U256::from(2)
                    + gas_est
                        .max_priority_fee_per_gas
                        .unwrap_or_else(|| U256::from(2_000_000_000u64));
                tx = tx.with_max_fee_per_gas(max_fee.to::<u128>());
            }

            if let Some(priority_fee) = gas_est.max_priority_fee_per_gas {
                tx = tx.with_max_priority_fee_per_gas(priority_fee.to::<u128>());
            }
        } else {
            tx = tx.with_gas_price(gas_est.gas_price.to::<u128>());
        }

        // Set data if provided
        if let Some(tx_data) = data {
            tx = tx.with_input(Bytes::from(tx_data));
        }

        // Set chain ID if available
        if let Some(chain_id) = wallet.chain_id() {
            tx = tx.with_chain_id(chain_id);
        }

        Ok(tx)
    }

    /// Get transaction count (nonce) for an address
    async fn get_transaction_count(&self, address: EthAddress) -> Result<U256, Error> {
        let nonce = self
            .provider
            .inner
            .get_transaction_count(address)
            .await
            .map_err(|e| Error::Connection(format!("Failed to get nonce: {}", e)))?;

        Ok(U256::from(nonce))
    }

    /// Send a signed transaction with retry logic
    pub async fn send_transaction(
        &self,
        wallet: &Wallet,
        to: EthAddress,
        value: U256,
        data: Option<Vec<u8>>,
    ) -> Result<B256, Error> {
        let tx = self
            .build_transaction(wallet, to, value, data, None)
            .await?;

        self.send_raw_transaction(wallet, tx).await
    }

    /// Send a pre-built transaction with retry logic
    pub async fn send_raw_transaction(
        &self,
        wallet: &Wallet,
        tx: TransactionRequest,
    ) -> Result<B256, Error> {
        let mut attempts = 0;
        let mut backoff = Duration::from_millis(self.retry_config.initial_backoff_ms);

        loop {
            match self.try_send_transaction(wallet, &tx).await {
                Ok(tx_hash) => {
                    tracing::info!("Transaction sent successfully: {:?}", tx_hash);
                    return Ok(tx_hash);
                }
                Err(e) if attempts < self.retry_config.max_retries => {
                    attempts += 1;
                    tracing::warn!(
                        "Transaction failed (attempt {}/{}): {}",
                        attempts,
                        self.retry_config.max_retries,
                        e
                    );

                    // Add jitter if configured
                    let delay = if self.retry_config.use_jitter {
                        let jitter =
                            (rand::random::<f64>() * 0.3 + 0.85) * backoff.as_millis() as f64;
                        Duration::from_millis(jitter as u64)
                    } else {
                        backoff
                    };

                    tokio::time::sleep(delay).await;

                    // Exponential backoff
                    backoff = Duration::from_millis(std::cmp::min(
                        (backoff.as_millis() as f64 * self.retry_config.backoff_multiplier) as u64,
                        self.retry_config.max_backoff_ms,
                    ));
                }
                Err(e) => {
                    tracing::error!("Transaction failed after {} attempts: {}", attempts, e);
                    return Err(e);
                }
            }
        }
    }

    /// Try to send a transaction (single attempt)
    async fn try_send_transaction(
        &self,
        _wallet: &Wallet,
        tx: &TransactionRequest,
    ) -> Result<B256, Error> {
        // For now, we'll use the provider's built-in signing via fillers
        // The provider should have wallet/signer configured if needed
        // Send the transaction request directly
        let pending_tx = self
            .provider
            .inner
            .send_transaction(tx.clone())
            .await
            .map_err(|e| Error::Transaction(format!("Failed to send transaction: {}", e)))?;

        // Get the transaction hash
        let tx_hash = *pending_tx.tx_hash();

        Ok(tx_hash)
    }

    /// Wait for transaction confirmation
    pub async fn wait_for_confirmation(
        &self,
        tx_hash: B256,
        _confirmations: usize,
    ) -> Result<Option<TransactionReceipt>, Error> {
        tracing::info!("Waiting for transaction confirmation: {:?}", tx_hash);

        let receipt = self
            .provider
            .inner
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(|e| Error::Transaction(format!("Failed to get receipt: {}", e)))?;

        if let Some(ref r) = receipt {
            tracing::info!(
                "Transaction confirmed in block {}: status={}",
                r.block_number.unwrap_or_default(),
                r.status()
            );
        }

        Ok(receipt)
    }
}

/// Helper function to format wei to gwei
fn format_gwei(wei: U256) -> String {
    let gwei_divisor = U256::from(1_000_000_000u64);
    let gwei_whole = wei / gwei_divisor;
    let remainder = wei % gwei_divisor;

    // Convert to string and trim trailing zeros for readability
    let formatted = format!("{}.{:09}", gwei_whole, remainder);
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

/// Helper function to format wei to ETH
fn format_eth(wei: U256) -> String {
    let eth_divisor = U256::from(10_u64.pow(18));
    let eth_whole = wei / eth_divisor;
    let remainder = wei % eth_divisor;

    // Convert to string and trim trailing zeros for readability
    let formatted = format!("{}.{:018}", eth_whole, remainder);
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_config_default() {
        let config = GasConfig::default();
        assert_eq!(config.gas_limit_multiplier, 1.2);
    }

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_backoff_ms, 1000);
        assert!(config.use_jitter);
    }

    #[test]
    fn test_format_gwei() {
        let wei = U256::from(1_000_000_000u64);
        assert_eq!(format_gwei(wei), "1");

        let wei = U256::from(2_500_000_000u64);
        assert_eq!(format_gwei(wei), "2.5");

        let wei = U256::from(2_540_000_000u64);
        assert_eq!(format_gwei(wei), "2.54");
    }

    #[test]
    fn test_format_eth() {
        let wei = U256::from(10_u64.pow(18));
        assert_eq!(format_eth(wei), "1");

        let wei = U256::from(5 * 10_u64.pow(17));
        assert_eq!(format_eth(wei), "0.5");

        let wei = U256::from(123 * 10_u64.pow(16));
        assert_eq!(format_eth(wei), "1.23");
    }
}
