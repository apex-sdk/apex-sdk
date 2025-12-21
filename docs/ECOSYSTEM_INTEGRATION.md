# Apex SDK Ecosystem Integration Guide

## Overview

This guide describes how to integrate Apex SDK with various blockchain ecosystems, tools, and services to build comprehensive cross-chain applications.

## Table of Contents

- [Supported Ecosystems](#supported-ecosystems)
- [DeFi Protocol Integration](#defi-protocol-integration)
- [NFT Marketplace Integration](#nft-marketplace-integration)
- [Oracle Integration](#oracle-integration)
- [Wallet Integration](#wallet-integration)
- [Bridge Integration](#bridge-integration)
- [Developer Tools](#developer-tools)

## Supported Ecosystems

### Polkadot Ecosystem

#### Parachains

**Integrated:**
- **Moonbeam**: EVM-compatible smart contracts
- **Astar**: Multi-VM parachain (EVM + WASM)
- **Acala**: DeFi hub with native DEX
- **Phala**: Confidential computing
- **Bifrost**: Liquid staking derivatives

**Coming Soon:**
- **Interlay**: Bitcoin bridge
- **Parallel**: DeFi lending protocol
- **Centrifuge**: Real-world asset financing

#### Example: Acala DeFi Integration

```rust
use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to Acala
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://acala.api.onfinality.io/public-ws")
        .build()
        .await?;

    // Interact with Acala DEX
    let dex = sdk.substrate()
        .expect("Substrate adapter not initialized");

    // Swap DOT for LDOT (Liquid DOT)
    let swap_call = dex.build_extrinsic(
        "Dex",
        "swap_with_exact_supply",
        vec![
            ("path", vec!["DOT", "LDOT"]),
            ("supply_amount", 1000000000000u128), // 1 DOT
            ("min_target_amount", 900000000000u128), // Min 0.9 LDOT
        ],
    ).await?;

    let result = dex.submit_extrinsic(&swap_call).await?;
    println!("Swap transaction: {:?}", result);

    Ok(())
}
```

### Ethereum Ecosystem

#### Layer 2 Solutions

**Integrated:**
- **Arbitrum One**: Optimistic rollup
- **Optimism**: Optimistic rollup
- **zkSync Era**: ZK rollup
- **Base**: Coinbase L2

**Coming Soon:**
- **Polygon zkEVM**: ZK rollup
- **Scroll**: ZK rollup
- **Starknet**: ZK rollup (Cairo)

#### Example: Arbitrum Integration

```rust
use apex_sdk::prelude::*;
use ethers::types::{Address as EthAddress, U256};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to Arbitrum
    let sdk = ApexSDK::builder()
        .with_evm_endpoint("https://arb1.arbitrum.io/rpc")
        .build()
        .await?;

    let evm = sdk.evm()
        .expect("EVM adapter not initialized");

    // Deploy contract or interact with existing
    let contract_address = "0x912CE59144191C1204E64559FE8253a0e49E6548";
    let abi = include_str!("../contracts/ERC20.json");

    let contract = evm.contract(contract_address, abi).await?;

    // Call balanceOf
    let balance: U256 = contract
        .query("balanceOf", ("0xYourAddress",))
        .await?;

    println!("ARB Balance: {}", balance);

    Ok(())
}
```

## DeFi Protocol Integration

### Uniswap V3 Integration

```rust
use apex_sdk::prelude::*;
use ethers::prelude::*;

pub struct UniswapV3Integration {
    sdk: ApexSDK,
    router_address: String,
    factory_address: String,
}

impl UniswapV3Integration {
    pub async fn new(sdk: ApexSDK) -> Result<Self> {
        Ok(Self {
            sdk,
            router_address: "0xE592427A0AEce92De3Edee1F18E0157C05861564".to_string(),
            factory_address: "0x1F98431c8aD98523631AE4a59f267346ea31F984".to_string(),
        })
    }

    pub async fn swap_tokens(
        &self,
        token_in: &str,
        token_out: &str,
        amount_in: U256,
        min_amount_out: U256,
    ) -> Result<String> {
        let evm = self.sdk.evm()
            .expect("EVM adapter required");

        // Build swap parameters
        let params = SwapParams {
            token_in: token_in.parse()?,
            token_out: token_out.parse()?,
            fee: 3000, // 0.3% fee tier
            recipient: evm.default_account()?,
            deadline: U256::from(chrono::Utc::now().timestamp() + 300),
            amount_in,
            amount_out_minimum: min_amount_out,
            sqrt_price_limit_x96: U256::zero(),
        };

        // Execute swap
        let tx_hash = evm.send_transaction(
            &self.router_address,
            "exactInputSingle",
            params,
        ).await?;

        Ok(tx_hash)
    }

    pub async fn add_liquidity(
        &self,
        token0: &str,
        token1: &str,
        amount0: U256,
        amount1: U256,
        fee_tier: u32,
    ) -> Result<String> {
        // Implementation for adding liquidity
        todo!("Add liquidity implementation")
    }
}

#[derive(Debug)]
struct SwapParams {
    token_in: Address,
    token_out: Address,
    fee: u32,
    recipient: Address,
    deadline: U256,
    amount_in: U256,
    amount_out_minimum: U256,
    sqrt_price_limit_x96: U256,
}
```

### Aave Integration

```rust
pub struct AaveIntegration {
    sdk: ApexSDK,
    lending_pool: String,
}

impl AaveIntegration {
    pub async fn deposit(
        &self,
        asset: &str,
        amount: U256,
    ) -> Result<String> {
        let evm = self.sdk.evm()
            .expect("EVM adapter required");

        // Approve token
        let approve_tx = evm.send_transaction(
            asset,
            "approve",
            (&self.lending_pool, amount),
        ).await?;

        // Wait for approval
        evm.wait_for_transaction(&approve_tx).await?;

        // Deposit
        let tx_hash = evm.send_transaction(
            &self.lending_pool,
            "deposit",
            (asset, amount, evm.default_account()?, 0u16),
        ).await?;

        Ok(tx_hash)
    }

    pub async fn borrow(
        &self,
        asset: &str,
        amount: U256,
        interest_rate_mode: u8,
    ) -> Result<String> {
        let evm = self.sdk.evm()
            .expect("EVM adapter required");

        let tx_hash = evm.send_transaction(
            &self.lending_pool,
            "borrow",
            (asset, amount, interest_rate_mode, 0u16, evm.default_account()?),
        ).await?;

        Ok(tx_hash)
    }
}
```

## NFT Marketplace Integration

### OpenSea Integration

```rust
use apex_sdk::prelude::*;

pub struct OpenSeaIntegration {
    sdk: ApexSDK,
    seaport_address: String,
}

impl OpenSeaIntegration {
    pub async fn create_listing(
        &self,
        nft_contract: &str,
        token_id: U256,
        price: U256,
    ) -> Result<String> {
        let evm = self.sdk.evm()
            .expect("EVM adapter required");

        // Approve NFT
        let approve_tx = evm.send_transaction(
            nft_contract,
            "setApprovalForAll",
            (&self.seaport_address, true),
        ).await?;

        evm.wait_for_transaction(&approve_tx).await?;

        // Create order
        let order = self.build_seaport_order(
            nft_contract,
            token_id,
            price,
        ).await?;

        // Submit order to OpenSea
        self.submit_order_to_opensea(order).await
    }

    async fn build_seaport_order(
        &self,
        nft_contract: &str,
        token_id: U256,
        price: U256,
    ) -> Result<SeaportOrder> {
        // Build Seaport order structure
        todo!("Build Seaport order")
    }

    async fn submit_order_to_opensea(&self, order: SeaportOrder) -> Result<String> {
        // Submit to OpenSea API
        todo!("Submit to OpenSea")
    }
}

struct SeaportOrder {
    // Order parameters
}
```

### Cross-Chain NFT Bridge

```rust
pub struct CrossChainNFTBridge {
    sdk: ApexSDK,
    substrate_bridge: String,
    evm_bridge: String,
}

impl CrossChainNFTBridge {
    pub async fn bridge_nft_to_evm(
        &self,
        nft_id: u32,
        destination_chain: Chain,
    ) -> Result<String> {
        let substrate = self.sdk.substrate()
            .expect("Substrate adapter required");

        // Lock NFT on Substrate
        let lock_call = substrate.build_extrinsic(
            "NFTBridge",
            "lock_nft",
            vec![
                ("nft_id", nft_id),
                ("destination_chain", destination_chain as u8),
            ],
        ).await?;

        let tx_hash = substrate.submit_extrinsic(&lock_call).await?;

        // Wait for XCM message and mint on EVM
        self.wait_for_bridge_completion(&tx_hash).await
    }

    async fn wait_for_bridge_completion(&self, tx_hash: &str) -> Result<String> {
        // Monitor bridge events
        todo!("Monitor bridge completion")
    }
}
```

## Oracle Integration

### Chainlink Integration

```rust
use apex_sdk::prelude::*;

pub struct ChainlinkOracle {
    sdk: ApexSDK,
    price_feeds: std::collections::HashMap<String, String>,
}

impl ChainlinkOracle {
    pub fn new(sdk: ApexSDK) -> Self {
        let mut price_feeds = std::collections::HashMap::new();

        // Ethereum mainnet price feeds
        price_feeds.insert(
            "ETH/USD".to_string(),
            "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(),
        );
        price_feeds.insert(
            "BTC/USD".to_string(),
            "0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c".to_string(),
        );

        Self { sdk, price_feeds }
    }

    pub async fn get_price(&self, pair: &str) -> Result<f64> {
        let feed_address = self.price_feeds.get(pair)
            .ok_or_else(|| Error::Other(format!("Price feed not found: {}", pair)))?;

        let evm = self.sdk.evm()
            .expect("EVM adapter required");

        let abi = r#"[{
            "inputs": [],
            "name": "latestRoundData",
            "outputs": [
                {"name": "roundId", "type": "uint80"},
                {"name": "answer", "type": "int256"},
                {"name": "startedAt", "type": "uint256"},
                {"name": "updatedAt", "type": "uint256"},
                {"name": "answeredInRound", "type": "uint80"}
            ],
            "stateMutability": "view",
            "type": "function"
        }]"#;

        let contract = evm.contract(feed_address, abi).await?;
        let (_, answer, _, _, _): (u128, i128, u64, u64, u128) =
            contract.query("latestRoundData", ()).await?;

        // Convert to float (assuming 8 decimals)
        Ok(answer as f64 / 100_000_000.0)
    }

    pub async fn subscribe_to_price_updates(
        &self,
        pair: &str,
    ) -> Result<EventSubscription> {
        use apex_sdk_types::{Event, EventFilter};

        let filter = EventFilter {
            event_names: Some(vec!["AnswerUpdated".to_string()]),
            addresses: Some(vec![
                Address::evm(self.price_feeds.get(pair).unwrap())
            ]),
            from_block: None,
            to_block: None,
        };

        Ok(EventSubscription::new(filter))
    }
}
```

## Wallet Integration

### Browser Wallet Integration

```rust
// For web applications using WASM
#[cfg(target_arch = "wasm32")]
pub mod browser {
    use apex_sdk::prelude::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct WalletConnector {
        provider: String,
    }

    #[wasm_bindgen]
    impl WalletConnector {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self {
                provider: "metamask".to_string(),
            }
        }

        pub async fn connect_metamask(&self) -> Result<String, JsValue> {
            // Request account access
            let window = web_sys::window().unwrap();
            let ethereum = js_sys::Reflect::get(
                &window,
                &JsValue::from_str("ethereum"),
            )?;

            let accounts = js_sys::Reflect::get(
                &ethereum,
                &JsValue::from_str("request"),
            )?;

            // Call eth_requestAccounts
            Ok("connected".to_string())
        }

        pub async fn sign_transaction(&self, tx_data: &str) -> Result<String, JsValue> {
            // Sign transaction with wallet
            todo!("Sign transaction")
        }
    }
}
```

### Hardware Wallet Support (Planned)

```rust
// Future implementation
pub trait HardwareWallet {
    async fn connect(&self) -> Result<()>;
    async fn get_accounts(&self) -> Result<Vec<Address>>;
    async fn sign_transaction(&self, tx: &Transaction) -> Result<Vec<u8>>;
    async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>>;
}

pub struct LedgerWallet {
    // Ledger-specific implementation
}

pub struct TrezorWallet {
    // Trezor-specific implementation
}
```

## Bridge Integration

### Snowbridge (Polkadot ↔ Ethereum)

```rust
pub struct SnowbridgeIntegration {
    sdk: ApexSDK,
}

impl SnowbridgeIntegration {
    pub async fn transfer_to_ethereum(
        &self,
        amount: u128,
        eth_recipient: &str,
    ) -> Result<String> {
        let substrate = self.sdk.substrate()
            .expect("Substrate adapter required");

        let call = substrate.build_extrinsic(
            "EthereumOutboundQueue",
            "send_message",
            vec![
                ("recipient", eth_recipient),
                ("amount", amount),
            ],
        ).await?;

        substrate.submit_extrinsic(&call).await
    }

    pub async fn monitor_transfer(&self, tx_hash: &str) -> Result<TransferStatus> {
        // Monitor transfer progress
        todo!("Monitor Snowbridge transfer")
    }
}

pub enum TransferStatus {
    Initiated,
    ValidatorsSigned,
    ExecutedOnDestination,
    Failed(String),
}
```

## Developer Tools

### Block Explorer Integration

```rust
pub struct BlockExplorerClient {
    api_key: String,
    base_url: String,
}

impl BlockExplorerClient {
    pub async fn get_transaction(&self, tx_hash: &str) -> Result<TransactionDetails> {
        let url = format!("{}/api?module=transaction&action=gettxinfo&txhash={}",
            self.base_url, tx_hash);

        let response = reqwest::get(&url).await?;
        let data: TransactionDetails = response.json().await?;

        Ok(data)
    }

    pub async fn get_contract_abi(&self, address: &str) -> Result<String> {
        let url = format!("{}/api?module=contract&action=getabi&address={}",
            self.base_url, address);

        let response = reqwest::get(&url).await?;
        let data: ApiResponse = response.json().await?;

        Ok(data.result)
    }
}
```

### IPFS Integration

```rust
pub struct IPFSClient {
    gateway: String,
}

impl IPFSClient {
    pub async fn upload_metadata(&self, metadata: &NFTMetadata) -> Result<String> {
        let json = serde_json::to_string(metadata)?;

        // Upload to IPFS
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/api/v0/add", self.gateway))
            .body(json)
            .send()
            .await?;

        let data: IPFSResponse = response.json().await?;
        Ok(format!("ipfs://{}", data.hash))
    }

    pub async fn fetch_metadata(&self, ipfs_hash: &str) -> Result<NFTMetadata> {
        let url = format!("{}/ipfs/{}", self.gateway, ipfs_hash);
        let response = reqwest::get(&url).await?;
        let metadata = response.json().await?;
        Ok(metadata)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}
```

## Complete Example: Cross-Chain DeFi Application

```rust
use apex_sdk::prelude::*;

pub struct CrossChainDeFiApp {
    sdk: ApexSDK,
    chainlink: ChainlinkOracle,
    uniswap: UniswapV3Integration,
    acala_dex: AcalaDexIntegration,
}

impl CrossChainDeFiApp {
    pub async fn new() -> Result<Self> {
        let sdk = ApexSDK::builder()
            .with_substrate_endpoint("wss://acala.api.onfinality.io/public-ws")
            .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
            .build()
            .await?;

        let chainlink = ChainlinkOracle::new(sdk.clone());
        let uniswap = UniswapV3Integration::new(sdk.clone()).await?;
        let acala_dex = AcalaDexIntegration::new(sdk.clone()).await?;

        Ok(Self {
            sdk,
            chainlink,
            uniswap,
            acala_dex,
        })
    }

    pub async fn execute_arbitrage(&self) -> Result<()> {
        // Get prices from both chains
        let eth_price = self.uniswap.get_price("DOT/USDT").await?;
        let acala_price = self.acala_dex.get_price("DOT/aUSD").await?;

        // Check for arbitrage opportunity
        if eth_price > acala_price * 1.01 {
            // Buy on Acala, sell on Ethereum
            self.execute_cross_chain_arbitrage(acala_price, eth_price).await?;
        }

        Ok(())
    }

    async fn execute_cross_chain_arbitrage(
        &self,
        buy_price: f64,
        sell_price: f64,
    ) -> Result<()> {
        // Implementation
        todo!("Cross-chain arbitrage")
    }
}
```

## Resources

- [Polkadot Parachain Directory](https://polkadot.network/ecosystem/)
- [Ethereum L2 Comparison](https://l2beat.com)
- [Chainlink Price Feeds](https://data.chain.link)
- [OpenSea API Docs](https://docs.opensea.io/reference/api-overview)

## Support

For integration support:
- **Email:** partnerships@apexsdk.dev
- **Discord:** [Apex SDK Community](https://discord.gg/zCDFsBaZJN)
- **Docs:** https://docs.rs/apex-sdk

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../LICENSE) for details.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.


