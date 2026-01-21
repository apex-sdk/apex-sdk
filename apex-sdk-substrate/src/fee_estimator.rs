//! Dynamic fee estimation for Substrate transactions
//!
//! This module provides comprehensive fee estimation capabilities including:
//! - Weight-based dynamic fee calculation using runtime metadata
//! - Network congestion monitoring and analysis
//! - Configurable fee strategies (Fast, Normal, Slow)
//! - Fee estimation accuracy metrics and tracking
//! - Integration with TransactionPayment runtime API

use crate::{Error, Result};
use parity_scale_codec::{Decode, Encode};
use std::collections::VecDeque;
use std::sync::Arc;
use subxt::{OnlineClient, PolkadotConfig};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Fee strategy for transaction prioritization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FeeStrategy {
    /// Fast confirmation with higher fees (1.5x multiplier)
    Fast,
    /// Normal confirmation with standard fees (1.2x multiplier)
    #[default]
    Normal,
    /// Slow confirmation with lower fees (1.0x multiplier)
    Slow,
}

impl FeeStrategy {
    /// Get the fee multiplier for this strategy
    pub fn multiplier(&self) -> f64 {
        match self {
            FeeStrategy::Fast => 1.5,
            FeeStrategy::Normal => 1.2,
            FeeStrategy::Slow => 1.0,
        }
    }

    /// Get the tip amount for this strategy (in Planck)
    pub fn tip(&self) -> u128 {
        match self {
            FeeStrategy::Fast => 1_000_000, // 0.001 DOT tip
            FeeStrategy::Normal => 100_000, // 0.0001 DOT tip
            FeeStrategy::Slow => 0,         // No tip
        }
    }

    /// Get a human-readable description
    pub fn description(&self) -> &str {
        match self {
            FeeStrategy::Fast => "Fast: Higher fees for quicker confirmation",
            FeeStrategy::Normal => "Normal: Standard fees with typical confirmation time",
            FeeStrategy::Slow => "Slow: Lower fees with longer confirmation time",
        }
    }
}

/// Network congestion level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionLevel {
    /// Low congestion - blocks are not full
    Low,
    /// Medium congestion - blocks are moderately full
    Medium,
    /// High congestion - blocks are near capacity
    High,
}

/// Network congestion information
#[derive(Debug, Clone)]
pub struct NetworkCongestion {
    /// Current congestion level
    pub level: CongestionLevel,
    /// Average block fullness (0.0 to 1.0)
    pub avg_block_fullness: f64,
    /// Recent average fee (in Planck)
    pub avg_fee: u128,
    /// Number of blocks analyzed
    pub blocks_analyzed: u32,
    /// Timestamp of last analysis
    pub last_updated: std::time::SystemTime,
}

impl NetworkCongestion {
    /// Create a new network congestion snapshot
    pub fn new(avg_block_fullness: f64, avg_fee: u128, blocks_analyzed: u32) -> Self {
        let level = if avg_block_fullness > 0.8 {
            CongestionLevel::High
        } else if avg_block_fullness > 0.5 {
            CongestionLevel::Medium
        } else {
            CongestionLevel::Low
        };

        Self {
            level,
            avg_block_fullness,
            avg_fee,
            blocks_analyzed,
            last_updated: std::time::SystemTime::now(),
        }
    }

    /// Get the congestion multiplier to apply to fees
    pub fn multiplier(&self) -> f64 {
        match self.level {
            CongestionLevel::Low => 1.0,
            CongestionLevel::Medium => 1.1,
            CongestionLevel::High => 1.3,
        }
    }
}

impl Default for NetworkCongestion {
    fn default() -> Self {
        Self {
            level: CongestionLevel::Low,
            avg_block_fullness: 0.0,
            avg_fee: 0,
            blocks_analyzed: 0,
            last_updated: std::time::SystemTime::now(),
        }
    }
}

/// Fee estimation result with detailed breakdown
#[derive(Debug, Clone)]
pub struct FeeEstimate {
    /// Total estimated fee (in Planck)
    pub total_fee: u128,
    /// Base fee from runtime
    pub base_fee: u128,
    /// Length fee component
    pub length_fee: u128,
    /// Weight fee component
    pub weight_fee: u128,
    /// Tip amount
    pub tip: u128,
    /// Fee strategy used
    pub strategy: FeeStrategy,
    /// Network congestion at time of estimation
    pub congestion: NetworkCongestion,
    /// Estimated transaction weight
    pub weight: Option<Weight>,
}

