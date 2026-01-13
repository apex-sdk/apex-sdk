//! EVM Contract Call Example - REAL Implementation
//!
//! This example demonstrates REAL EVM contract interactions using Alloy's sol! macro
//! for type-safe ABI encoding. Unlike the simulated examples, this one can execute
//! actual transactions on Sepolia testnet.
//!
//! ## What This Example Does
//! - Queries an ERC20 token balance (read-only, no gas)
//! - Queries total supply (read-only)
//! - Demonstrates how to prepare write transactions (commented out to prevent accidental execution)
//!
//! ## Running This Example
//! ```bash
//! cargo run --bin evm-contract-call
//! ```
//!
//! ## To Execute Write Transactions
//! Uncomment the write transaction section and provide:
//! - A private key via environment variable: PRIVATE_KEY=0x...
//! - Ensure you have Sepolia ETH for gas

use alloy::sol;
use alloy_primitives::{Address as EthAddress, U256};
use anyhow::Result;
use apex_sdk::prelude::*;

// Define the ERC20 interface using Alloy's sol! macro
// This generates type-safe Rust bindings for the contract
sol! {
    #[sol(rpc)]
    interface IERC20 {
        function balanceOf(address account) external view returns (uint256);
        function totalSupply() external view returns (uint256);
        function name() external view returns (string memory);
        function symbol() external view returns (string memory);
        function decimals() external view returns (uint8);
        function transfer(address to, uint256 amount) external returns (bool);
    }
}

