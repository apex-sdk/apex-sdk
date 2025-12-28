#!/bin/bash

# Extract Benchmark Results for Whitepaper
# Usage: ./scripts/extract_benchmark_results.sh

set -e

CRITERION_DIR="target/criterion"
OUTPUT_FILE="BENCHMARK_RESULTS.md"

echo "# Apex SDK Benchmark Results" > $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
echo "**Generated:** $(date)" >> $OUTPUT_FILE
echo "**Rust Version:** $(rustc --version)" >> $OUTPUT_FILE
echo "**Criterion Version:** 0.8.1" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE

# Function to extract mean time from estimates.json
extract_mean() {
    local benchmark_path=$1
    local estimates_file="${CRITERION_DIR}/${benchmark_path}/new/estimates.json"

    if [ -f "$estimates_file" ]; then
        # Extract mean in nanoseconds
        local mean_ns=$(jq '.mean.point_estimate' "$estimates_file" 2>/dev/null || echo "N/A")

        if [ "$mean_ns" != "N/A" ]; then
            # Convert to appropriate unit
            local mean_float=$(echo "$mean_ns" | awk '{printf "%.2f", $1}')

            if (( $(echo "$mean_ns < 1000" | bc -l) )); then
                echo "${mean_float} ns"
            elif (( $(echo "$mean_ns < 1000000" | bc -l) )); then
                local mean_us=$(echo "$mean_ns / 1000" | bc -l | awk '{printf "%.2f", $1}')
                echo "${mean_us} µs"
            else
                local mean_ms=$(echo "$mean_ns / 1000000" | bc -l | awk '{printf "%.2f", $1}')
                echo "${mean_ms} ms"
            fi
        else
            echo "N/A"
        fi
    else
        echo "N/A"
    fi
}

# Function to add section
add_section() {
    local title=$1
    echo "" >> $OUTPUT_FILE
    echo "## $title" >> $OUTPUT_FILE
    echo "" >> $OUTPUT_FILE
}

# Function to add metric
add_metric() {
    local name=$1
    local value=$2
    echo "- **${name}**: ${value}" >> $OUTPUT_FILE
}

# Type System Benchmarks
add_section "Type System Performance (apex-sdk-types)"

echo "### Address Validation" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "EVM Unchecked Creation" "$(extract_mean 'evm_address_validation/unchecked_creation')"
add_metric "EVM Checked Creation (EIP-55)" "$(extract_mean 'evm_address_validation/checked_creation')"
add_metric "EVM Single Validation" "$(extract_mean 'evm_address_validation/single_validation')"
add_metric "Substrate Unchecked Creation" "$(extract_mean 'substrate_address_validation/unchecked_creation')"
add_metric "Substrate Checked Creation (SS58)" "$(extract_mean 'substrate_address_validation/checked_creation')"
add_metric "Substrate Network-Specific Validation" "$(extract_mean 'substrate_address_validation/network_specific_validation')"

echo "" >> $OUTPUT_FILE
echo "### Chain Operations" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Chain Type Determination" "$(extract_mean 'chain_operations/chain_type_determination')"
add_metric "Default RPC Endpoint" "$(extract_mean 'chain_operations/default_rpc_endpoint')"
add_metric "Chain ID Retrieval" "$(extract_mean 'chain_operations/chain_id_retrieval')"
add_metric "Smart Contract Support Check" "$(extract_mean 'chain_operations/smart_contract_support')"

echo "" >> $OUTPUT_FILE
echo "### Serialization" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Serialize EVM Address" "$(extract_mean 'serialization/serialize_evm_address')"
add_metric "Serialize Substrate Address" "$(extract_mean 'serialization/serialize_substrate_address')"
add_metric "Serialize Chain" "$(extract_mean 'serialization/serialize_chain')"
add_metric "Deserialize EVM Address" "$(extract_mean 'serialization/deserialize_evm_address')"

# Substrate Adapter Benchmarks
add_section "Substrate Adapter Performance (apex-sdk-substrate)"

echo "### Configuration Creation" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Polkadot Config" "$(extract_mean 'config_creation/polkadot_config')"
add_metric "Pool Config" "$(extract_mean 'config_creation/pool_config')"
add_metric "Cache Config" "$(extract_mean 'config_creation/cache_config')"

echo "" >> $OUTPUT_FILE
echo "### Cache Operations" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Cache Insert" "$(extract_mean 'cache_operations/cache_insert')"
add_metric "Cache Hit" "$(extract_mean 'cache_operations/cache_hit')"
add_metric "Cache Miss" "$(extract_mean 'cache_operations/cache_miss')"
add_metric "Cache Clear" "$(extract_mean 'cache_operations/cache_clear')"

echo "" >> $OUTPUT_FILE
echo "### Wallet Operations" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Create SR25519 Wallet" "$(extract_mean 'wallet_operations/create_sr25519_wallet')"
add_metric "Create ED25519 Wallet" "$(extract_mean 'wallet_operations/create_ed25519_wallet')"
add_metric "Wallet from Seed (SR25519)" "$(extract_mean 'wallet_operations/wallet_from_seed_sr25519')"
add_metric "Derive Address" "$(extract_mean 'wallet_operations/derive_address')"

