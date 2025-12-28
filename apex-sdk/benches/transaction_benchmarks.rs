use apex_sdk::{transaction::Transaction, ApexSDK};
use apex_sdk_types::{Address, Chain, ChainType};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::time::Duration;

fn benchmark_transaction_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_creation");

    group.bench_function("create_substrate_transfer", |b| {
        b.iter(|| {
            Transaction::builder()
                .from(Address::substrate(
                    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                ))
                .to(Address::substrate(
                    "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                ))
                .amount(1_000_000_000_000) // 1 DOT (12 decimals)
                .chain(Chain::Polkadot)
                .build()
        })
    });

    #[cfg(feature = "evm")]
    group.bench_function("create_evm_transfer", |b| {
        b.iter(|| {
            Transaction::builder()
                .from(Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc"))
                .to(Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc"))
                .amount(1_000_000_000_000_000_000) // 1 ETH (18 decimals)
                .gas_limit(21000)
                .gas_price(20_000_000_000) // 20 gwei
                .chain(Chain::Ethereum)
                .build()
        })
    });

    group.finish();
}

fn benchmark_address_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("address_validation");

    let substrate_addresses = [
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
        "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV",
    ];

    #[cfg(feature = "evm")]
    let evm_addresses = [
        "0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc",
        "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
    ];

    for (i, addr) in substrate_addresses.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("substrate_validation", i),
            addr,
            |b, addr| b.iter(|| Address::substrate(*addr)),
        );
    }

    #[cfg(feature = "evm")]
    for (i, addr) in evm_addresses.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("evm_validation", i), addr, |b, addr| {
            b.iter(|| Address::evm(*addr))
        });
    }

    group.finish();
}

fn benchmark_sdk_initialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("sdk_initialization");

    group.bench_function("create_substrate_sdk", |b| {
        b.iter(|| {
            std::mem::drop(
                ApexSDK::builder()
                    .with_substrate_endpoint("wss://rpc.polkadot.io")
                    .build(),
            );
        })
    });

    #[cfg(feature = "evm")]
    group.bench_function("create_evm_sdk", |b| {
        b.iter(|| {
            std::mem::drop(
                ApexSDK::builder()
                    .with_evm_endpoint("https://eth.llamarpc.com")
                    .build(),
            );
        })
    });

    group.bench_function("create_multi_chain_sdk", |b| {
        b.iter(|| {
            let mut builder = ApexSDK::builder().with_substrate_endpoint("wss://rpc.polkadot.io");

            #[cfg(feature = "evm")]
            {
                builder = builder.with_evm_endpoint("https://eth.llamarpc.com");
            }

            std::mem::drop(builder.build());
        })
    });

    group.finish();
}

fn benchmark_transaction_signing_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_signing");

    // Simulate signing overhead (without actual cryptographic operations)
    group.bench_function("substrate_sign_simulation", |b| {
        b.iter(|| {
            // Simulate signature computation time
            std::thread::sleep(Duration::from_micros(100));
            vec![0u8; 64] // Mock signature
        })
    });

    #[cfg(feature = "evm")]
    group.bench_function("evm_sign_simulation", |b| {
        b.iter(|| {
            // Simulate signature computation time
            std::thread::sleep(Duration::from_micros(80));
            vec![0u8; 65] // Mock signature with recovery id
        })
    });

    group.finish();
}

fn benchmark_amount_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("amount_operations");

    group.bench_function("amount_creation", |b| {
        b.iter(|| {
            1_000_000_000_000_000_000u128 // 1 ETH in wei
        })
    });

    group.bench_function("amount_arithmetic", |b| {
        let amount_a = 100_000_000_000_000_000_000u128; // 100 ETH in wei
        let amount_b = 50_000_000_000_000_000_000u128; // 50 ETH in wei
        b.iter(|| {
            let _ = amount_a + amount_b;
            let _ = amount_a - amount_b;
            let _ = amount_a * 2;
        })
    });

    group.finish();
}

// ============================================================================
// Cross-Chain Operations Benchmarks
// ============================================================================

