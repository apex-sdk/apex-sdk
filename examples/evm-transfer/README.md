# EVM Transfer Example

This example demonstrates how to execute a **ETH transfer transaction** on Ethereum Sepolia testnet using Apex SDK with wallet private key signing.

## What This Example Does

1. **Creates a wallet** from a private key with chain ID configuration
2. **Connects to Sepolia testnet** via public RPC endpoint
3. **Checks account balance** to ensure sufficient funds
4. **Builds a transaction** using the unified Apex SDK interface
5. **Signs and sends** the transaction to the network
6. **Displays results** including transaction hash and Etherscan link

## Prerequisites

- **Rust 1.85+** and Cargo
- **Sepolia testnet ETH** (get free test ETH from [sepoliafaucet.com](https://sepoliafaucet.com))
- **Private key** for testing (never use mainnet keys!)

## Running the Example

### 1. Get Test ETH

Visit [https://sepoliafaucet.com](https://sepoliafaucet.com) and get some free Sepolia ETH for testing.

### 2. Set Your Private Key

```bash
export PRIVATE_KEY=0x...your-private-key-here...
```

**Security Warning**: Never use your mainnet private key for testing! Only use test network private keys.

### 3. Run the Example

```bash
cd examples/evm-transfer
cargo run
```

Or from the project root:

```bash
cd /path/to/apex-sdk
cargo run --manifest-path examples/evm-transfer/Cargo.toml
```

## Expected Output

```
=== Apex SDK: EVM Transfer Example ===

This example executes a real ETH transfer on Sepolia testnet.

Creating wallet...
  Wallet address: 0x...

Connecting to Sepolia testnet...
  ✓ Connected to Sepolia

Checking balance...
  Current balance: 100000000000000000 wei (0.1 ETH)

Transaction Details:
  From:   0x...
  To:     0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
  Amount: 10000000000000 wei (0.00001 ETH)
  Chain:  Sepolia (chain ID: 11155111)

Building transaction...
  ✓ Transaction built

Signing and sending transaction...
(This will use real test ETH on Sepolia)

Transaction Successful!

Transaction Hash:
  0x...

Status: Confirmed

View on Etherscan:
  https://sepolia.etherscan.io/tx/0x...

=== Example Complete ===

What happened:
  1. Created wallet from private key
  2. Connected to Sepolia testnet via RPC
  3. Checked account balance
  4. Built transaction with recipient and amount
  5. Signed transaction with wallet's private key
  6. Sent signed transaction to network
  7. Received transaction hash

Key Features Demonstrated:
  ✓ Real wallet integration with private key signing
  ✓ Type-safe transaction building
  ✓ Actual EVM transaction execution
  ✓ Transaction confirmation and hash retrieval
```

## Code Walkthrough

### Wallet Creation

```rust
let wallet = Wallet::from_private_key(&private_key)?
    .with_chain_id(11155111); // Sepolia chain ID
```

The wallet is created from a private key and configured with the Sepolia chain ID for proper transaction signing.

### SDK Initialization

```rust
let sdk = ApexSDK::builder()
    .with_evm_endpoint("https://eth-sepolia.g.alchemy.com/v2/demo")
    .with_evm_wallet(wallet)
    .build()
    .await?;
```

The SDK is initialized with:
- A Sepolia RPC endpoint (using Alchemy's public demo endpoint)
- The wallet for transaction signing

### Transaction Building

```rust
let tx = sdk
    .transaction()
    .from_evm_address(&from_address)
    .to_evm_address(recipient)
    .amount(amount)
    .build()?;
```

Apex SDK provides a type-safe transaction builder that:
- Validates addresses
- Converts amount to proper format
- Ensures all required fields are set

### Transaction Execution

```rust
let result = sdk.execute(tx).await?;
```

The SDK handles:
- Gas estimation
- Transaction signing with the wallet's private key
- Nonce management
- RLP encoding
- Network submission
- Status tracking

## Understanding the Transaction

### Transaction Components

- **From**: Your wallet address (automatically set from wallet)
- **To**: Recipient address (0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045)
- **Amount**: 10,000,000,000,000 wei = 0.00001 ETH
- **Gas**: Automatically estimated by the network
- **Nonce**: Automatically fetched and set
- **Chain ID**: 11155111 (Sepolia)
- **Signature**: ECDSA signature from your private key

### Transaction Lifecycle

1. **Build**: Create transaction request with all parameters
2. **Sign**: Generate ECDSA signature using private key and chain ID
3. **Encode**: RLP-encode the signed transaction
4. **Submit**: Send raw transaction bytes to RPC endpoint
5. **Confirm**: Wait for transaction to be included in a block

## Security Best Practices

**DO**:
- Use environment variables for private keys
- Test on testnets (Sepolia, Goerli) before mainnet
- Verify recipient addresses before sending
- Keep private keys secure and never commit them

**DON'T**:
- Use mainnet private keys for testing
- Hardcode private keys in source code
- Share private keys or commit them to version control
- Send transactions without checking balances first

## Troubleshooting

### "PRIVATE_KEY environment variable not set"

Make sure you've exported your private key:
```bash
export PRIVATE_KEY=0x...
```

### "Insufficient balance"

Get Sepolia test ETH from [https://sepoliafaucet.com](https://sepoliafaucet.com)

### "Failed to connect"

- Check your internet connection
- Try a different RPC endpoint
- Verify the endpoint URL is correct

### "Invalid private key"

Ensure your private key:
- Starts with `0x`
- Is 64 hexadecimal characters (+ the 0x prefix)
- Is a valid secp256k1 private key

## Next Steps

- Try modifying the recipient address
- Change the transfer amount
- Add contract interaction (see `evm-contract-call` example)
- Implement error handling and retry logic
- Add transaction confirmation waiting

## Related Examples

- **[evm-contract-call](../evm-contract-call/)** - Smart contract interaction
- **[account-manager](../account-manager/)** - Multi-chain account management

## Resources

- [Sepolia Faucet](https://sepoliafaucet.com) - Get free test ETH
- [Sepolia Etherscan](https://sepolia.etherscan.io) - View transactions
- [Apex SDK Documentation](../../docs/API.md) - Full API reference
- [Alloy Documentation](https://alloy.rs) - Underlying Ethereum library