impl FeeEstimate {
    /// Create a new fee estimate
    pub fn new(
        base_fee: u128,
        length_fee: u128,
        weight_fee: u128,
        tip: u128,
        strategy: FeeStrategy,
        congestion: NetworkCongestion,
        weight: Option<Weight>,
    ) -> Self {
        let total_fee = base_fee + length_fee + weight_fee + tip;
        Self {
            total_fee,
            base_fee,
            length_fee,
            weight_fee,
            tip,
            strategy,
            congestion,
            weight,
        }
    }
}

/// Transaction weight information
#[derive(Debug, Clone, Copy)]
pub struct Weight {
    /// Reference time (computational weight)
    pub ref_time: u64,
    /// Proof size (storage proof weight)
    pub proof_size: u64,
}

impl Weight {
    /// Create a new weight
    pub fn new(ref_time: u64, proof_size: u64) -> Self {
        Self {
            ref_time,
            proof_size,
        }
    }

    /// Create from a combined weight value
    pub fn from_parts(ref_time: u64, proof_size: u64) -> Self {
        Self::new(ref_time, proof_size)
    }
}

/// Runtime dispatch info from TransactionPaymentApi
#[derive(Debug, Clone, Decode, Encode)]
pub struct RuntimeDispatchInfo {
    /// Weight of the extrinsic
    pub weight: WeightV2,
    /// Class of the dispatch
    pub class: DispatchClass,
    /// Partial fee (does not include tip)
    pub partial_fee: u128,
}

/// Weight V2 structure (matches sp_weights::Weight)
#[derive(Debug, Clone, Decode, Encode)]
pub struct WeightV2 {
    /// Reference time component
    pub ref_time: u64,
    /// Proof size component
    pub proof_size: u64,
}

/// Dispatch class
#[derive(Debug, Clone, Decode, Encode)]
pub enum DispatchClass {
    /// Normal dispatch
    Normal,
    /// Operational dispatch
    Operational,
    /// Mandatory dispatch
    Mandatory,
}

/// Fee estimation accuracy metric
#[derive(Debug, Clone)]
pub struct FeeAccuracyMetric {
    /// Estimated fee
    pub estimated: u128,
    /// Actual fee paid
    pub actual: u128,
    /// Absolute error
    pub absolute_error: i128,
    /// Percentage error
    pub percentage_error: f64,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

impl FeeAccuracyMetric {
    /// Create a new fee accuracy metric
    pub fn new(estimated: u128, actual: u128) -> Self {
        let absolute_error = estimated as i128 - actual as i128;
        let percentage_error = if actual > 0 {
            (absolute_error as f64 / actual as f64) * 100.0
        } else {
            0.0
        };

        Self {
            estimated,
            actual,
            absolute_error,
            percentage_error,
            timestamp: std::time::SystemTime::now(),
        }
    }
}

/// Fee estimation accuracy statistics
#[derive(Debug, Clone)]
pub struct FeeAccuracyStats {
    /// Number of samples
    pub sample_count: usize,
    /// Average absolute error
    pub avg_absolute_error: f64,
    /// Average percentage error
    pub avg_percentage_error: f64,
    /// Maximum percentage error
    pub max_percentage_error: f64,
    /// Minimum percentage error
    pub min_percentage_error: f64,
}

/// Dynamic fee estimator with dynamic calculation
pub struct DynamicFeeEstimator {
    client: OnlineClient<PolkadotConfig>,
    congestion: Arc<RwLock<NetworkCongestion>>,
    accuracy_metrics: Arc<RwLock<VecDeque<FeeAccuracyMetric>>>,
    max_metrics: usize,
    congestion_update_interval: std::time::Duration,
}

impl DynamicFeeEstimator {
    /// Create a new dynamic fee estimator
    pub fn new(client: OnlineClient<PolkadotConfig>) -> Self {
        Self {
            client,
            congestion: Arc::new(RwLock::new(NetworkCongestion::default())),
            accuracy_metrics: Arc::new(RwLock::new(VecDeque::new())),
            max_metrics: 1000,
            congestion_update_interval: std::time::Duration::from_secs(30),
        }
    }

    /// Create with custom configuration
    pub fn with_config(
        client: OnlineClient<PolkadotConfig>,
        max_metrics: usize,
        congestion_update_interval: std::time::Duration,
    ) -> Self {
        Self {
            client,
            congestion: Arc::new(RwLock::new(NetworkCongestion::default())),
            accuracy_metrics: Arc::new(RwLock::new(VecDeque::new())),
            max_metrics,
            congestion_update_interval,
        }
    }