fn benchmark_cross_chain_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_chain_operations");

    // Define chains for cross-chain benchmarks
    let chains = vec![
        (Chain::Polkadot, ChainType::Substrate),
        (Chain::Kusama, ChainType::Substrate),
        (Chain::Ethereum, ChainType::Evm),
        (Chain::Polygon, ChainType::Evm),
        (Chain::Moonbeam, ChainType::Hybrid),
        (Chain::Astar, ChainType::Hybrid),
    ];

    // Benchmark chain type detection
    group.bench_function("chain_type_detection", |b| {
        b.iter(|| {
            for (chain, _) in &chains {
                black_box(chain.chain_type());
            }
        })
    });

    // Benchmark creating transactions for different chain types
    group.bench_function("multi_chain_transaction_creation", |b| {
        b.iter(|| {
            // Substrate transaction
            black_box(
                Transaction::builder()
                    .from(Address::substrate(
                        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    ))
                    .to(Address::substrate(
                        "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                    ))
                    .amount(1_000_000_000_000)
                    .chain(Chain::Polkadot)
                    .build(),
            );

            #[cfg(feature = "evm")]
            {
                // EVM transaction
                black_box(
                    Transaction::builder()
                        .from(Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc"))
                        .to(Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc"))
                        .amount(1_000_000_000_000_000_000)
                        .gas_limit(21000)
                        .gas_price(20_000_000_000)
                        .chain(Chain::Ethereum)
                        .build(),
                );
            }
        })
    });

    // Benchmark address conversion overhead across chains
    group.bench_function("cross_chain_address_handling", |b| {
        b.iter(|| {
            let substrate_addr =
                Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
            black_box(substrate_addr);

            #[cfg(feature = "evm")]
            {
                let evm_addr = Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc");
                black_box(evm_addr);
            }
        })
    });

    group.finish();
}

fn benchmark_hybrid_chain_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("hybrid_chain_operations");

    // Hybrid chains support both Substrate and EVM
    let hybrid_chains = vec![Chain::Moonbeam, Chain::Astar];

    // Benchmark hybrid chain type checks
    group.bench_function("hybrid_chain_type_check", |b| {
        b.iter(|| {
            for chain in &hybrid_chains {
                let chain_type = chain.chain_type();
                black_box(chain_type == ChainType::Hybrid);
            }
        })
    });

    // Benchmark transaction creation for hybrid chains
    #[cfg(feature = "evm")]
    group.bench_function("hybrid_evm_transaction", |b| {
        b.iter(|| {
            black_box(
                Transaction::builder()
                    .from(Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc"))
                    .to(Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc"))
                    .amount(1_000_000_000_000_000_000)
                    .gas_limit(21000)
                    .gas_price(20_000_000_000)
                    .chain(Chain::Moonbeam)
                    .build(),
            );
        })
    });

    group.finish();
}

fn benchmark_bulk_transaction_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_transaction_creation");

    // Benchmark creating 10 transactions
    group.bench_with_input(BenchmarkId::new("substrate_bulk", 10), &10, |b, &count| {
        b.iter(|| {
            for _ in 0..count {
                black_box(
                    Transaction::builder()
                        .from(Address::substrate(
                            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                        ))
                        .to(Address::substrate(
                            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                        ))
                        .amount(1_000_000_000_000)
                        .chain(Chain::Polkadot)
                        .build(),
                );
            }
        })
    });

    // Benchmark creating 100 transactions
    group.bench_with_input(
        BenchmarkId::new("substrate_bulk", 100),
        &100,
        |b, &count| {
            b.iter(|| {
                for _ in 0..count {
                    black_box(
                        Transaction::builder()
                            .from(Address::substrate(
                                "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                            ))
                            .to(Address::substrate(
                                "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                            ))
                            .amount(1_000_000_000_000)
                            .chain(Chain::Polkadot)
                            .build(),
                    );
                }
            })
        },
    );

    #[cfg(feature = "evm")]
    {
        // Benchmark EVM bulk transactions
        group.bench_with_input(BenchmarkId::new("evm_bulk", 10), &10, |b, &count| {
            b.iter(|| {
                for _ in 0..count {
                    black_box(
                        Transaction::builder()
                            .from(Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc"))
                            .to(Address::evm("0x742d35Cc6634C0532925a3b8D3aC02f1Cfc96bDc"))
                            .amount(1_000_000_000_000_000_000)
                            .gas_limit(21000)
                            .gas_price(20_000_000_000)
                            .chain(Chain::Ethereum)
                            .build(),
                    );
                }
            })
        });
    }

    group.finish();
}

fn benchmark_decimal_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("decimal_conversions");

    // Different chains have different decimal places
    // DOT: 10 decimals
    // KSM: 12 decimals
    // ETH: 18 decimals

    group.bench_function("dot_to_planck", |b| {
        let dot_amount = 1u64;
        b.iter(|| black_box(dot_amount as u128 * 10_000_000_000)) // 10 decimals
    });

    group.bench_function("ksm_to_planck", |b| {
        let ksm_amount = 1u64;
        b.iter(|| black_box(ksm_amount as u128 * 1_000_000_000_000)) // 12 decimals
    });

    group.bench_function("eth_to_wei", |b| {
        let eth_amount = 1u64;
        b.iter(|| black_box(eth_amount as u128 * 1_000_000_000_000_000_000)) // 18 decimals
    });

    // Reverse conversions
    group.bench_function("planck_to_dot", |b| {
        let planck = 10_000_000_000u128;
        b.iter(|| black_box(planck / 10_000_000_000))
    });

    group.bench_function("wei_to_eth", |b| {
        let wei = 1_000_000_000_000_000_000u128;
        b.iter(|| black_box(wei / 1_000_000_000_000_000_000))
    });

    group.finish();
}

fn benchmark_chain_metadata_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("chain_metadata_operations");

    let chains = vec![
        Chain::Polkadot,
        Chain::Kusama,
        Chain::Ethereum,
        Chain::Polygon,
        Chain::Moonbeam,
    ];

    // Benchmark getting default RPC endpoints
    group.bench_function("get_rpc_endpoints", |b| {
        b.iter(|| {
            for chain in &chains {
                black_box(chain.default_endpoint());
            }
        })
    });

    // Benchmark checking smart contract support
    group.bench_function("check_smart_contract_support", |b| {
        b.iter(|| {
            for chain in &chains {
                black_box(chain.supports_smart_contracts());
            }
        })
    });

    // Benchmark chain ID lookups (EVM)
    #[cfg(feature = "evm")]
    group.bench_function("get_chain_ids", |b| {
        let evm_chains = vec![Chain::Ethereum, Chain::Polygon, Chain::Arbitrum];
        b.iter(|| {
            for chain in &evm_chains {
                black_box(chain.chain_id());
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_transaction_creation,
    benchmark_address_validation,
    benchmark_sdk_initialization,
    benchmark_transaction_signing_simulation,
    benchmark_amount_operations,
    benchmark_cross_chain_operations,
    benchmark_hybrid_chain_operations,
    benchmark_bulk_transaction_creation,
    benchmark_decimal_conversions,
    benchmark_chain_metadata_operations,
);

criterion_main!(benches);