// Define Uniswap V2 Router interface for DEX interactions
sol! {
    #[sol(rpc)]
    interface IUniswapV2Router02 {
        function swapExactTokensForTokens(
            uint256 amountIn,
            uint256 amountOutMin,
            address[] calldata path,
            address to,
            uint256 deadline
        ) external returns (uint256[] memory amounts);

        function getAmountsOut(
            uint256 amountIn,
            address[] calldata path
        ) external view returns (uint256[] memory amounts);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    println!("=== Real EVM Contract Interaction Example ===\n");
    println!("This example demonstrates ACTUAL contract calls using Alloy's sol! macro\n");

    // Initialize SDK with Sepolia testnet endpoint
    let sdk = ApexSDK::builder()
        .with_evm_endpoint("https://eth-sepolia.g.alchemy.com/v2/demo")
        .build()
        .await?;

    println!("✓ SDK initialized with Sepolia testnet\n");

    // Contract addresses on Sepolia
    let weth_address: EthAddress = "0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14"
        .parse()
        .expect("Invalid WETH address");

    let usdc_address: EthAddress = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238"
        .parse()
        .expect("Invalid USDC address");

    println!("Contract Addresses:");
    println!("  WETH: {:?}", weth_address);
    println!("  USDC: {:?}\n", usdc_address);

    // Example wallet address for demonstration
    let demo_address: EthAddress = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
        .parse()
        .expect("Invalid address");

    println!("Query Address: {:?}\n", demo_address);

    // ================================================================
    // STEP 1: Query ERC20 Token Metadata (Read-Only)
    // ================================================================
    println!("Step 1: Query WETH Token Metadata");
    println!("─────────────────────────────────────");

    // Get the EVM adapter to interact with contracts
    let evm_adapter = sdk.evm()?;
    let provider = evm_adapter.provider();

    // Query token name
    let name_call = IERC20::nameCall {};
    let name_data = name_call.abi_encode();

    println!("Calling name()...");
    let name_result = provider
        .inner
        .call(&alloy::rpc::types::TransactionRequest::default()
            .to(weth_address)
            .input(name_data.into()))
        .await?;

    let name_return = IERC20::nameCall::abi_decode_returns(&name_result, true)?;
    println!("  Token Name: {}", name_return._0);

    // Query token symbol
    let symbol_call = IERC20::symbolCall {};
    let symbol_data = symbol_call.abi_encode();

    println!("Calling symbol()...");
    let symbol_result = provider
        .inner
        .call(&alloy::rpc::types::TransactionRequest::default()
            .to(weth_address)
            .input(symbol_data.into()))
        .await?;

    let symbol_return = IERC20::symbolCall::abi_decode_returns(&symbol_result, true)?;
    println!("  Token Symbol: {}", symbol_return._0);

    // Query decimals
    let decimals_call = IERC20::decimalsCall {};
    let decimals_data = decimals_call.abi_encode();

    println!("Calling decimals()...");
    let decimals_result = provider
        .inner
        .call(&alloy::rpc::types::TransactionRequest::default()
            .to(weth_address)
            .input(decimals_data.into()))
        .await?;

    let decimals_return = IERC20::decimalsCall::abi_decode_returns(&decimals_result, true)?;
    println!("  Token Decimals: {}\n", decimals_return._0);

    // ================================================================
    // STEP 2: Query Token Balance (Read-Only)
    // ================================================================
    println!("Step 2: Query Token Balance");
    println!("─────────────────────────────────────");

    let balance_call = IERC20::balanceOfCall {
        account: demo_address,
    };
    let balance_data = balance_call.abi_encode();

    println!("Calling balanceOf({:?})...", demo_address);
    let balance_result = provider
        .inner
        .call(&alloy::rpc::types::TransactionRequest::default()
            .to(weth_address)
            .input(balance_data.into()))
        .await?;

    let balance_return = IERC20::balanceOfCall::abi_decode_returns(&balance_result, true)?;
    let balance = balance_return._0;

    // Format balance with decimals
    let decimals_u8 = decimals_return._0;
    let divisor = 10u128.pow(decimals_u8 as u32);
    let balance_formatted = balance.to::<u128>() as f64 / divisor as f64;

    println!("  Balance: {} {} ({} wei)", balance_formatted, symbol_return._0, balance);

    // ================================================================
    // STEP 3: Query Total Supply (Read-Only)
    // ================================================================
    println!("\nStep 3: Query Total Supply");
    println!("─────────────────────────────────────");

    let supply_call = IERC20::totalSupplyCall {};
    let supply_data = supply_call.abi_encode();

    println!("Calling totalSupply()...");
    let supply_result = provider
        .inner
        .call(&alloy::rpc::types::TransactionRequest::default()
            .to(weth_address)
            .input(supply_data.into()))
        .await?;

    let supply_return = IERC20::totalSupplyCall::abi_decode_returns(&supply_result, true)?;
    let total_supply = supply_return._0;

    let supply_formatted = total_supply.to::<u128>() as f64 / divisor as f64;
    println!("  Total Supply: {} {} ({} wei)", supply_formatted, symbol_return._0, total_supply);

    // ================================================================
    // STEP 4: Demonstrate Transaction Encoding (Not Executed)
    // ================================================================
    println!("\nStep 4: Demonstrate Transaction Encoding");
    println!("─────────────────────────────────────");
    println!("(Transaction building only - not executed)");

    // Build a transfer transaction (not executed)
    let recipient: EthAddress = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7"
        .parse()
        .expect("Invalid recipient");
    let transfer_amount = U256::from(1_000_000_000_000_000u128); // 0.001 WETH

    let transfer_call = IERC20::transferCall {
        to: recipient,
        amount: transfer_amount,
    };

    let transfer_data = transfer_call.abi_encode();

    println!("\nTransfer Transaction (NOT EXECUTED):");
    println!("  Function: transfer(address,uint256)");
    println!("  To: {:?}", recipient);
    println!("  Amount: {} WETH", transfer_amount.to::<u128>() as f64 / divisor as f64);
    println!("  Encoded Data Length: {} bytes", transfer_data.len());
    println!("  Data (first 32 bytes): 0x{}", hex::encode(&transfer_data[..32.min(transfer_data.len())]));

    // ================================================================
    // STEP 5: How to Execute Transactions
    // ================================================================
    println!("\n\nHow to Execute Real Transactions:");
    println!("─────────────────────────────────────");
    println!("To execute write transactions, you need:");
    println!("1. A wallet with a private key");
    println!("2. Sepolia ETH for gas fees");
    println!("3. Uncomment the code below and run:\n");

    println!("```rust");
    println!("// Create wallet from private key");
    println!("let private_key = std::env::var(\"PRIVATE_KEY\")?;");
    println!("let wallet = apex_sdk_evm::wallet::Wallet::from_private_key(&private_key)?;");
    println!();
    println!("// Build SDK with wallet");
    println!("let sdk = ApexSDK::builder()");
    println!("    .with_evm_endpoint(\"https://eth-sepolia.g.alchemy.com/v2/demo\")");
    println!("    .with_evm_wallet(wallet)");
    println!("    .build()");
    println!("    .await?;");
    println!();
    println!("// Build and execute transaction");
    println!("let tx = sdk.transaction()");
    println!("    .from_evm_address(&wallet.address())");
    println!("    .to_evm_address(\"{:?}\")", weth_address);
    println!("    .amount(0) // No ETH sent");
    println!("    .with_data(transfer_data)");
    println!("    .build()?;");
    println!();
    println!("let result = sdk.execute(tx).await?;");
    println!("println!(\"Transaction hash: {{}}\", result.source_tx_hash);");
    println!("```\n");

    // ================================================================
    // Summary
    // ================================================================
    println!("\nSummary:");
    println!("─────────────────────────────────────");
    println!("✓ Connected to Sepolia testnet");
    println!("✓ Queried ERC20 token metadata");
    println!("✓ Queried token balance (read-only)");
    println!("✓ Queried total supply (read-only)");
    println!("✓ Demonstrated transaction encoding");
    println!();
    println!("Key Advantages of Alloy's sol! macro:");
    println!("  Type safety - Compile-time checking of all parameters");
    println!("  ABI encoding/decoding - Automatic and correct");
    println!("  No runtime errors - Wrong types won't compile");
    println!("  Clean API - Natural Rust syntax");
    println!();
    println!("These are REAL contract calls on Sepolia testnet!");

    Ok(())
}
