//! Apex SDK CLI tool

use anyhow::Context;
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

mod account;
mod balance;
mod completions;
mod config;
mod config_cmd;
mod deploy;
mod keystore;

#[derive(Parser)]
#[command(name = "apex")]
#[command(about = "Apex SDK CLI - Unified Rust SDK for Substrate & EVM", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Apex SDK project
    New {
        /// Name of the project
        name: String,
        /// Project template (default, defi, nft)
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    /// Build the project
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,
    },
    /// Run tests
    Test {
        /// Run only tests matching this pattern
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Deploy a smart contract
    Deploy {
        /// Path to the contract file
        contract: String,
        /// Chain to deploy to (polkadot, ethereum, etc.)
        #[arg(short, long)]
        chain: String,
        /// RPC endpoint URL
        #[arg(short, long)]
        endpoint: String,
        /// Account name to use for deployment
        #[arg(short, long)]
        account: Option<String>,
        /// Perform a dry-run without broadcasting the transaction
        #[arg(long)]
        dry_run: bool,
    },
    /// Manage accounts and wallets
    Account {
        #[command(subcommand)]
        action: AccountCommands,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    /// Get chain information
    Chain {
        #[command(subcommand)]
        action: ChainCommands,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for (bash, zsh, fish, powershell, elvish)
        shell: String,
    },
    /// Initialize configuration (deprecated: use 'apex config init')
    Init {
        /// Interactive mode
        #[arg(short, long)]
        interactive: bool,
    },
    /// Run benchmarks
    Bench {
        /// Benchmark filter pattern
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Show version information
    Version,
}

#[derive(Subcommand)]
enum AccountCommands {
    /// Generate a new account
    Generate {
        /// Account type (substrate, evm)
        #[arg(short = 't', long)]
        account_type: String,
        /// Account name (optional, will prompt to save if provided)
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Import account from mnemonic
    Import {
        /// Mnemonic phrase
        mnemonic: String,
        /// Account type (substrate, evm)
        #[arg(short = 't', long)]
        account_type: String,
        /// Account name
        #[arg(short, long)]
        name: String,
    },
    /// List all accounts
    List,
    /// Export account mnemonic
    Export {
        /// Account name
        name: String,
    },
    /// Remove an account
    Remove {
        /// Account name
        name: String,
    },
    /// Get account balance
    Balance {
        /// Account address
        address: String,
        /// Chain name
        #[arg(short, long)]
        chain: String,
        /// RPC endpoint
        #[arg(short, long)]
        endpoint: String,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,
    /// Set a configuration value
    Set {
        /// Configuration key (e.g., default_chain, endpoints.polkadot)
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Validate configuration
    Validate,
    /// Reset configuration to defaults
    Reset {
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    /// Initialize configuration interactively
    Init,
}

#[derive(Subcommand)]
enum ChainCommands {
    /// List supported chains
    List,
    /// Get chain information
    Info {
        /// Chain name
        chain: String,
        /// RPC endpoint
        #[arg(short, long)]
        endpoint: String,
    },
    /// Check chain health
    Health {
        /// RPC endpoint
        endpoint: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, template } => {
            print_apex_banner();
            println!("\nInitializing new Apex SDK project...\n");
            create_project(&name, &template)?;
            print_success_message(&name, &template);
        }
        Commands::Build { release } => {
            println!("🔨 Building project...");
            build_project(release).await?;
            println!("Build completed!");
        }
        Commands::Test { filter } => {
            println!("🧪 Running tests...");
            run_tests(filter).await?;
            println!("Tests passed!");
        }
        Commands::Deploy {
            contract,
            chain,
            endpoint,
            account,
            dry_run,
        } => {
            if dry_run {
                println!("Dry-run mode: Simulating deployment without broadcasting...");
            } else {
                println!("Deploying contract...");
            }
            deploy::deploy_contract(&contract, &chain, &endpoint, account, dry_run).await?;
        }
        Commands::Account { action } => match action {
            AccountCommands::Generate { account_type, name } => {
                println!("🔑 Generating new {} account...", account_type);
                account::generate_account(&account_type, name)?;
            }
            AccountCommands::Import {
                mnemonic,
                account_type,
                name,
            } => {
                println!("📥 Importing {} account...", account_type);
                account::import_account(&mnemonic, &account_type, name)?;
            }
            AccountCommands::List => {
                account::list_accounts()?;
            }
            AccountCommands::Export { name } => {
                account::export_account(&name)?;
            }
            AccountCommands::Remove { name } => {
                account::remove_account(&name)?;
            }
            AccountCommands::Balance {
                address,
                chain,
                endpoint,
            } => {
                balance::get_balance(&address, &chain, &endpoint).await?;
            }
        },
        Commands::Config { action } => match action {
            ConfigCommands::Show => {
                config_cmd::show_config()?;
            }
            ConfigCommands::Set { key, value } => {
                config_cmd::set_config(&key, &value)?;
            }
            ConfigCommands::Get { key } => {
                config_cmd::get_config(&key)?;
            }
            ConfigCommands::Validate => {
                config_cmd::validate_config()?;
            }
            ConfigCommands::Reset { force } => {
                config_cmd::reset_config(force)?;
            }
            ConfigCommands::Init => {
                config_cmd::init_config_interactive().await?;
            }
        },
        Commands::Chain { action } => match action {
            ChainCommands::List => {
                println!("Supported chains:");
                list_chains();
            }
            ChainCommands::Info { chain, endpoint } => {
                println!("ℹ️  Fetching chain info for {}...", chain);
                get_chain_info(&chain, &endpoint).await?;
            }
            ChainCommands::Health { endpoint } => {
                println!("🏥 Checking chain health...");
                check_chain_health(&endpoint).await?;
            }
        },
        Commands::Completions { shell } => {
            completions::generate_completions(&shell)?;
            eprintln!("\n# Installation instructions:");
            completions::print_install_instructions(&shell);
        }
        Commands::Init { interactive } => {
            eprintln!("Note: 'apex init' is deprecated. Use 'apex config init' instead.\n");
            if interactive {
                config_cmd::init_config_interactive().await?;
            } else {
                let config_path = config::get_config_path()?;
                let config = config::Config::default();
                config.save(&config_path)?;
                println!("Configuration initialized at: {}", config_path.display());
                println!("Use 'apex config init' for interactive setup");
            }
        }
        Commands::Bench { filter } => {
            println!("📊 Running benchmarks...");
            run_benchmarks(filter).await?;
            println!("Benchmarks completed!");
        }
        Commands::Version => {
            println!("Apex SDK CLI v{}", env!("CARGO_PKG_VERSION"));
            println!("Rust SDK for Substrate & EVM blockchain development");
            println!("\nSupported chains:");
            println!("  • Substrate: Polkadot, Kusama, Moonbeam, Astar");
            println!("  • EVM: Ethereum, BSC, Polygon, Avalanche");
        }
    }

    Ok(())
}

fn print_apex_banner() {
    println!(
        r#"
    ╔═══════════════════════════════════════════════════════════════════╗
    ║                                                                   ║
    ║      █████╗ ██████╗ ███████╗██╗  ██╗    ███████╗██████╗ ██╗  ██╗ ║
    ║     ██╔══██╗██╔══██╗██╔════╝╚██╗██╔╝    ██╔════╝██╔══██╗██║ ██╔╝ ║
    ║     ███████║██████╔╝█████╗   ╚███╔╝     ███████╗██║  ██║█████╔╝  ║
    ║     ██╔══██║██╔═══╝ ██╔══╝   ██╔██╗     ╚════██║██║  ██║██╔═██╗  ║
    ║     ██║  ██║██║     ███████╗██╔╝ ██╗    ███████║██████╔╝██║  ██╗ ║
    ║     ╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝    ╚══════╝╚═════╝ ╚═╝  ╚═╝ ║
    ║                                                                   ║
    ║           Unified Rust SDK for Substrate & EVM Chains            ║
    ║                    Cross-Chain Made Simple                       ║
    ║                                                                   ║
    ╚═══════════════════════════════════════════════════════════════════╝
"#
    );
}

fn create_project(name: &str, template: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(name);

    // Step 1: Create directory structure
    print_step(1, "Creating project structure");
    std::fs::create_dir_all(&path)?;
    std::fs::create_dir_all(path.join("src"))?;
    std::fs::create_dir_all(path.join("tests"))?;
    std::fs::create_dir_all(path.join("examples"))?;
    std::fs::create_dir_all(path.join(".vscode"))?;
    println!("   ✓ Project directories created");

    // Step 2: Create Cargo.toml
    print_step(2, "Configuring project dependencies");
    let cargo_toml = generate_cargo_toml(name, template);
    std::fs::write(path.join("Cargo.toml"), cargo_toml)?;
    println!("   ✓ Cargo.toml configured");

    // Step 3: Create main source file
    print_step(3, "Generating source code from template");
    let main_rs = match template {
        "defi" => include_str!("../templates/defi.rs"),
        "nft" => include_str!("../templates/nft.rs"),
        _ => include_str!("../templates/default.rs"),
    };
    std::fs::write(path.join("src/main.rs"), main_rs)?;
    println!("   ✓ Source code generated");

    // Step 4: Create additional files
    print_step(4, "Creating project documentation");
    create_readme(&path, name, template)?;
    create_gitignore(&path)?;
    create_vscode_settings(&path)?;
    create_example_test(&path)?;
    println!("   ✓ Documentation and configs created");

    // Step 5: Create example file
    print_step(5, "Setting up examples");
    create_example_file(&path, template)?;
    println!("   ✓ Example files created");

    Ok(())
}

fn print_step(step: u8, description: &str) {
    println!("\n📍 Step {}/5: {}", step, description);
}

fn generate_cargo_toml(name: &str, template: &str) -> String {
    let description = match template {
        "defi" => "A DeFi application built with Apex SDK",
        "nft" => "An NFT marketplace built with Apex SDK",
        _ => "A cross-chain application built with Apex SDK",
    };

    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "{}"
license = "MIT OR Apache-2.0"

[dependencies]
apex-sdk = "0.1.0"
tokio = {{ version = "1.35", features = ["full"] }}
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tokio-test = "0.4"

[[example]]
name = "quickstart"
path = "examples/quickstart.rs"
"#,
        name, description
    )
}

fn create_readme(path: &Path, name: &str, template: &str) -> anyhow::Result<()> {
    let readme = format!(
        r#"# {}

> {} built with [Apex SDK](https://github.com/kherldhussein/apex-sdk)

## Overview

This project demonstrates cross-chain blockchain development using the Apex SDK.

**Template:** `{}`

## Features

-Substrate & EVM support
-Type-safe blockchain interactions
-Built-in connection pooling
-Automatic retry logic
-Comprehensive error handling

## 🛠️ Getting Started

### Prerequisites

- Rust 1.85+ (edition 2021)
- Cargo

### Installation

```bash
cargo build
```

### Running

```bash
# Run the main application
cargo run

# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Run examples
cargo run --example quickstart
```

## 📖 Documentation

- [Apex SDK Documentation](https://github.com/kherldhussein/apex-sdk)
- [API Reference](https://docs.rs/apex-sdk)
- [Examples](./examples/)

## 🔧 Configuration

Edit `src/main.rs` to customize:
- RPC endpoints
- Chain selection
- Transaction parameters

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under MIT OR Apache-2.0

## 🙏 Acknowledgments

Built with [Apex SDK](https://github.com/kherldhussein/apex-sdk) - Unified Rust SDK for Substrate & EVM chains.
"#,
        name,
        match template {
            "defi" => "A DeFi application",
            "nft" => "An NFT marketplace",
            _ => "A cross-chain application",
        },
        template
    );

    std::fs::write(path.join("README.md"), readme)?;
    Ok(())
}

fn create_gitignore(path: &Path) -> anyhow::Result<()> {
    let gitignore = r#"# Rust
/target/
**/*.rs.bk
*.pdb

# Cargo
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Environment
.env
.env.local
"#;

    std::fs::write(path.join(".gitignore"), gitignore)?;
    Ok(())
}

fn create_vscode_settings(path: &Path) -> anyhow::Result<()> {
    let settings = r#"{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "rust-analyzer.cargo.features": "all"
}
"#;

    std::fs::write(path.join(".vscode/settings.json"), settings)?;
    Ok(())
}

fn create_example_test(path: &Path) -> anyhow::Result<()> {
    let test = r#"#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = async { 42 }.await;
        assert_eq!(result, 42);
    }
}
"#;

    std::fs::write(path.join("tests/integration_test.rs"), test)?;
    Ok(())
}

fn create_example_file(path: &Path, template: &str) -> anyhow::Result<()> {
    let example = match template {
        "defi" => {
            r#"//! DeFi Quickstart Example
//!
//! This example demonstrates basic DeFi operations using Apex SDK.

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🏦 Apex SDK DeFi Quickstart Example\n");

    // Your DeFi logic here

    Ok(())
}
"#
        }
        "nft" => {
            r#"//! NFT Quickstart Example
//!
//! This example demonstrates NFT operations using Apex SDK.

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🎨 Apex SDK NFT Quickstart Example\n");

    // Your NFT logic here

    Ok(())
}
"#
        }
        _ => {
            r#"//! Cross-Chain Quickstart Example
//!
//! This example demonstrates basic cross-chain operations using Apex SDK.

use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("⚡ Apex SDK Cross-Chain Quickstart Example\n");

    // Connect to Polkadot
    println!("Connecting to Polkadot...");
    // Your cross-chain logic here

    Ok(())
}
"#
        }
    };

    std::fs::write(path.join("examples/quickstart.rs"), example)?;
    Ok(())
}

fn print_success_message(name: &str, template: &str) {
    println!(
        r#"
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║  ✨ SUCCESS! Your project is ready to go! ✨                      ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝

Project: {}
🎨 Template: {}

📁 Project Structure:
   {}
   ├── 📄 Cargo.toml          (Project configuration)
   ├── 📄 README.md           (Project documentation)
   ├── 📄 .gitignore          (Git configuration)
   ├── 📂 src/
   │   └── 📄 main.rs         (Your main application)
   ├── 📂 tests/
   │   └── 📄 integration_test.rs
   ├── 📂 examples/
   │   └── 📄 quickstart.rs   (Example code)
   └── 📂 .vscode/
       └── 📄 settings.json   (VS Code settings)

Next Steps:

   1️⃣  Navigate to your project:
       cd {}

   2️⃣  Build the project:
       cargo build

   3️⃣  Run the application:
       cargo run

   4️⃣  Run the example:
       cargo run --example quickstart

   5️⃣  Read the docs:
       cargo doc --open

Useful Commands:

   • cargo test              Run tests
   • cargo clippy            Lint your code
   • cargo fmt               Format your code
   • cargo build --release   Build optimized binary

📚 Resources:

   • Apex SDK Docs:    https://github.com/kherldhussein/apex-sdk
   • API Reference:    https://docs.rs/apex-sdk
   • CLI Guide:        apex --help

Happy coding!
"#,
        name, template, name, name
    );
}

async fn build_project(release: bool) -> anyhow::Result<()> {
    println!(
        "   🔨 Building project{}...",
        if release { " (release mode)" } else { "" }
    );
    println!("   This may take a while on first build...\n");

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");
    if release {
        cmd.arg("--release");
    }

    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "\nBuild failed!\n\n\
            Error details:\n{}\n\n\
            Common fixes:\n\
            • Run 'cargo clean' to clear build cache\n\
            • Update dependencies with 'cargo update'\n\
            • Check for compilation errors above\n\
            • Ensure Rust toolchain is up to date: rustup update\n",
            stderr
        );
    }

    println!("Build completed successfully!");
    if release {
        println!("   Binary available in ./target/release/");
    } else {
        println!("   Binary available in ./target/debug/");
    }
    Ok(())
}

async fn run_tests(filter: Option<String>) -> anyhow::Result<()> {
    if let Some(ref pattern) = filter {
        println!("   🧪 Running tests matching '{}'...", pattern);
    } else {
        println!("   🧪 Running all tests...");
    }
    println!("   Please wait...\n");

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("test");
    if let Some(pattern) = filter {
        cmd.arg(pattern);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "\nTests failed!\n\n\
            Error details:\n{}\n\n\
            Troubleshooting:\n\
            • Review the test output above for specific failures\n\
            • Run tests individually: cargo test <test_name>\n\
            • Run with output: cargo test -- --nocapture\n\
            • Check for compilation errors first: cargo check\n",
            stderr
        );
    }

    println!("All tests passed!");
    Ok(())
}

fn list_chains() {
    println!("\n   Substrate Mainnets:");
    println!("     • polkadot    - Polkadot Relay Chain");
    println!("     • kusama      - Kusama Relay Chain");

    println!("\n   Substrate Testnets:");
    println!("     • paseo       - Paseo Testnet (Default)");
    println!("     • westend     - Westend Testnet");

    println!("\n   Parachains:");
    println!("     • moonbeam    - Moonbeam (EVM-compatible)");
    println!("     • astar       - Astar Multi-VM");
    println!("     • acala       - Acala DeFi Hub");
    println!("     • phala       - Phala Privacy Cloud");
    println!("     • bifrost     - Bifrost Liquid Staking");

    println!("\n   EVM Mainnets:");
    println!("     • ethereum    - Ethereum Mainnet");
    println!("     • bsc         - Binance Smart Chain");
    println!("     • polygon     - Polygon (Matic)");
    println!("     • avalanche   - Avalanche C-Chain");
    println!("     • arbitrum    - Arbitrum One (L2)");
    println!("     • optimism    - Optimism (L2)");
    println!("     • zksync      - zkSync Era (L2)");

    println!("\n   EVM Testnets:");
    println!("     • sepolia     - Ethereum Sepolia");

    println!("\n   Use 'apex config show' to see configured endpoints");
}

async fn get_chain_info(chain: &str, endpoint: &str) -> anyhow::Result<()> {
    use colored::Colorize;

    println!("\n{}", "Chain Information".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Chain".dimmed(), chain);
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    println!();

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Connecting to chain...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Determine chain type
    let is_substrate = endpoint.starts_with("ws://") || endpoint.starts_with("wss://");

    if is_substrate {
        // Substrate chain info
        use subxt::{OnlineClient, PolkadotConfig};

        let api = OnlineClient::<PolkadotConfig>::from_url(endpoint)
            .await
            .context("Failed to connect to Substrate endpoint")?;

        spinner.set_message("Fetching chain data...");

        // Get latest block
        let block = api.blocks().at_latest().await?;
        let block_number = block.number();
        let block_hash = block.hash();

        // Get runtime version
        let runtime_version = api.runtime_version();

        spinner.finish_and_clear();

        println!("{}", "Network Information:".yellow().bold());
        println!("  {}: {}", "Block Height".cyan(), block_number);
        println!("  {}: {}", "Block Hash".dimmed(), block_hash);
        println!();

        println!("{}", "Runtime:".yellow().bold());
        println!(
            "  {}: {}",
            "Spec Version".cyan(),
            runtime_version.spec_version
        );
        println!(
            "  {}: {}",
            "Transaction Version".dimmed(),
            runtime_version.transaction_version
        );
    } else {
        // EVM chain info
        use alloy::providers::{Provider, ProviderBuilder};
        use alloy::rpc::types::BlockNumberOrTag;

        let provider =
            ProviderBuilder::new().connect_http(endpoint.parse().context("Invalid endpoint URL")?);

        spinner.set_message("Fetching chain data...");

        // Get chain ID
        let chain_id = provider.get_chain_id().await?;

        // Get latest block number
        let block_number = provider.get_block_number().await?;

        // Get latest block
        let block = provider
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to fetch latest block"))?;

        spinner.finish_and_clear();

        println!("{}", "Network Information:".yellow().bold());
        println!("  {}: {}", "Chain ID".cyan(), chain_id);
        println!("  {}: {}", "Block Height".cyan(), block_number);
        println!("  {}: {:?}", "Block Hash".dimmed(), block.header.hash);
        println!();

        println!("{}", "Block Details:".yellow().bold());
        println!("  {}: {}", "Timestamp".dimmed(), block.header.timestamp);
        println!("  {}: {}", "Gas Limit".dimmed(), block.header.gas_limit);
        println!("  {}: {}", "Gas Used".dimmed(), block.header.gas_used);
        println!(
            "  {}: {}",
            "Transactions".dimmed(),
            block.transactions.len()
        );

        // Determine network name from chain ID
        let network_name = match chain_id {
            1 => "Ethereum Mainnet",
            5 => "Goerli Testnet",
            11155111 => "Sepolia Testnet",
            137 => "Polygon Mainnet",
            80001 => "Polygon Mumbai",
            56 => "BSC Mainnet",
            97 => "BSC Testnet",
            43114 => "Avalanche C-Chain",
            43113 => "Avalanche Fuji",
            _ => "Unknown Network",
        };

        println!("\n{}: {}", "Network".green().bold(), network_name);
    }

    Ok(())
}

async fn check_chain_health(endpoint: &str) -> anyhow::Result<()> {
    use colored::Colorize;
    use std::time::Instant;

    println!("\n{}", "🏥 Chain Health Check".cyan().bold());
    println!("{}", "═══════════════════════════════════════".dimmed());
    println!("{}: {}", "Endpoint".dimmed(), endpoint);
    println!();

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Checking connection...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Determine chain type
    let is_substrate = endpoint.starts_with("ws://") || endpoint.starts_with("wss://");

    if is_substrate {
        // Substrate health check
        use subxt::{OnlineClient, PolkadotConfig};

        let start = Instant::now();
        let api = OnlineClient::<PolkadotConfig>::from_url(endpoint)
            .await
            .context("Failed to connect to Substrate endpoint")?;

        spinner.set_message("Fetching latest block...");

        // Try to get latest block as a health check
        let block = api.blocks().at_latest().await?;
        let latency = start.elapsed();

        spinner.finish_and_clear();

        println!("{}", "Connection Successful".green().bold());
        println!();
        println!("{}", "Health Metrics:".yellow().bold());
        println!("  {}: {}ms", "Latency".cyan(), latency.as_millis());
        println!("  {}: {}", "Latest Block".dimmed(), block.number());
        println!("  {}: Healthy", "Status".green().bold());
    } else {
        // EVM health check
        use alloy::providers::{Provider, ProviderBuilder};

        let start = Instant::now();
        let provider =
            ProviderBuilder::new().connect_http(endpoint.parse().context("Invalid endpoint URL")?);

        spinner.set_message("Fetching chain data...");

        // Get block number as a health check
        let block_number = provider.get_block_number().await?;

        // Try to get chain ID
        let chain_id = provider.get_chain_id().await?;

        let latency = start.elapsed();

        spinner.finish_and_clear();

        println!("{}", "Connection Successful".green().bold());
        println!();
        println!("{}", "Health Metrics:".yellow().bold());
        println!("  {}: {}ms", "Latency".cyan(), latency.as_millis());
        println!("  {}: {}", "Latest Block".dimmed(), block_number);
        println!("  {}: {}", "Chain ID".dimmed(), chain_id);
        println!("  {}: Healthy", "Status".green().bold());
    }

    Ok(())
}

async fn run_benchmarks(filter: Option<String>) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("bench");
    if let Some(pattern) = filter {
        cmd.arg(pattern);
    }

    let output = cmd.output()?;
    if !output.status.success() {
        anyhow::bail!(
            "Benchmarks failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
