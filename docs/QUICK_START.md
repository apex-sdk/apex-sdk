# Apex SDK Quick Start Guide

Welcome to Apex SDK! This guide will help you get started with building cross-chain applications in under 10 minutes.

## Prerequisites

- Rust 1.75 or higher
- Cargo package manager
- Basic understanding of blockchain concepts

## Installation

### Option 1: Via Cargo (Recommended)

Add Apex SDK to your `Cargo.toml`:

```toml
[dependencies]
apex-sdk = "0.1.4"
tokio = { version = "1.35", features = ["full"] }
anyhow = "1.0"
```

### Option 2: Using the CLI

```bash
# Install the CLI
cargo install apex-sdk-cli

# Create a new project
apex new my-project

# Navigate to your project
cd my-project

# Build and run
cargo build
cargo run
```

## Your First Cross-Chain Transaction

Create a new file `src/main.rs`:

```rust
use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize SDK with both Substrate and EVM support
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
        .with_evm_endpoint("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY")
        .build()
        .await?;

    // Build a cross-chain transaction
    let tx = sdk
        .transaction()
        .from_substrate_account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
        .to_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
        .amount(1_000_000_000_000) // 1 DOT in Planck
        .build()?;

    // Execute the transaction
    let result = sdk.execute(tx).await?;

    println!("Transaction successful!");
    println!("Source TX: {}", result.source_tx_hash);
    if let Some(dest_tx) = result.destination_tx_hash {
        println!("Destination TX: {}", dest_tx);
    }

    Ok(())
}
```

## Key Concepts

### 1. SDK Initialization

The SDK supports three modes of operation:

- **Substrate-only**: Connect to Substrate chains (Polkadot, Kusama)
- **EVM-only**: Connect to EVM chains (Ethereum, Polygon, BSC)
- **Multi-chain**: Connect to both Substrate and EVM chains

```rust
// Substrate-only
let sdk = ApexSDK::builder()
    .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    .build()
    .await?;

// EVM-only
let sdk = ApexSDK::builder()
    .with_evm_endpoint("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY")
    .build()
    .await?;

// Multi-chain
let sdk = ApexSDK::builder()
    .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    .with_evm_endpoint("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY")
    .build()
    .await?;
```

### 2. Building Transactions

The Transaction Builder provides a fluent API:

```rust
let tx = sdk
    .transaction()
    .from_evm_address("0x...")       // Set sender
    .to_evm_address("0x...")         // Set recipient
    .amount(1_000_000_000)           // Set amount
    .with_gas_limit(21000)           // Optional: set gas limit
    .with_data(vec![1, 2, 3])        // Optional: add data
    .build()?;
```

### 3. Address Types

Apex SDK uses type-safe addresses:

```rust
// Substrate address (SS58 format)
let substrate_addr = Address::substrate("5GrwvaEF5z...");

// EVM address (hex format)
let evm_addr = Address::evm("0x742d35Cc663...");
```

### 4. Chain Support

Check which chains are supported:

```rust
if sdk.is_chain_supported(&Chain::Ethereum) {
    println!("Ethereum is supported!");
}

// Supported chains:
// - Chain::Polkadot
// - Chain::Kusama
// - Chain::Ethereum
// - Chain::BinanceSmartChain
// - Chain::Polygon
// - Chain::Avalanche
// - Chain::Moonbeam (Hybrid)
// - Chain::Astar (Hybrid)
```

### 5. Transaction Status

Query transaction status after execution:

```rust
let status = sdk
    .get_transaction_status(&Chain::Ethereum, "0x123...")
    .await?;

match status {
    TransactionStatus::Pending => println!("Transaction pending"),
    TransactionStatus::Confirmed { block_number, confirmations } => {
        println!("Confirmed at block {} with {} confirmations",
                 block_number, confirmations);
    }
    TransactionStatus::Failed { error } => {
        println!("Transaction failed: {}", error);
    }
    TransactionStatus::Unknown => println!("Status unknown"),
}
```

## Common Patterns

### Same-Chain Transfer (EVM)

```rust
let tx = sdk
    .transaction()
    .from_evm_address("0x...")
    .to_evm_address("0x...")
    .amount(1_000_000_000_000_000_000u128) // 1 ETH
    .with_gas_limit(21000)
    .build()?;

let result = sdk.execute(tx).await?;
```

### Cross-Chain Transfer (Substrate → EVM)

