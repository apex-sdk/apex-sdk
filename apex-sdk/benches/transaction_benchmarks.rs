use apex_sdk::{transaction::Transaction, ApexSDK};
use apex_sdk_types::{Address, Chain};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
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

criterion_group!(
    benches,
    benchmark_transaction_creation,
    benchmark_address_validation,
    benchmark_sdk_initialization,
    benchmark_transaction_signing_simulation,
    benchmark_amount_operations
);

criterion_main!(benches);
