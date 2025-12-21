# Testnet Guide for Apex SDK Protocol

This guide provides comprehensive information about testnets supported by Apex SDK, with a focus on **Paseo** as the recommended default testnet for Substrate development.

## Table of Contents

- [Overview](#overview)
- [Substrate Testnets](#substrate-testnets)
  - [Paseo (Recommended)](#paseo-recommended)
  - [Westend](#westend)
- [EVM Testnets](#evm-testnets)
  - [Sepolia](#sepolia)
- [Getting Started](#getting-started)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Overview

Apex SDK follows a **testnet-first approach** to ensure developers can safely test their applications before deploying to mainnet. This approach:

- Prevents accidental mainnet deployments
- Reduces financial risk during development
- Provides a safe environment for experimentation
- Mirrors mainnet functionality without real value

## Substrate Testnets

### Paseo (Recommended)

**Paseo** is the default and recommended testnet for Substrate-based development in Apex SDK.

#### **What is Paseo?**

Paseo is a community-run Polkadot testnet that provides a reliable testing environment for parachain teams and developers. It offers:

- Full Polkadot feature parity
- Stable infrastructure maintained by the community
- Regular runtime upgrades to match Polkadot
- Active validator set and network participants

#### **Network Specifications**

| Property | Value |
|----------|-------|
| **Network Name** | Paseo |
| **Chain Type** | Substrate (Relay Chain) |
| **RPC Endpoint** | `wss://paseo.rpc.amforc.com` |
| **SS58 Prefix** | 42 (Generic Substrate) |
| **Token Symbol** | PAS |
| **Token Decimals** | 10 |
| **Block Time** | ~6 seconds |
| **Finality** | ~10-12 blocks |

#### **Getting Testnet Tokens**

**Option 1: Faucet (Recommended)**
```bash
# Coming soon - Check Paseo documentation for current faucet
```

**Option 2: Community Channels**
- Join the [Polkadot Discord](https://dot.li/discord)
- Request tokens in the #paseo-faucet channel
- Provide your Paseo address (SS58 format with prefix 42)

**Option 3: Matrix Chat**
- Join [#paseo:matrix.org](https://matrix.to/#/#paseo:matrix.org)
- Request tokens from community members

#### **Block Explorers**

- **Subscan**: https://paseo.subscan.io/
- **Polkadot.js Apps**: https://polkadot.js.org/apps/?rpc=wss://paseo.rpc.amforc.com

#### **Using Paseo with Apex SDK**

**CLI Usage** (Default - No configuration needed):
```bash
# Paseo is the default testnet
apex account generate --type substrate

# Check balance (automatically uses Paseo)
apex balance <YOUR_ADDRESS>

# Deploy contract to Paseo
apex deploy contract.wasm --account my-account
```

**Explicit Paseo Configuration**:
```bash
# Explicitly specify Paseo
apex balance <ADDRESS> --chain paseo --endpoint wss://paseo.rpc.amforc.com
```

**SDK Library Usage**:
```rust
use apex_sdk_substrate::{SubstrateAdapter, ChainConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Option 1: Use predefined Paseo configuration
    let adapter = SubstrateAdapter::connect_with_config(
        ChainConfig::paseo()
    ).await?;

    // Option 2: Custom endpoint
    let adapter = SubstrateAdapter::connect("wss://paseo.rpc.amforc.com").await?;

    // Get balance
    let balance = adapter.get_balance("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").await?;
    println!("Balance: {}", balance);

    Ok(())
}
```

#### **Common Operations**

**Generate a Paseo Address**:
```bash
apex account generate --type substrate --name my-paseo-account
# Address will use SS58 prefix 42 (generic Substrate format)
```

**Check Balance**:
```bash
apex balance 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --chain paseo
```

**Deploy WASM Contract** (requires Contracts pallet support):
```bash
apex deploy my-contract.wasm --chain paseo --account my-paseo-account
```

#### **Limitations & Considerations**

**Important Notes**:

1. **Contract Support**: Paseo relay chain does not natively support smart contracts. For contract deployment:
   - Use a parachain with Contracts pallet (e.g., Contracts on Rococo)
   - Or deploy to a Paseo parachain that supports contracts

2. **Network Resets**: As a testnet, Paseo may undergo planned resets. Always backup important data.

3. **Token Value**: PAS tokens have no real value - never pay for them!

4. **Rate Limits**: Public RPC endpoints may have rate limits. Consider running your own node for heavy usage.

---

### Westend

**Westend** is the official Parity-maintained testnet for Polkadot.

#### **Network Specifications**

| Property | Value |
|----------|-------|
| **Network Name** | Westend |
| **Chain Type** | Substrate (Relay Chain) |
| **RPC Endpoint** | `wss://westend-rpc.polkadot.io` |
| **Alternative Endpoints** | `wss://rpc.ibp.network/westend`<br>`wss://westend.dotters.network` |
| **SS58 Prefix** | 42 |
| **Token Symbol** | WND |
| **Token Decimals** | 12 |

#### **Using Westend**

```bash
# Configure Westend as the default chain
apex config set default_chain westend

# Or use explicitly
apex balance <ADDRESS> --chain westend
```

#### **Getting WND Tokens**

- **Faucet**: https://faucet.polkadot.io/westend
- **Matrix**: [#westend_faucet:matrix.org](https://matrix.to/#/#westend_faucet:matrix.org)

---

## EVM Testnets

### Sepolia

**Sepolia** is the recommended Ethereum testnet (successor to Goerli).

#### **Network Specifications**

| Property | Value |
|----------|-------|
| **Network Name** | Sepolia |
| **Chain Type** | EVM |
| **RPC Endpoint** | `https://ethereum-sepolia-rpc.publicnode.com` |
| **Chain ID** | 11155111 |
| **Block Explorer** | https://sepolia.etherscan.io |

#### **Getting Testnet ETH**

**Faucets**:
- Alchemy: https://sepoliafaucet.com/
- Infura: https://www.infura.io/faucet/sepolia
- QuickNode: https://faucet.quicknode.com/ethereum/sepolia

#### **Using Sepolia with Apex SDK**

```bash
# Configure Sepolia
apex config set default_chain sepolia
apex config set endpoints.sepolia https://ethereum-sepolia-rpc.publicnode.com

# Deploy EVM contract
apex deploy contract.bin --chain sepolia --account my-evm-account
```

---

## Getting Started

### Quick Start with Paseo (Recommended)

1. **Install Apex SDK CLI**:
   ```bash
   cargo install apex-sdk-cli
   ```

2. **Initialize Configuration** (Paseo is default):
   ```bash
   apex config init
   # Select "paseo (Testnet - Recommended)" when prompted
   ```

3. **Generate a Testnet Account**:
   ```bash
   apex account generate --type substrate --name testnet-account
   ```

4. **Request Testnet Tokens**:
   - Copy your address from the previous step
   - Visit a Paseo faucet or request in community channels
   - Paste your address to receive PAS tokens

5. **Verify Your Balance**:
   ```bash
   apex balance <YOUR_ADDRESS>
   ```

6. **Start Building**:
   ```bash
   apex new my-dapp
   cd my-dapp
   cargo build
   ```

---

## Best Practices

### Development Workflow

**DO**:
- Always test on testnets before mainnet deployment
- Use Paseo as your primary Substrate testnet
- Keep testnet and mainnet accounts separate
- Document your testnet configurations
- Test transaction fees and gas estimates
- Verify contract behavior on testnet first

**DON'T**:
- Skip testnet testing to save time
- Use the same mnemonic for testnet and mainnet
- Assume testnet behavior matches mainnet exactly
- Deploy untested code to mainnet
- Share testnet private keys (still good practice)

### Security on Testnets

While testnet tokens have no value, maintain good security practices:

- **Never reuse mainnet seeds** on testnets
- **Use separate wallets** for different environments
- **Don't commit private keys** to version control
- **Test security features** on testnets first
- **Practice incident response** on testnets

### Configuration Management

**Recommended Setup**:

```bash
# ~/.config/apex-sdk/config.json
{
  "default_chain": "paseo",
  "default_endpoint": "wss://paseo.rpc.amforc.com",
  "default_account": "testnet-dev",
  "endpoints": {
    "paseo": "wss://paseo.rpc.amforc.com",
    "westend": "wss://westend-rpc.polkadot.io",
    "sepolia": "https://ethereum-sepolia-rpc.publicnode.com",
    "polkadot": "wss://polkadot.api.onfinality.io/public-ws",
    "ethereum": "https://eth.llamarpc.com"
  }
}
```

**Environment-Based Configuration**:

```bash
# Development (default testnet)
export APEX_CHAIN=paseo

# Staging (alternative testnet)
export APEX_CHAIN=westend

# Production (explicit mainnet)
export APEX_CHAIN=polkadot
```

---

## Troubleshooting

### Common Issues

#### "Connection failed" or "Endpoint unreachable"

**Solutions**:
1. Check your internet connection
2. Try alternative RPC endpoints:
   ```bash
   apex config set endpoints.paseo wss://paseo-rpc.dwellir.com
   ```
3. Verify the endpoint is operational:
   ```bash
   curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' https://paseo.rpc.amforc.com
   ```

#### "Insufficient balance" when deploying

**Solutions**:
1. Request more testnet tokens from faucet
2. Verify your account address is correct
3. Check balance: `apex balance <ADDRESS> --chain paseo`

#### "Contract deployment failed: Contracts pallet not found"

**Explanation**: The Paseo relay chain doesn't support smart contracts directly.

**Solutions**:
1. Use a parachain testnet with Contracts pallet support
2. Deploy to Contracts on Rococo (specialized contracts testnet)
3. Use a local development node with `pallet-contracts`

#### "Transaction timeout" or "Block not finalized"

**Solutions**:
1. Wait longer - testnet block times can vary
2. Check network status on block explorer
3. Retry with higher gas/fees
4. Use a different RPC endpoint

### Getting Help

**Community Support**:
- **Documentation**: https://docs.rs/apex-sdk
- **Discord**: Join Polkadot Discord for testnet support
- **Issues**: https://github.com/carbobit/apex-sdk/issues
- **Email**: support@apex-sdk.io (for enterprise users)

**Useful Resources**:
- [Substrate Documentation](https://docs.substrate.io/)
- [Polkadot Wiki - Testnets](https://wiki.polkadot.network/docs/build-networks)
- [Parity GitHub](https://github.com/paritytech)

---

## Appendix: All Supported Testnets

### Substrate Testnets

| Network | Endpoint | Prefix | Symbol | Decimals |
|---------|----------|--------|--------|----------|
| Paseo (Default) | `wss://paseo.rpc.amforc.com` | 42 | PAS | 10 |
| Westend | `wss://westend-rpc.polkadot.io` | 42 | WND | 12 |

### EVM Testnets

| Network | Endpoint | Chain ID | Symbol |
|---------|----------|----------|--------|
| Sepolia | `https://ethereum-sepolia-rpc.publicnode.com` | 11155111 | ETH |

---

## Contributing

Found an issue with testnet support? Want to add information about a new testnet?

1. Fork the repository
2. Update this documentation
3. Submit a pull request