    /// Estimate fee for a transaction with detailed breakdown
    pub async fn estimate_fee(
        &self,
        extrinsic_bytes: &[u8],
        strategy: FeeStrategy,
    ) -> Result<FeeEstimate> {
        debug!(
            "Estimating fee for {} byte extrinsic with {:?} strategy",
            extrinsic_bytes.len(),
            strategy
        );

        self.update_congestion_if_needed().await?;

        let congestion = self.congestion.read().await.clone();

        let dispatch_info = match self.query_fee_details(extrinsic_bytes).await {
            Ok(info) => {
                debug!(
                    "Got fee details from runtime: partial_fee={}, weight=({}, {})",
                    info.partial_fee, info.weight.ref_time, info.weight.proof_size
                );
                Some(info)
            }
            Err(e) => {
                warn!("Failed to query runtime fee details: {}, using fallback", e);
                None
            }
        };

        let (base_fee, weight_opt) = if let Some(info) = dispatch_info {
            let weight = Weight::from_parts(info.weight.ref_time, info.weight.proof_size);
            (info.partial_fee, Some(weight))
        } else {
            (self.calculate_fallback_fee(extrinsic_bytes), None)
        };

        let length_fee = (extrinsic_bytes.len() as u128) * 1_000;
        let weight_fee = if let Some(weight) = weight_opt {
            (weight.ref_time as u128) / 1_000_000
        } else {
            0
        };

        let strategy_multiplier = strategy.multiplier();
        let congestion_multiplier = congestion.multiplier();
        let combined_multiplier = strategy_multiplier * congestion_multiplier;

        let adjusted_base = (base_fee as f64 * combined_multiplier) as u128;
        let tip = strategy.tip();

        let estimate = FeeEstimate::new(
            adjusted_base,
            length_fee,
            weight_fee,
            tip,
            strategy,
            congestion,
            weight_opt,
        );

        debug!(
            "Fee estimate: total={}, base={}, strategy_mult={}, congestion_mult={}",
            estimate.total_fee, adjusted_base, strategy_multiplier, congestion_multiplier
        );

        Ok(estimate)
    }

    /// Query fee details from runtime
    async fn query_fee_details(&self, extrinsic_bytes: &[u8]) -> Result<RuntimeDispatchInfo> {
        let length = extrinsic_bytes.len() as u32;
        let call_data = (extrinsic_bytes, length).encode();

        let result = self
            .client
            .runtime_api()
            .at_latest()
            .await
            .map_err(|e| Error::Connection(format!("Failed to get latest block: {}", e)))?
            .call_raw("TransactionPaymentApi_query_info", Some(&call_data))
            .await
            .map_err(|e| Error::Transaction(format!("Failed to query fee info: {}", e)))?;

        RuntimeDispatchInfo::decode(&mut &result[..])
            .map_err(|e| Error::Transaction(format!("Failed to decode dispatch info: {}", e)))
    }

    /// Calculate fallback fee when runtime query fails
    fn calculate_fallback_fee(&self, extrinsic_bytes: &[u8]) -> u128 {
        let base_fee = 100_000u128;
        let per_byte_fee = 1_000u128;
        let size_fee = (extrinsic_bytes.len() as u128) * per_byte_fee;

        let weight_estimate = if extrinsic_bytes.len() > 200 {
            500_000u128
        } else if extrinsic_bytes.len() > 100 {
            200_000u128
        } else {
            100_000u128
        };

        base_fee + size_fee + weight_estimate
    }

    /// Update network congestion information
    pub async fn update_congestion(&self) -> Result<()> {
        debug!("Updating network congestion information");

        let blocks_to_analyze = 10u32;
        let latest_block = self
            .client
            .blocks()
            .at_latest()
            .await
            .map_err(|e| Error::Connection(format!("Failed to get latest block: {}", e)))?;

        let latest_number = latest_block.number();
        let mut total_fullness = 0.0f64;
        let mut total_fees = 0u128;
        let mut blocks_analyzed = 0u32;

        for offset in 0..blocks_to_analyze {
            let block_number = latest_number.saturating_sub(offset);
            match self.analyze_block_congestion(block_number).await {
                Ok((fullness, avg_fee)) => {
                    total_fullness += fullness;
                    total_fees += avg_fee;
                    blocks_analyzed += 1;
                }
                Err(e) => {
                    warn!("Failed to analyze block {}: {}", block_number, e);
                }
            }
        }

        if blocks_analyzed > 0 {
            let avg_fullness = total_fullness / blocks_analyzed as f64;
            let avg_fee = total_fees / blocks_analyzed as u128;

            let congestion = NetworkCongestion::new(avg_fullness, avg_fee, blocks_analyzed);
            info!(
                "Network congestion updated: level={:?}, fullness={:.2}%, avg_fee={}",
                congestion.level,
                avg_fullness * 100.0,
                avg_fee
            );

            *self.congestion.write().await = congestion;
        }

        Ok(())
    }