```rust
let tx = sdk
    .transaction()
    .from_substrate_account("5GrwvaEF...")
    .to_evm_address("0x...")
    .amount(5_000_000_000_000) // 5 DOT
    .build()?;

let result = sdk.execute(tx).await?;
// result.destination_tx_hash will contain the EVM transaction hash
```

### With Custom Data

```rust
let tx = sdk
    .transaction()
    .from_evm_address("0x...")
    .to_evm_address("0x...")
    .amount(0) // Can be zero for data-only transactions
    .with_data(b"Hello, Apex SDK!".to_vec())
    .build()?;
```

## Examples

Apex SDK includes comprehensive examples:

### Basic Transfer
```bash
cd examples/basic-transfer
cargo run
```

Demonstrates:
- SDK initialization
- Same-chain transfers
- Cross-chain transfers
- Transaction status queries

### DeFi Aggregator
```bash
cd examples/defi-aggregator
cargo run
```

Demonstrates:
- Multi-chain liquidity aggregation
- Cross-chain swaps
- Yield farming optimization

### NFT Bridge
```bash
cd examples/nft-bridge
cargo run
```

Demonstrates:
- Cross-chain NFT transfers
- Metadata synchronization
- NFT locking and minting

### DAO Governance
```bash
cd examples/dao-governance
cargo run
```

Demonstrates:
- Multi-chain voting
- Cross-chain proposal execution
- Treasury management

## Error Handling

Apex SDK uses Rust's Result type for error handling:

```rust
use apex_sdk::prelude::*;

async fn transfer() -> Result<()> {
    let sdk = ApexSDK::builder()
        .with_evm_endpoint("https://...")
        .build()
        .await?;  // Returns Error::Config if failed

    let tx = sdk
        .transaction()
        .from_evm_address("0x...")
        .to_evm_address("0x...")
        .amount(1000)
        .build()?;  // Returns Error::Transaction if invalid

    let result = sdk.execute(tx).await?;  // Returns Error::Connection if failed

    Ok(())
}
```

Common error types:
- `Error::Config`: Configuration errors
- `Error::Connection`: Network connection errors
- `Error::Transaction`: Transaction building errors
- `Error::InvalidAddress`: Invalid address format
- `Error::UnsupportedChain`: Chain not supported

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests for a specific package
cargo test --package apex-sdk

# Run with verbose output
cargo test --verbose

# Run a specific test
cargo test test_transaction_builder
```

## Next Steps

1. **Explore Examples**: Check out the `examples/` directory for more use cases
2. **Read the API Docs**: Run `cargo doc --open` to view API documentation
3. **Join the Community**:
   - GitHub: [github.com/kherldhussein/apexsdk](https://github.com/kherldhussein/apex-sdk)
   - Issues: [github.com/kherldhussein/apexsdk/issues](https://github.com/kherldhussein/apex-sdk/issues)
   - Discussions: [github.com/kherldhussein/apexsdk/discussions](https://github.com/kherldhussein/apex-sdk/discussions)
4. **Contribute**: See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines

## Production Considerations

Before deploying to production:

1. **Use Real Endpoints**: Replace test endpoints with production RPC URLs
2. **Secure Private Keys**: Never hardcode private keys in your code
3. **Implement Retry Logic**: Handle network failures gracefully
4. **Monitor Gas Prices**: Especially for EVM chains
5. **Test Thoroughly**: Use testnets before mainnet deployment
6. **Handle Edge Cases**: Account for network congestion, slippage, etc.
7. **Security Audit**: For high-value applications, consider a security audit

## Troubleshooting

### "At least one adapter must be configured"
- Ensure you call either `with_substrate_endpoint()` or `with_evm_endpoint()` before `build()`

### Connection Timeout
- Check your RPC endpoint is accessible
- Verify your internet connection
- Try a different RPC provider

### Invalid Address Format
- Substrate addresses should be in SS58 format (e.g., `5GrwvaEF...`)
- EVM addresses should be in hex format with `0x` prefix (e.g., `0x742d35Cc...`)

### Transaction Failed
- Check account has sufficient balance
- Verify gas limit is adequate (for EVM)
- Ensure recipient address is valid

## Resources

- **Documentation**: [github.com/kherldhussein/apex-sdk/docs](https://github.com/kherldhussein/apex-sdk/docs
- **API Reference**: Run `cargo doc --open`
- **Examples**: See the `examples/` directory
- **GitHub**: [github.com/kherldhussein/apex-sdk](https://github.com/kherldhussein/apex-sdk)
- **Issues**: Report bugs at [github.com/kherldhussein/apex-sdk/issues](https://github.com/kherldhussein/apex-sdk/issues)