echo "" >> $OUTPUT_FILE
echo "### Metrics" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Record RPC Call" "$(extract_mean 'metrics_operations/record_rpc_call')"
add_metric "Increment Connections" "$(extract_mean 'metrics_operations/increment_connections')"
add_metric "Record Cache Hit" "$(extract_mean 'metrics_operations/record_cache_hit')"
add_metric "Get Snapshot" "$(extract_mean 'metrics_operations/get_snapshot')"

# EVM Adapter Benchmarks
add_section "EVM Adapter Performance (apex-sdk-evm)"

echo "### Address Operations" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Parse ETH Address" "$(extract_mean 'address_parsing/parse_eth_address')"
add_metric "Checksum Validation" "$(extract_mean 'address_parsing/checksum_validation')"
add_metric "Parse Multiple Addresses" "$(extract_mean 'address_parsing/parse_multiple_addresses')"

echo "" >> $OUTPUT_FILE
echo "### U256 Arithmetic" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "U256 from u64" "$(extract_mean 'u256_operations/u256_from_u64')"
add_metric "U256 Addition" "$(extract_mean 'u256_operations/u256_addition')"
add_metric "U256 Subtraction" "$(extract_mean 'u256_operations/u256_subtraction')"
add_metric "U256 Multiplication" "$(extract_mean 'u256_operations/u256_multiplication')"
add_metric "U256 Division" "$(extract_mean 'u256_operations/u256_division')"

echo "" >> $OUTPUT_FILE
echo "### Gas Calculations" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Transfer Cost Calculation" "$(extract_mean 'gas_calculations/transfer_cost_calculation')"
add_metric "Gas Price Comparison" "$(extract_mean 'gas_calculations/gas_price_comparison')"
add_metric "EIP-1559 Max Fee Calculation" "$(extract_mean 'gas_calculations/eip1559_max_fee_calculation')"

echo "" >> $OUTPUT_FILE
echo "### Value Conversions" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "ETH to Wei" "$(extract_mean 'value_conversions/eth_to_wei')"
add_metric "Wei to ETH" "$(extract_mean 'value_conversions/wei_to_eth')"
add_metric "Gwei to Wei" "$(extract_mean 'value_conversions/gwei_to_wei')"
add_metric "Wei to Gwei" "$(extract_mean 'value_conversions/wei_to_gwei')"

# Cross-Chain Benchmarks
add_section "Cross-Chain Performance (apex-sdk)"

echo "### Transaction Creation" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Create Substrate Transfer" "$(extract_mean 'transaction_creation/create_substrate_transfer')"
add_metric "Create EVM Transfer" "$(extract_mean 'transaction_creation/create_evm_transfer')"

echo "" >> $OUTPUT_FILE
echo "### Cross-Chain Operations" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Chain Type Detection" "$(extract_mean 'cross_chain_operations/chain_type_detection')"
add_metric "Multi-Chain Transaction Creation" "$(extract_mean 'cross_chain_operations/multi_chain_transaction_creation')"
add_metric "Cross-Chain Address Handling" "$(extract_mean 'cross_chain_operations/cross_chain_address_handling')"

echo "" >> $OUTPUT_FILE
echo "### Decimal Conversions" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "DOT to Planck" "$(extract_mean 'decimal_conversions/dot_to_planck')"
add_metric "KSM to Planck" "$(extract_mean 'decimal_conversions/ksm_to_planck')"
add_metric "ETH to Wei" "$(extract_mean 'decimal_conversions/eth_to_wei')"
add_metric "Planck to DOT" "$(extract_mean 'decimal_conversions/planck_to_dot')"
add_metric "Wei to ETH" "$(extract_mean 'decimal_conversions/wei_to_eth')"

echo "" >> $OUTPUT_FILE
echo "### SDK Initialization" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
add_metric "Create Substrate SDK" "$(extract_mean 'sdk_initialization/create_substrate_sdk')"
add_metric "Create EVM SDK" "$(extract_mean 'sdk_initialization/create_evm_sdk')"
add_metric "Create Multi-Chain SDK" "$(extract_mean 'sdk_initialization/create_multi_chain_sdk')"

echo "" >> $OUTPUT_FILE
echo "---" >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
echo "**Note:** All measurements are mean values from Criterion benchmarks with statistical analysis." >> $OUTPUT_FILE
echo "" >> $OUTPUT_FILE
echo "For detailed results and graphs, see:" >> $OUTPUT_FILE
echo "- HTML Reports: \`target/criterion/reports/index.html\`" >> $OUTPUT_FILE
echo "- Raw Data: \`target/criterion/*/new/estimates.json\`" >> $OUTPUT_FILE

echo ""
echo "✅ Benchmark results extracted to: $OUTPUT_FILE"
echo ""
echo "To view HTML reports:"
echo "  open target/criterion/reports/index.html"
echo ""