    /// Analyze a single block for congestion metrics
    async fn analyze_block_congestion(&self, block_number: u32) -> Result<(f64, u128)> {
        let latest = self
            .client
            .blocks()
            .at_latest()
            .await
            .map_err(|e| Error::Connection(format!("Failed to get latest block: {}", e)))?;

        let target_number = block_number;
        let latest_number = latest.number();

        if target_number > latest_number {
            return Err(Error::Connection(format!(
                "Block {} is in the future (latest: {})",
                target_number, latest_number
            )));
        }

        let offset = latest_number.saturating_sub(target_number);
        let mut current_block = latest;

        for _ in 0..offset {
            let parent_hash = current_block.header().parent_hash;
            current_block =
                self.client.blocks().at(parent_hash).await.map_err(|e| {
                    Error::Connection(format!("Failed to navigate to block: {}", e))
                })?;
        }

        let extrinsics = current_block
            .extrinsics()
            .await
            .map_err(|e| Error::Transaction(format!("Failed to get extrinsics: {}", e)))?;

        let extrinsic_count = extrinsics.len();
        let max_block_weight = 2_000_000_000_000u64;
        let mut total_weight = 0u64;
        let mut total_fees = 0u128;
        let mut fee_count = 0u32;

        for ext in extrinsics.iter() {
            match ext.events().await {
                Ok(events) => {
                    for event_result in events.iter() {
                        match event_result {
                            Ok(event) => {
                                if event.pallet_name() == "TransactionPayment"
                                    && event.variant_name() == "TransactionFeePaid"
                                {
                                    let fee_event = event.field_bytes();
                                    if fee_event.len() >= 16 {
                                        let fee_bytes = &fee_event[fee_event.len() - 16..];
                                        if fee_bytes.len() == 16 {
                                            let mut fee_array = [0u8; 16];
                                            fee_array.copy_from_slice(fee_bytes);
                                            let fee = u128::from_le_bytes(fee_array);
                                            total_fees += fee;
                                            fee_count += 1;
                                        }
                                    }
                                }
                            }
                            Err(_) => continue,
                        }
                    }
                }
                Err(_) => continue,
            }

            total_weight += 200_000_000;
        }

        let block_fullness = (total_weight as f64) / (max_block_weight as f64);
        let avg_fee = if fee_count > 0 {
            total_fees / fee_count as u128
        } else {
            100_000u128
        };

        debug!(
            "Block {} analysis: {} extrinsics, fullness={:.2}%, avg_fee={}",
            block_number,
            extrinsic_count,
            block_fullness * 100.0,
            avg_fee
        );

        Ok((block_fullness.min(1.0), avg_fee))
    }

    /// Update congestion if enough time has passed
    async fn update_congestion_if_needed(&self) -> Result<()> {
        let should_update = {
            let congestion = self.congestion.read().await;
            match congestion.last_updated.elapsed() {
                Ok(elapsed) => elapsed >= self.congestion_update_interval,
                Err(_) => true,
            }
        };

        if should_update {
            self.update_congestion().await?;
        }

        Ok(())
    }

    /// Get current network congestion
    pub async fn get_congestion(&self) -> NetworkCongestion {
        self.congestion.read().await.clone()
    }

    /// Record actual fee for accuracy tracking
    pub async fn record_actual_fee(&self, estimated: u128, actual: u128) {
        let metric = FeeAccuracyMetric::new(estimated, actual);
        debug!(
            "Recording fee accuracy: estimated={}, actual={}, error={:.2}%",
            estimated, actual, metric.percentage_error
        );

        let mut metrics = self.accuracy_metrics.write().await;
        metrics.push_back(metric);

        while metrics.len() > self.max_metrics {
            metrics.pop_front();
        }
    }

    /// Get fee estimation accuracy statistics
    pub async fn get_accuracy_stats(&self) -> Option<FeeAccuracyStats> {
        let metrics = self.accuracy_metrics.read().await;

        if metrics.is_empty() {
            return None;
        }

        let mut total_abs_error = 0.0;
        let mut total_pct_error = 0.0;
        let mut max_pct_error = f64::MIN;
        let mut min_pct_error = f64::MAX;

        for metric in metrics.iter() {
            total_abs_error += metric.absolute_error.abs() as f64;
            total_pct_error += metric.percentage_error.abs();
            max_pct_error = max_pct_error.max(metric.percentage_error.abs());
            min_pct_error = min_pct_error.min(metric.percentage_error.abs());
        }

        let count = metrics.len();

        Some(FeeAccuracyStats {
            sample_count: count,
            avg_absolute_error: total_abs_error / count as f64,
            avg_percentage_error: total_pct_error / count as f64,
            max_percentage_error: max_pct_error,
            min_percentage_error: min_pct_error,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_strategy_multipliers() {
        assert_eq!(FeeStrategy::Fast.multiplier(), 1.5);
        assert_eq!(FeeStrategy::Normal.multiplier(), 1.2);
        assert_eq!(FeeStrategy::Slow.multiplier(), 1.0);
    }

    #[test]
    fn test_fee_strategy_tips() {
        assert_eq!(FeeStrategy::Fast.tip(), 1_000_000);
        assert_eq!(FeeStrategy::Normal.tip(), 100_000);
        assert_eq!(FeeStrategy::Slow.tip(), 0);
    }

    #[test]
    fn test_fee_strategy_default() {
        assert_eq!(FeeStrategy::default(), FeeStrategy::Normal);
    }

    #[test]
    fn test_congestion_level_detection() {
        let low = NetworkCongestion::new(0.3, 100_000, 10);
        assert_eq!(low.level, CongestionLevel::Low);

        let medium = NetworkCongestion::new(0.6, 200_000, 10);
        assert_eq!(medium.level, CongestionLevel::Medium);

        let high = NetworkCongestion::new(0.9, 500_000, 10);
        assert_eq!(high.level, CongestionLevel::High);
    }

    #[test]
    fn test_congestion_multipliers() {
        let low = NetworkCongestion::new(0.3, 100_000, 10);
        assert_eq!(low.multiplier(), 1.0);

        let medium = NetworkCongestion::new(0.6, 200_000, 10);
        assert_eq!(medium.multiplier(), 1.1);

        let high = NetworkCongestion::new(0.9, 500_000, 10);
        assert_eq!(high.multiplier(), 1.3);
    }

    #[test]
    fn test_weight_creation() {
        let weight = Weight::new(1_000_000, 5_000);
        assert_eq!(weight.ref_time, 1_000_000);
        assert_eq!(weight.proof_size, 5_000);

        let weight2 = Weight::from_parts(2_000_000, 10_000);
        assert_eq!(weight2.ref_time, 2_000_000);
        assert_eq!(weight2.proof_size, 10_000);
    }

    #[test]
    fn test_fee_accuracy_metric() {
        let metric = FeeAccuracyMetric::new(1_200_000, 1_000_000);
        assert_eq!(metric.estimated, 1_200_000);
        assert_eq!(metric.actual, 1_000_000);
        assert_eq!(metric.absolute_error, 200_000);
        assert!((metric.percentage_error - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_fee_accuracy_metric_exact() {
        let metric = FeeAccuracyMetric::new(1_000_000, 1_000_000);
        assert_eq!(metric.absolute_error, 0);
        assert_eq!(metric.percentage_error, 0.0);
    }

    #[test]
    fn test_fee_accuracy_metric_underestimate() {
        let metric = FeeAccuracyMetric::new(800_000, 1_000_000);
        assert_eq!(metric.absolute_error, -200_000);
        assert!((metric.percentage_error + 20.0).abs() < 0.01);
    }

    #[test]
    fn test_fee_estimate_creation() {
        let congestion = NetworkCongestion::default();
        let weight = Some(Weight::new(1_000_000, 5_000));
        let estimate = FeeEstimate::new(
            100_000,
            50_000,
            200_000,
            10_000,
            FeeStrategy::Normal,
            congestion,
            weight,
        );

        assert_eq!(estimate.total_fee, 360_000);
        assert_eq!(estimate.base_fee, 100_000);
        assert_eq!(estimate.length_fee, 50_000);
        assert_eq!(estimate.weight_fee, 200_000);
        assert_eq!(estimate.tip, 10_000);
    }
}
