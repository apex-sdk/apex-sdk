# Apex SDK System Architecture Document

**Version:** 1.0
**Date:** December 12, 2025
**Status:** Living Document
**Authors:** Khalid Hussein
**Classification:** Public

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Problem Statement & Business Context](#2-problem-statement--business-context)
3. [Requirements](#3-requirements)
4. [Architecture Principles](#4-architecture-principles)
5. [High-Level System Overview](#5-high-level-system-overview)
6. [Component Architecture](#6-component-architecture)
7. [Data Architecture](#7-data-architecture)
8. [Infrastructure & Deployment](#8-infrastructure--deployment)
9. [Security Architecture](#9-security-architecture)
10. [Performance & Scalability](#10-performance--scalability)
11. [Reliability & Observability](#11-reliability--observability)
12. [Error Handling & Resilience](#12-error-handling--resilience)
13. [Technology Stack](#13-technology-stack)
14. [Integration & Interoperability](#14-integration--interoperability)
15. [Risks, Constraints & Assumptions](#15-risks-constraints--assumptions)
16. [Future Evolution](#16-future-evolution)
17. [Appendices](#17-appendices)

---

## 1. Executive Summary

### 1.1 Purpose

This document provides a comprehensive architectural specification for the Apex SDK Protocol, a unified Rust SDK enabling cross-chain development across Substrate and EVM blockchain ecosystems. It serves as the authoritative reference for engineers, stakeholders, and maintainers.

### 1.2 Scope

The architecture covers:
- Six interconnected Rust crates forming the SDK
- CLI tooling for project scaffolding and management
- Integration patterns for 18+ blockchain networks
- Security, performance, and reliability strategies

### 1.3 Audience

| Audience | Focus Areas |
|----------|-------------|
| Engineers | Component details, APIs, data flows, integration patterns |
| DevOps/SRE | Deployment, monitoring, scaling, disaster recovery |
| Security Team | Threat model, controls, compliance requirements |
| Product/Business | Requirements, trade-offs, roadmap alignment |
| External Partners | Integration points, SLAs, API contracts |

### 1.4 Key Architectural Decisions

| Decision | Rationale | Trade-off |
|----------|-----------|-----------|
| Rust as primary language | Memory safety, performance, compile-time guarantees | Steeper learning curve, smaller talent pool |
| Async-first with Tokio | Non-blocking I/O, high concurrency | Complexity in error handling, debugging |
| Adapter pattern for chains | Extensibility, clean abstractions | Additional indirection layer |
| In-memory caching | Low latency, simplicity | No persistence across restarts |
| Library-first deployment | Flexibility, no infrastructure overhead | Users manage their own scaling |

---

## 2. Problem Statement & Business Context

### 2.1 Problem Statement

**Core Challenge:** Blockchain developers face significant friction when building applications that span multiple chain ecosystems.

**Current Pain Points:**

1. **Ecosystem Fragmentation**
   - Substrate and EVM use fundamentally different data models, signing schemes, and APIs
   - Developers must learn multiple SDKs, each with different patterns and conventions
   - Code reuse across ecosystems is minimal

2. **Type Safety Gaps**
   - Many existing SDKs use dynamic typing, leading to runtime errors
   - Transaction encoding errors discovered only at submission time
   - Metadata mismatches cause silent failures

3. **Operational Complexity**
   - Managing connections to multiple chains requires custom pooling logic
   - Error handling patterns differ across ecosystems
   - No unified approach to retries, circuit breaking, or failover

4. **Performance Overhead**
   - JavaScript/Python SDKs introduce unnecessary latency
   - Serialization overhead in cross-language FFI
   - No compile-time optimization opportunities

### 2.2 Business Context

**Market Opportunity:**
- Cross-chain DeFi TVL exceeds $50B
- 500+ parachains and 1000+ EVM chains in production
- Growing demand for multi-chain infrastructure tooling

**Target Users:**

| Segment | Percentage | Key Needs |
|---------|------------|-----------|
| Individual dApp Developers | 70% | Ease of use, documentation, examples |
| Infrastructure Builders | 30% | Performance, reliability, extensibility |
| Enterprise Teams | Future | Compliance, support, SLAs |

**Business Goals:**

1. **Short-term (Q1-Q2 2025)**
   - Achieve 100+ production deployments
   - Reach 1000+ GitHub stars
   - Establish as go-to Substrate+EVM SDK

2. **Medium-term (Q3-Q4 2025)**
   - Enterprise client acquisition
   - Professional security audit certification
   - Multi-language SDK availability

3. **Long-term (2026+)**
   - Industry standard for cross-chain development
   - 10+ blockchain ecosystem support
   - Sustainable open-source funding model

### 2.3 Success Metrics

| Metric | Current | Phase 1 Target | Phase 3 Target |
|--------|---------|----------------|----------------|
| Test Coverage | ~60% | 85% | 95% |
| SDK Init Time | <100ms | <100ms | <50ms |
| Production Deployments | 0 | 10+ | 100+ |
| GitHub Stars | ~100 | 1000+ | 5000+ |
| P0 Bugs in Production | N/A | 0 | 0 |

---

## 3. Requirements

### 3.1 Functional Requirements

#### 3.1.1 Core SDK Capabilities

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-001 | Unified API for Substrate and EVM transactions | Critical | Implemented |
| FR-002 | Compile-time type safety for all chain operations | Critical | Implemented |
| FR-003 | Connection pooling with health monitoring | High | Implemented |
| FR-004 | Transaction building with fee estimation | High | Partial |
| FR-005 | Event subscription and filtering | High | Implemented |
| FR-006 | Batch transaction execution | Medium | Implemented |
| FR-007 | Cross-chain message passing (XCM) | Medium | Partial |

#### 3.1.2 Account Management

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-010 | Key generation (SR25519, ECDSA, Ed25519) | Critical | Implemented |
| FR-011 | BIP-39 mnemonic support | High | Implemented |
| FR-012 | Encrypted key storage | High | Implemented |
| FR-013 | Multi-signature account support | Medium | Planned |
| FR-014 | Hardware wallet integration | Medium | Planned (Phase 3) |

#### 3.1.3 CLI Tooling

| ID | Requirement | Priority | Status |
|----|-------------|----------|--------|
| FR-020 | Project scaffolding with templates | High | Implemented |
| FR-021 | Configuration management | High | Partial |
| FR-022 | Contract deployment | High | Planned |
| FR-023 | Interactive mode | Medium | Planned |
| FR-024 | Shell completions | Low | Planned |

### 3.2 Non-Functional Requirements

#### 3.2.1 Performance

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR-P01 | SDK initialization time | <100ms | Benchmark suite |
| NFR-P02 | Transaction signing latency | <50ms | Benchmark suite |
| NFR-P03 | Transaction submission (sign+broadcast) | <500ms | End-to-end test |
| NFR-P04 | Cached state query | <200ms | Benchmark suite |
| NFR-P05 | Uncached state query | <500ms | Integration test |
| NFR-P06 | Memory usage per connection | <10MB | Profiling |
| NFR-P07 | Throughput per instance | 100+ TPS | Load test |

#### 3.2.2 Scalability

| ID | Requirement | Target | Notes |
|----|-------------|--------|-------|
| NFR-S01 | Concurrent connections per pool | 10 (configurable) | Can scale to 100+ |
| NFR-S02 | Supported chains simultaneously | 18+ | 9 Substrate + 9+ EVM |
| NFR-S03 | Horizontal scaling | Stateless design | Users deploy multiple instances |
| NFR-S04 | Cache size | Configurable LRU | Default 1000 entries |

#### 3.2.3 Reliability

| ID | Requirement | Target | Implementation |
|----|-------------|--------|----------------|
| NFR-R01 | Automatic reconnection | Yes | Exponential backoff |
| NFR-R02 | Connection health checks | 60-second intervals | Configurable |
| NFR-R03 | Circuit breaker pattern | Yes | Configurable thresholds |
| NFR-R04 | Retry with backoff | Yes | Configurable attempts |
| NFR-R05 | Graceful degradation | Yes | Fallback endpoints |

#### 3.2.4 Security

| ID | Requirement | Target | Status |
|----|-------------|--------|--------|
| NFR-SEC01 | Encryption at rest (keys) | AES-256-GCM | Implemented |
| NFR-SEC02 | Secure key derivation | Argon2 | Implemented |
| NFR-SEC03 | Memory protection | Zeroize on drop | Implemented |
| NFR-SEC04 | Input validation | All external inputs | Implemented |
| NFR-SEC05 | Dependency auditing | cargo-audit in CI | Implemented |
| NFR-SEC06 | No secret logging | Log scrubbing | Implemented |

#### 3.2.5 Compliance

| ID | Requirement | Target Date | Status |
|----|-------------|-------------|--------|
| NFR-C01 | SOC 2 Type II | Q3 2025 | Planned |
| NFR-C02 | GDPR compliance | Q3 2025 | Planned |
| NFR-C03 | Security audit (third-party) | Q3 2025 | Planned |

#### 3.2.6 Usability

| ID | Requirement | Target | Measurement |
|----|-------------|--------|-------------|
| NFR-U01 | Time to first transaction | <30 minutes | User testing |
| NFR-U02 | API discoverability | Intuitive naming | Code review |
| NFR-U03 | Error message clarity | Actionable guidance | User feedback |
| NFR-U04 | Documentation coverage | 100% public API | Doc generation |

#### 3.2.7 Maintainability

| ID | Requirement | Target | Enforcement |
|----|-------------|--------|-------------|
| NFR-M01 | Code documentation | All public items | CI check |
| NFR-M02 | Test coverage | 85%+ | CI gate |
| NFR-M03 | Clippy compliance | Zero warnings | CI gate |
| NFR-M04 | Formatting | rustfmt | CI gate |
| NFR-M05 | MSRV (Minimum Rust Version) | 1.85+ | CI matrix |

#### 3.2.8 Observability

| ID | Requirement | Target | Implementation |
|----|-------------|--------|----------------|
| NFR-O01 | Structured logging | All operations | tracing crate |
| NFR-O02 | Metrics endpoint | Prometheus format | Port 9090 |
| NFR-O03 | Distributed tracing | OpenTelemetry | Planned |
| NFR-O04 | Health check endpoint | HTTP /health | Planned |

---

## 4. Architecture Principles

### 4.1 Core Principles

#### 4.1.1 Compile-Time Safety Over Runtime Checks

**Principle:** Catch errors at compile time, not in production.

**Implementation:**
- Strong typing with Rust's type system
- Generic constraints for chain-specific operations
- Metadata-driven type generation

**Trade-off:** Longer compile times, more complex generics

#### 4.1.2 Unified Abstractions, Native Performance

**Principle:** Single API surface without sacrificing chain-specific optimizations.

**Implementation:**
- Core traits define common operations
- Adapters implement chain-specific logic
- Zero-cost abstractions where possible

**Trade-off:** Additional indirection layer, more code to maintain

#### 4.1.3 Async-First, Non-Blocking I/O

**Principle:** Never block threads on I/O operations.

**Implementation:**
- Tokio async runtime
- async/await throughout the codebase
- Connection pooling with async acquisition

**Trade-off:** Complexity in error handling, colored function problem

#### 4.1.4 Fail-Fast with Graceful Recovery

**Principle:** Detect failures early but provide recovery mechanisms.

**Implementation:**
- Comprehensive input validation
- Circuit breakers for external services
- Automatic retry with exponential backoff

**Trade-off:** More complex control flow

#### 4.1.5 Sensible Defaults, Full Configurability

**Principle:** Work out of the box but allow customization.

**Implementation:**
- Builder pattern with defaults
- Configuration hierarchy (code > file > environment)
- Runtime reconfiguration where safe

**Trade-off:** More configuration surface to document and test

### 4.2 Design Patterns

| Pattern | Usage | Rationale |
|---------|-------|-----------|
| **Builder** | SDK, Transaction, Connection construction | Complex objects with many optional parameters |
| **Adapter** | Chain-specific implementations | Decouple abstractions from implementations |
| **Strategy** | Signing algorithms, serialization | Runtime algorithm selection |
| **Circuit Breaker** | External service calls | Prevent cascade failures |
| **Object Pool** | Connection management | Resource reuse, bounded concurrency |
| **Decorator** | Middleware for requests | Cross-cutting concerns (logging, metrics) |

---

## 5. High-Level System Overview

### 5.1 Context Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                         USER APPLICATIONS                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │   dApps     │  │  Indexers   │  │   Oracles   │  │  Wallets    │ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘ │
└─────────┼────────────────┼────────────────┼────────────────┼────────┘
          │                │                │                │
          └────────────────┴────────┬───────┴────────────────┘
                                    │
                    ┌───────────────▼───────────────┐
                    │                               │
                    │         APEX SDK              │
                    │                               │
                    │  ┌─────────────────────────┐  │
                    │  │     Unified API         │  │
                    │  │  (apex-sdk crate)       │  │
                    │  └───────────┬─────────────┘  │
                    │              │                │
                    │  ┌───────────┴─────────────┐  │
                    │  │      Core Traits        │  │
                    │  │  (apex-sdk-core)        │  │
                    │  └───────────┬─────────────┘  │
                    │              │                │
                    │      ┌───────┴───────┐        │
                    │      │               │        │
                    │  ┌───▼───┐       ┌───▼───┐    │
                    │  │Substrate│     │  EVM  │    │
                    │  │Adapter │     │Adapter│    │
                    │  └───┬───┘       └───┬───┘    │
                    │      │               │        │
                    └──────┼───────────────┼────────┘
                           │               │
          ┌────────────────┴───┐       ┌───┴────────────────┐
          │                    │       │                    │
          ▼                    ▼       ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   SUBSTRATE     │  │   SUBSTRATE     │  │      EVM        │
│   NETWORKS      │  │   PARACHAINS    │  │    NETWORKS     │
├─────────────────┤  ├─────────────────┤  ├─────────────────┤
│ • Polkadot      │  │ • Moonbeam      │  │ • Ethereum      │
│ • Kusama        │  │ • Astar         │  │ • Polygon       │
│ • Westend       │  │ • Acala         │  │ • BSC           │
│                 │  │ • Phala         │  │ • Avalanche     │
│                 │  │ • Bifrost       │  │ • Arbitrum      │
│                 │  │                 │  │ • Optimism      │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

### 5.2 System Boundaries

**In Scope:**
- SDK library crates (6 crates)
- CLI tooling for development workflows
- Documentation and examples

**Out of Scope:**
- Hosted RPC infrastructure (users provide endpoints)
- Blockchain nodes (external dependency)
- Application-layer business logic
- User interface components

### 5.3 Key Stakeholders

| Stakeholder | Interests | Concerns |
|-------------|-----------|----------|
| SDK Users | Ease of use, reliability, performance | Breaking changes, documentation gaps |
| Contributors | Code quality, clear architecture | Complexity, build times |
| Security Auditors | Cryptographic correctness, input validation | Key management, dependency risks |
| Blockchain Teams | Correct protocol implementation | Maintenance burden |

---

## 6. Component Architecture

### 6.1 Crate Dependency Graph

```
                    ┌─────────────┐
                    │  apex-sdk   │
                    │   (main)    │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              ▼            ▼            ▼
      ┌───────────┐ ┌───────────┐ ┌───────────┐
      │ substrate │ │    evm    │ │   core    │
      │  adapter  │ │  adapter  │ │  traits   │
      └─────┬─────┘ └─────┬─────┘ └───────────┘
            │             │              ▲
            └──────┬──────┘              │
                   │                     │
                   ▼                     │
            ┌───────────┐                │
            │   types   │ ───────────────┘
            │  (shared) │
            └───────────┘
```

### 6.2 Component Details

#### 6.2.1 apex-sdk (Main Crate)

**Purpose:** Unified entry point providing high-level API for cross-chain operations.

**Responsibilities:**
- SDK initialization and configuration
- Transaction building and execution
- Error recovery and retry logic
- Performance optimizations (pooling, caching, batching)

**Key Modules:**

| Module | Responsibility | Key Types |
|--------|----------------|-----------|
| `sdk` | Main SDK struct and lifecycle | `ApexSDK` |
| `builder` | SDK construction with configuration | `ApexSDKBuilder` |
| `transaction` | Transaction building and execution | `Transaction`, `TransactionBuilder`, `TransactionResult` |
| `error` | Error types and handling | `Error`, `Result` |
| `error_recovery` | Retry and circuit breaker logic | `RetryConfig`, `CircuitBreaker` |
| `performance` | Pooling, caching, rate limiting | `ConnectionPool`, `RateLimiter`, `AsyncMemo` |
| `advanced` | Block/event subscriptions, batching | `BlockSubscription`, `EventSubscription`, `ParallelExecutor` |

**Public API Surface:**

```rust
// SDK Initialization
ApexSDK::builder()
    .with_substrate_endpoint(url)
    .with_evm_endpoint(url)
    .with_timeout(duration)
    .build()
    .await?;

// Transaction Building
sdk.transaction()
    .to(address)
    .value(amount)
    .data(payload)
    .build()
    .sign(&signer)
    .submit()
    .await?;

// Error Recovery
with_retry(RetryConfig::default(), || async {
    sdk.query_balance(address).await
}).await?;
```

**Dependencies:**
- `apex-sdk-core` - Core trait definitions
- `apex-sdk-substrate` - Substrate chain support
- `apex-sdk-evm` - EVM chain support
- `apex-sdk-types` - Shared type definitions
- `tokio` - Async runtime
- `tracing` - Structured logging

---

#### 6.2.2 apex-sdk-core (Core Traits)

**Purpose:** Define abstract interfaces that chain adapters must implement.

**Responsibilities:**
- Chain-agnostic trait definitions
- Common operation signatures
- Abstraction layer between SDK and adapters

**Key Traits:**

```rust
/// Core blockchain operations
#[async_trait]
pub trait BlockchainAdapter: Send + Sync {
    type Address;
    type Balance;
    type Transaction;
    type Receipt;

    async fn get_balance(&self, address: &Self::Address) -> Result<Self::Balance>;
    async fn submit_transaction(&self, tx: Self::Transaction) -> Result<Self::Receipt>;
    async fn get_transaction(&self, hash: &[u8]) -> Result<Option<Self::Receipt>>;
}

/// Wallet and signing operations
#[async_trait]
pub trait WalletAdapter: Send + Sync {
    type Signer;
    type Signature;

    async fn sign(&self, signer: &Self::Signer, message: &[u8]) -> Result<Self::Signature>;
    fn verify(&self, signature: &Self::Signature, message: &[u8], pubkey: &[u8]) -> bool;
}

/// Event subscription
#[async_trait]
pub trait EventAdapter: Send + Sync {
    type Event;
    type Subscription: Stream<Item = Self::Event>;

    async fn subscribe_events(&self, filter: EventFilter) -> Result<Self::Subscription>;
}
```

**Design Rationale:**
- Traits use associated types for chain-specific data
- `async_trait` enables async methods in traits
- `Send + Sync` bounds enable use across threads

---

#### 6.2.3 apex-sdk-substrate (Substrate Adapter)

**Purpose:** Implement core traits for Substrate-based blockchains.

**Responsibilities:**
- WebSocket connection management to Substrate nodes
- SCALE codec encoding/decoding
- Extrinsic construction and submission
- Runtime metadata parsing
- XCM message building

**Key Modules:**

| Module | Responsibility | Key Types |
|--------|----------------|-----------|
| `client` | Substrate RPC client | `SubstrateClient` |
| `wallet` | SR25519/Ed25519 key management | `SubstrateWallet`, `Keypair` |
| `transaction` | Extrinsic building | `SubstrateTransaction` |
| `xcm` | Cross-chain messaging | `XcmBuilder`, `MultiLocation` |
| `contracts` | ink! contract interaction | `ContractCall`, `ContractDeploy` |
| `cache` | Metadata and state caching | `MetadataCache`, `StateCache` |

**Supported Networks:**

| Network | Chain Type | Features |
|---------|------------|----------|
| Polkadot | Relay Chain | Full support |
| Kusama | Relay Chain | Full support |
| Westend | Testnet | Full support |
| Moonbeam | Parachain | Substrate + EVM |
| Astar | Parachain | Substrate + EVM |
| Acala | Parachain | DeFi primitives |
| Phala | Parachain | Privacy features |
| Bifrost | Parachain | Liquid staking |

**Key Implementation Details:**

```rust
pub struct SubstrateClient {
    api: OnlineClient<PolkadotConfig>,
    metadata_cache: Arc<RwLock<MetadataCache>>,
    connection_pool: ConnectionPool,
}

impl SubstrateClient {
    pub async fn connect(endpoint: &str) -> Result<Self> {
        let api = OnlineClient::<PolkadotConfig>::from_url(endpoint).await?;
        // Cache metadata for compile-time type safety
        let metadata = api.metadata();
        Ok(Self {
            api,
            metadata_cache: Arc::new(RwLock::new(MetadataCache::new(metadata))),
            connection_pool: ConnectionPool::new(10),
        })
    }
}
```

**Dependencies:**
- `subxt` - Substrate client library
- `sp-core` - Substrate primitives (crypto, hashing)
- `sp-runtime` - Runtime types
- `parity-scale-codec` - SCALE encoding

---

#### 6.2.4 apex-sdk-evm (EVM Adapter)

**Purpose:** Implement core traits for EVM-compatible blockchains.

**Responsibilities:**
- JSON-RPC connection management
- RLP encoding/decoding
- Transaction construction (legacy, EIP-1559, EIP-2930)
- ABI encoding/decoding
- Gas estimation

**Key Modules:**

| Module | Responsibility | Key Types |
|--------|----------------|-----------|
| `client` | EVM RPC client | `EvmClient` |
| `wallet` | ECDSA key management | `EvmWallet`, `LocalWallet` |
| `transaction` | Transaction building | `EvmTransaction`, `TypedTransaction` |
| `contract` | Smart contract interaction | `Contract`, `ContractCall` |
| `pool` | Connection pooling | `EvmConnectionPool` |

**Supported Networks:**

| Network | Type | Chain ID | Features |
|---------|------|----------|----------|
| Ethereum | L1 | 1 | Full support |
| Polygon | L1 | 137 | Full support |
| BSC | L1 | 56 | Full support |
| Avalanche C-Chain | L1 | 43114 | Full support |
| Arbitrum One | L2 | 42161 | Full support |
| Optimism | L2 | 10 | Full support |
| Base | L2 | 8453 | Full support |
| zkSync Era | L2 | 324 | Planned |

**Key Implementation Details:**

```rust
pub struct EvmClient {
    provider: Provider<Http>,
    chain_id: u64,
    connection_pool: EvmConnectionPool,
}

impl EvmClient {
    pub async fn connect(endpoint: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(endpoint)?;
        let chain_id = provider.get_chainid().await?.as_u64();
        Ok(Self {
            provider,
            chain_id,
            connection_pool: EvmConnectionPool::new(10),
        })
    }

    pub async fn send_transaction(&self, tx: TypedTransaction) -> Result<TxHash> {
        let pending = self.provider.send_transaction(tx, None).await?;
        Ok(pending.tx_hash())
    }
}
```

**Dependencies:**
- `ethers` - Ethereum client library
- `alloy-primitives` - Core EVM types

---

#### 6.2.5 apex-sdk-types (Shared Types)

**Purpose:** Define types shared across all adapters and the main SDK.

**Responsibilities:**
- Chain-agnostic type definitions
- Common enumerations and constants
- Serialization implementations

**Key Types:**

```rust
/// Supported blockchain types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainType {
    Substrate,
    Evm,
}

/// Chain identifier
#[derive(Debug, Clone)]
pub enum Chain {
    // Substrate
    Polkadot,
    Kusama,
    Westend,
    Moonbeam,
    Astar,
    Acala,
    Phala,
    Bifrost,
    // EVM
    Ethereum,
    Polygon,
    Bsc,
    Avalanche,
    Arbitrum,
    Optimism,
    Base,
    ZkSync,
    // Custom
    Custom { chain_type: ChainType, chain_id: u64 },
}

/// Universal address type
#[derive(Debug, Clone)]
pub enum Address {
    Substrate(AccountId32),
    Evm(H160),
}

/// Transaction status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    InBlock { block_hash: Vec<u8> },
    Finalized { block_hash: Vec<u8> },
    Failed { error: String },
}

/// Event from any chain
#[derive(Debug, Clone)]
pub struct ChainEvent {
    pub chain: Chain,
    pub block_number: u64,
    pub event_index: u32,
    pub data: Vec<u8>,
}
```

---

#### 6.2.6 cli (Command-Line Interface)

**Purpose:** Provide developer tooling for project scaffolding and management.

**Responsibilities:**
- Project initialization with templates
- Configuration management
- Account management (planned)
- Contract deployment (planned)

**Command Structure:**

```
apex
├── new <project-name>     # Create new project
│   ├── --template <name>  # Use specific template
│   └── --chain <chain>    # Target chain
├── build                  # Build project
├── test                   # Run tests
├── account                # Account management (planned)
│   ├── create
│   ├── import
│   └── list
├── deploy                 # Contract deployment (planned)
│   ├── --contract <path>
│   └── --network <name>
└── config                 # Configuration (planned)
    ├── show
    ├── set
    └── validate
```

**Template Types:**

| Template | Description | Contents |
|----------|-------------|----------|
| `basic` | Minimal starter | Single-chain, basic transaction |
| `cross-chain` | Multi-chain starter | Substrate + EVM, bridge example |
| `defi` | DeFi application | Swap, liquidity, price feeds |
| `nft` | NFT application | Minting, marketplace |

---

### 6.3 Data Flow Diagrams

#### 6.3.1 Transaction Submission Flow

```
┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐
│  User   │     │   SDK   │     │ Adapter │     │  Pool   │     │  Chain  │
│  Code   │     │  (main) │     │         │     │         │     │  Node   │
└────┬────┘     └────┬────┘     └────┬────┘     └────┬────┘     └────┬────┘
     │               │               │               │               │
     │ build_tx()    │               │               │               │
     ├──────────────►│               │               │               │
     │               │               │               │               │
     │               │ validate()    │               │               │
     │               ├──────────────►│               │               │
     │               │               │               │               │
     │               │◄──────────────┤               │               │
     │               │     Ok        │               │               │
     │               │               │               │               │
     │ sign(&key)    │               │               │               │
     ├──────────────►│               │               │               │
     │               │               │               │               │
     │               │ sign()        │               │               │
     │               ├──────────────►│               │               │
     │               │               │               │               │
     │               │◄──────────────┤               │               │
     │               │  SignedTx     │               │               │
     │               │               │               │               │
     │ submit()      │               │               │               │
     ├──────────────►│               │               │               │
     │               │               │               │               │
     │               │ get_conn()    │               │               │
     │               ├───────────────┼──────────────►│               │
     │               │               │               │               │
     │               │◄──────────────┼───────────────┤               │
     │               │  Connection   │               │               │
     │               │               │               │               │
     │               │ submit()      │               │               │
     │               ├──────────────►│               │               │
     │               │               │               │               │
     │               │               │ send_raw()    │               │
     │               │               ├───────────────┼──────────────►│
     │               │               │               │               │
     │               │               │◄──────────────┼───────────────┤
     │               │               │   TxHash      │               │
     │               │               │               │               │
     │               │◄──────────────┤               │               │
     │               │  TxResult     │               │               │
     │               │               │               │               │
     │◄──────────────┤               │               │               │
     │  TxResult     │               │               │               │
     │               │               │               │               │
```

#### 6.3.2 Event Subscription Flow

```
┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐
│  User   │     │   SDK   │     │ Adapter │     │  Chain  │
│  Code   │     │         │     │         │     │  Node   │
└────┬────┘     └────┬────┘     └────┬────┘     └────┬────┘
     │               │               │               │
     │ subscribe()   │               │               │
     ├──────────────►│               │               │
     │               │               │               │
     │               │ subscribe()   │               │
     │               ├──────────────►│               │
     │               │               │               │
     │               │               │ ws_subscribe()│
     │               │               ├──────────────►│
     │               │               │               │
     │               │               │◄──────────────┤
     │               │               │  Subscription │
     │               │               │               │
     │               │◄──────────────┤               │
     │               │   Stream      │               │
     │               │               │               │
     │◄──────────────┤               │               │
     │   Stream      │               │               │
     │               │               │               │
     │               │               │   Event       │
     │               │               │◄──────────────┤
     │               │               │               │
     │               │   Event       │               │
     │               │◄──────────────┤               │
     │               │               │               │
     │   Event       │               │               │
     │◄──────────────┤               │               │
     │               │               │               │
     │   ...         │               │               │
```

---

### 6.4 API Design

#### 6.4.1 Design Principles

1. **Builder Pattern for Complex Objects**
   - Fluent API for configuration
   - Compile-time validation where possible
   - Sensible defaults

2. **Result Types for All Fallible Operations**
   - No panics in library code
   - Typed errors with context

3. **Async by Default**
   - All I/O operations are async
   - Sync wrappers available where needed

4. **Generic Over Chain-Specific Types**
   - Associated types in traits
   - Monomorphization for performance

#### 6.4.2 Public API Examples

**SDK Initialization:**

```rust
use apex_sdk::prelude::*;

// Minimal initialization
let sdk = ApexSDK::builder()
    .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    .build()
    .await?;

// Full configuration
let sdk = ApexSDK::builder()
    .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
    .with_timeout(Duration::from_secs(30))
    .with_pool_size(20)
    .with_retry_config(RetryConfig {
        max_attempts: 5,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    })
    .with_cache_config(CacheConfig {
        max_entries: 1000,
        ttl: Duration::from_secs(300),
    })
    .build()
    .await?;
```

**Transaction Building:**

```rust
// Substrate transaction
let tx_result = sdk.substrate()
    .transaction()
    .pallet("Balances")
    .call("transfer_keep_alive")
    .arg("dest", recipient)
    .arg("value", amount)
    .build()
    .sign(&keypair)
    .submit()
    .await?;

// EVM transaction
let tx_result = sdk.evm()
    .transaction()
    .to(recipient)
    .value(amount)
    .gas_limit(21000)
    .build()
    .sign(&wallet)
    .submit()
    .await?;
```

**Query Operations:**

```rust
// Balance query
let balance = sdk.substrate()
    .query()
    .storage("System", "Account")
    .key(account_id)
    .fetch()
    .await?;

// Contract call
let result = sdk.evm()
    .contract(contract_address)
    .call("balanceOf")
    .arg(owner)
    .query()
    .await?;
```

**Event Subscription:**

```rust
// Subscribe to transfers
let mut events = sdk.substrate()
    .subscribe_events()
    .filter_pallet("Balances")
    .filter_event("Transfer")
    .build()
    .await?;

while let Some(event) = events.next().await {
    println!("Transfer: {:?}", event);
}
```

---

## 7. Data Architecture

### 7.1 Data Model Overview

The Apex SDK operates primarily as a stateless library, but manages several categories of data:

```
┌─────────────────────────────────────────────────────────┐
│                    DATA CATEGORIES                       │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │  Transient  │  │   Cached    │  │  Encrypted  │     │
│  │   State     │  │   State     │  │   Storage   │     │
│  ├─────────────┤  ├─────────────┤  ├─────────────┤     │
│  │• Connection │  │• Metadata   │  │• Private    │     │
│  │  handles    │  │• Account    │  │  keys       │     │
│  │• Pending    │  │  balances   │  │• Mnemonics  │     │
│  │  requests   │  │• Nonces     │  │• Config     │     │
│  │• Streams    │  │• Gas prices │  │  secrets    │     │
│  └─────────────┘  └─────────────┘  └─────────────┘     │
│                                                         │
│  Lifetime:        Lifetime:        Lifetime:            │
│  Request scope    TTL-based        User-controlled      │
│                   (5 min default)  (persistent)         │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 7.2 Data Types and Schemas

#### 7.2.1 Core Data Types

**Transaction Data:**

```rust
/// Unified transaction representation
pub struct UnifiedTransaction {
    /// Unique identifier
    pub id: TransactionId,
    /// Target chain
    pub chain: Chain,
    /// Transaction payload (chain-specific encoding)
    pub payload: Vec<u8>,
    /// Signature (if signed)
    pub signature: Option<Vec<u8>>,
    /// Metadata
    pub metadata: TransactionMetadata,
}

pub struct TransactionMetadata {
    pub created_at: DateTime<Utc>,
    pub nonce: Option<u64>,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<u128>,
    pub tip: Option<u128>,
}
```

**Account Data:**

```rust
/// Account information
pub struct AccountInfo {
    pub address: Address,
    pub chain: Chain,
    pub balance: Balance,
    pub nonce: u64,
    pub last_updated: DateTime<Utc>,
}

/// Balance with denomination
pub struct Balance {
    pub free: u128,
    pub reserved: u128,
    pub frozen: u128,
    pub decimals: u8,
    pub symbol: String,
}
```

**Cache Entry:**

```rust
/// Cached data with metadata
pub struct CacheEntry<T> {
    pub value: T,
    pub inserted_at: Instant,
    pub ttl: Duration,
    pub access_count: AtomicU64,
}
```

#### 7.2.2 Encrypted Storage Schema

Keys and secrets are stored with AES-256-GCM encryption:

```rust
/// Encrypted key storage format
pub struct EncryptedKeyStore {
    /// Version for format migrations
    pub version: u32,
    /// KDF parameters
    pub kdf: KdfParams,
    /// Encrypted private key
    pub encrypted_key: Vec<u8>,
    /// Nonce for AES-GCM
    pub nonce: [u8; 12],
    /// Key derivation salt
    pub salt: [u8; 32],
}

pub struct KdfParams {
    pub algorithm: String,  // "argon2id"
    pub memory_cost: u32,   // 65536 KB
    pub time_cost: u32,     // 3 iterations
    pub parallelism: u32,   // 4 threads
}
```

### 7.3 Caching Strategy

#### 7.3.1 Cache Architecture

```
┌─────────────────────────────────────────────┐
│              CACHE HIERARCHY                 │
├─────────────────────────────────────────────┤
│                                             │
│  ┌─────────────────────────────────────┐    │
│  │         L1: Request Cache           │    │
│  │   (per-request, microseconds TTL)   │    │
│  └──────────────────┬──────────────────┘    │
│                     │                       │
│  ┌──────────────────▼──────────────────┐    │
│  │         L2: State Cache             │    │
│  │    (shared, configurable TTL)       │    │
│  └──────────────────┬──────────────────┘    │
│                     │                       │
│  ┌──────────────────▼──────────────────┐    │
│  │        L3: Metadata Cache           │    │
│  │   (long-lived, until chain update)  │    │
│  └─────────────────────────────────────┘    │
│                                             │
└─────────────────────────────────────────────┘
```

#### 7.3.2 Cache Configuration

| Cache Type | Default TTL | Max Entries | Eviction Policy |
|------------|-------------|-------------|-----------------|
| State Cache | 5 minutes | 1000 | LRU |
| Metadata Cache | Until upgrade | 100 | Manual invalidation |
| Nonce Cache | 30 seconds | 1000 | TTL |
| Gas Price Cache | 15 seconds | 100 | TTL |

#### 7.3.3 Cache Invalidation

**Automatic Invalidation:**
- TTL expiration (time-based)
- LRU eviction (space-based)
- Chain reorganization detection

**Manual Invalidation:**
- Runtime upgrade detection (metadata)
- User-triggered refresh
- Error-triggered refresh (stale data)

### 7.4 Data Lifecycle

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  Create  │────►│  Cache   │────►│   Use    │────►│  Expire  │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
     │                │                │                │
     │                │                │                │
     ▼                ▼                ▼                ▼
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│ Validate │     │  Index   │     │ Refresh  │     │  Clean   │
│  Input   │     │  by Key  │     │ on Miss  │     │    Up    │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
```

### 7.5 Data Migration Strategy

#### 7.5.1 Version Compatibility

The SDK maintains backward compatibility for:
- Encrypted key stores (version field)
- Configuration files (migration on load)
- Cache formats (clear on version mismatch)

#### 7.5.2 Migration Process

```rust
/// Migrate key store to new format
pub fn migrate_keystore(old: &[u8]) -> Result<EncryptedKeyStore> {
    let version = read_version(old)?;
    match version {
        1 => migrate_v1_to_v2(old),
        2 => Ok(deserialize(old)?),
        _ => Err(Error::UnsupportedVersion(version)),
    }
}
```

### 7.6 Backup and Restore

#### 7.6.1 What to Back Up

| Data | Location | Backup Method |
|------|----------|---------------|
| Private keys | `~/.apex/keys/` | User responsibility, export mnemonic |
| Configuration | `~/.apex/config.toml` | File copy |
| Cache | In-memory | Not persisted (ephemeral) |

#### 7.6.2 Key Export/Import

```rust
// Export mnemonic (for backup)
let mnemonic = wallet.export_mnemonic()?;

// Import from mnemonic (for restore)
let wallet = SubstrateWallet::from_mnemonic(&mnemonic, password)?;
```

---

## 8. Infrastructure & Deployment

### 8.1 Deployment Model

The Apex SDK is primarily deployed as a library embedded in user applications:

```
┌─────────────────────────────────────────────────────────┐
│              USER APPLICATION DEPLOYMENT                 │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────────────────────────────────────┐    │
│  │              User Application                    │    │
│  │  ┌─────────────────────────────────────────┐    │    │
│  │  │           Apex SDK (embedded)            │    │    │
│  │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ │    │    │
│  │  │  │ Substrate│ │   EVM    │ │   Core   │ │    │    │
│  │  │  └──────────┘ └──────────┘ └──────────┘ │    │    │
│  │  └─────────────────────────────────────────┘    │    │
│  └─────────────────────────────────────────────────┘    │
│                          │                              │
│                          ▼                              │
│  ┌─────────────────────────────────────────────────┐    │
│  │              External Services                   │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐        │    │
│  │  │   RPC    │ │ Explorer │ │  Indexer │        │    │
│  │  │  Nodes   │ │   APIs   │ │ Services │        │    │
│  │  └──────────┘ └──────────┘ └──────────┘        │    │
│  └─────────────────────────────────────────────────┘    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 8.2 Distribution Channels

| Channel | Purpose | Update Frequency |
|---------|---------|------------------|
| crates.io | Primary distribution | Every release |
| GitHub Releases | Source archives, binaries | Every release |
| Docker Hub | CLI container images | Every release |

### 8.3 Environment Configuration

#### 8.3.1 Configuration Hierarchy

```
Priority (highest to lowest):
1. Runtime configuration (code)
2. Environment variables
3. Configuration file (~/.apex/config.toml)
4. Default values
```

#### 8.3.2 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `APEX_SUBSTRATE_ENDPOINT` | Substrate RPC URL | None |
| `APEX_EVM_ENDPOINT` | EVM RPC URL | None |
| `APEX_LOG_LEVEL` | Logging verbosity | `info` |
| `APEX_CACHE_TTL` | Cache TTL in seconds | `300` |
| `APEX_POOL_SIZE` | Connection pool size | `10` |
| `APEX_TIMEOUT` | Request timeout (ms) | `30000` |

#### 8.3.3 Configuration File Format

```toml
# ~/.apex/config.toml

[substrate]
endpoint = "wss://polkadot.api.onfinality.io/public-ws"
timeout = 30000

[evm]
endpoint = "https://mainnet.infura.io/v3/YOUR_KEY"
chain_id = 1

[performance]
pool_size = 10
cache_ttl = 300
cache_max_entries = 1000

[retry]
max_attempts = 3
initial_backoff_ms = 100
max_backoff_ms = 10000
backoff_multiplier = 2.0

[logging]
level = "info"
format = "json"
```

### 8.4 CI/CD Pipeline

#### 8.4.1 Pipeline Overview

```
┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐
│  Push   │────►│  Build  │────►│  Test   │────►│ Release │────►│ Publish │
└─────────┘     └─────────┘     └─────────┘     └─────────┘     └─────────┘
                     │               │               │               │
                     ▼               ▼               ▼               ▼
                ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐
                │ Clippy  │     │  Unit   │     │  Tag    │     │crates.io│
                │ Format  │     │ Integ   │     │ GitHub  │     │ Docker  │
                │  Audit  │     │ Bench   │     │ Release │     │  Hub    │
                └─────────┘     └─────────┘     └─────────┘     └─────────┘
```

#### 8.4.2 GitHub Actions Workflows

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `ci.yml` | Push, PR | Build, test, lint |
| `security.yml` | Push, schedule | Dependency audit, SAST |
| `benchmarks.yml` | Push to main | Performance regression |
| `release.yml` | Tag push | Publish to crates.io |
| `docs.yml` | Push to main | Generate documentation |

#### 8.4.3 CI Quality Gates

| Gate | Tool | Threshold |
|------|------|-----------|
| Formatting | rustfmt | Zero diff |
| Linting | clippy | Zero warnings |
| Tests | cargo test | 100% pass |
| Coverage | cargo-tarpaulin | 85% minimum |
| Security | cargo-audit | Zero vulnerabilities |
| MSRV | cargo build | Rust 1.85+ |

### 8.5 Container Support

#### 8.5.1 Dockerfile (CLI)

```dockerfile
FROM rust:1.85-slim as builder
WORKDIR /usr/src/apex
COPY . .
RUN cargo build --release --package apex-cli

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/apex/target/release/apex /usr/local/bin/
ENTRYPOINT ["apex"]
```

#### 8.5.2 Docker Compose (Development)

```yaml
version: '3.8'
services:
  dev:
    build: .
    volumes:
      - .:/workspace
      - cargo-cache:/usr/local/cargo/registry
    environment:
      - RUST_LOG=debug
      - APEX_SUBSTRATE_ENDPOINT=ws://substrate:9944
      - APEX_EVM_ENDPOINT=http://ganache:8545
    depends_on:
      - substrate
      - ganache

  substrate:
    image: parity/polkadot:latest
    command: --dev --ws-external

  ganache:
    image: trufflesuite/ganache:latest
    command: --deterministic

volumes:
  cargo-cache:
```

### 8.6 Disaster Recovery

#### 8.6.1 Recovery Scenarios

| Scenario | Impact | Recovery |
|----------|--------|----------|
| RPC node failure | Service degradation | Automatic failover to backup endpoints |
| SDK crash | Application restart | Stateless design enables clean restart |
| Key corruption | Loss of funds access | Restore from mnemonic backup |
| Cache corruption | Stale data | Clear cache, rebuild from chain |

#### 8.6.2 Failover Configuration

```rust
// Multiple endpoints with failover
let sdk = ApexSDK::builder()
    .with_substrate_endpoints(vec![
        "wss://polkadot.api.onfinality.io/public-ws",
        "wss://rpc.polkadot.io",
        "wss://polkadot-rpc.dwellir.com",
    ])
    .with_failover_strategy(FailoverStrategy::RoundRobin)
    .build()
    .await?;
```

### 8.7 Network Architecture

For users deploying applications with the SDK:

```
┌─────────────────────────────────────────────────────────────┐
│                    RECOMMENDED DEPLOYMENT                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                    VPC / Network                     │    │
│  │                                                      │    │
│  │  ┌──────────────┐      ┌──────────────┐             │    │
│  │  │   App + SDK  │      │   App + SDK  │             │    │
│  │  │  Instance 1  │      │  Instance 2  │             │    │
│  │  └───────┬──────┘      └───────┬──────┘             │    │
│  │          │                      │                    │    │
│  │          └──────────┬───────────┘                    │    │
│  │                     │                                │    │
│  │          ┌──────────▼───────────┐                    │    │
│  │          │    NAT Gateway /     │                    │    │
│  │          │    Proxy (optional)  │                    │    │
│  │          └──────────┬───────────┘                    │    │
│  │                     │                                │    │
│  └─────────────────────┼────────────────────────────────┘    │
│                        │                                     │
│            ┌───────────┴───────────┐                         │
│            │                       │                         │
│      ┌─────▼─────┐           ┌─────▼─────┐                   │
│      │  Public   │           │  Private  │                   │
│      │   RPC     │           │   RPC     │                   │
│      │  Nodes    │           │  Nodes    │                   │
│      └───────────┘           └───────────┘                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 9. Security Architecture

### 9.1 Security Principles

1. **Defense in Depth**: Multiple layers of security controls
2. **Least Privilege**: Minimal permissions required for operations
3. **Secure Defaults**: Safe configuration out of the box
4. **Fail Secure**: Errors don't compromise security

### 9.2 Threat Model

#### 9.2.1 Assets to Protect

| Asset | Sensitivity | Impact if Compromised |
|-------|-------------|----------------------|
| Private keys | Critical | Loss of funds |
| Mnemonics | Critical | Loss of all derived keys |
| RPC credentials | High | Unauthorized API access |
| Transaction data | Medium | Privacy breach |
| Configuration | Low | Service disruption |

#### 9.2.2 Threat Actors

| Actor | Capability | Motivation |
|-------|------------|------------|
| External attacker | Network access, public exploits | Financial gain |
| Malicious dependency | Code execution | Supply chain attack |
| Insider threat | Full access | Data theft, sabotage |
| Compromised RPC | Manipulated responses | Transaction manipulation |

#### 9.2.3 Attack Vectors

| Vector | Threat | Mitigation |
|--------|--------|------------|
| Memory disclosure | Key extraction | Zeroize on drop |
| Dependency vulnerability | Code execution | cargo-audit, minimal deps |
| Man-in-the-middle | Transaction tampering | TLS verification |
| Replay attack | Double spending | Nonce management |
| Input injection | Arbitrary execution | Strict validation |

### 9.3 Authentication & Authorization

#### 9.3.1 Key Management

**Supported Algorithms:**

| Algorithm | Use Case | Security Level |
|-----------|----------|----------------|
| SR25519 | Substrate signing | 128-bit |
| Ed25519 | Substrate signing | 128-bit |
| ECDSA (secp256k1) | EVM signing | 128-bit |

**Key Derivation:**

```rust
// BIP-39 mnemonic generation
let mnemonic = Mnemonic::generate(24)?;  // 256-bit entropy

// Key derivation with Argon2
let derived_key = derive_key(
    password,
    salt,
    Argon2Params {
        memory_cost: 65536,  // 64 MB
        time_cost: 3,
        parallelism: 4,
    }
)?;
```

#### 9.3.2 Signing Flow

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│ Message  │────►│  Hash    │────►│   Sign   │────►│ Signature│
└──────────┘     └──────────┘     └──────────┘     └──────────┘
                      │                │
                      ▼                ▼
                 ┌──────────┐     ┌──────────┐
                 │ Blake2b  │     │ Private  │
                 │ Keccak   │     │   Key    │
                 └──────────┘     └──────────┘
```

### 9.4 Encryption

#### 9.4.1 Encryption at Rest

**Private Key Storage:**

```rust
/// Encrypt private key for storage
pub fn encrypt_key(
    private_key: &[u8],
    password: &str,
) -> Result<EncryptedKeyStore> {
    // Generate random salt and nonce
    let salt = generate_salt();
    let nonce = generate_nonce();

    // Derive encryption key from password
    let key = argon2_derive(password, &salt)?;

    // Encrypt with AES-256-GCM
    let cipher = Aes256Gcm::new(&key);
    let encrypted = cipher.encrypt(&nonce, private_key)?;

    Ok(EncryptedKeyStore {
        version: 2,
        kdf: KdfParams::argon2id_default(),
        encrypted_key: encrypted,
        nonce,
        salt,
    })
}
```

#### 9.4.2 Encryption in Transit

| Connection Type | Protocol | Verification |
|----------------|----------|--------------|
| Substrate RPC | WSS (TLS 1.3) | Certificate validation |
| EVM RPC | HTTPS (TLS 1.3) | Certificate validation |
| Custom endpoints | Configurable | User responsibility |

### 9.5 Security Controls

#### 9.5.1 Input Validation

```rust
/// Validate all external inputs
pub fn validate_address(input: &str, chain: Chain) -> Result<Address> {
    match chain.chain_type() {
        ChainType::Substrate => {
            // Validate SS58 format
            let decoded = bs58::decode(input)
                .into_vec()
                .map_err(|_| Error::InvalidAddress)?;

            // Check length and checksum
            if decoded.len() != 35 {
                return Err(Error::InvalidAddress);
            }

            verify_ss58_checksum(&decoded)?;

            Ok(Address::Substrate(AccountId32::from_slice(&decoded[1..33])?))
        }
        ChainType::Evm => {
            // Validate hex format
            let bytes = hex::decode(input.trim_start_matches("0x"))
                .map_err(|_| Error::InvalidAddress)?;

            if bytes.len() != 20 {
                return Err(Error::InvalidAddress);
            }

            Ok(Address::Evm(H160::from_slice(&bytes)))
        }
    }
}
```

#### 9.5.2 Memory Protection

```rust
use zeroize::Zeroize;

/// Private key with automatic zeroization
pub struct SecretKey {
    inner: [u8; 32],
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.inner.zeroize();
    }
}

impl Zeroize for SecretKey {
    fn zeroize(&mut self) {
        self.inner.zeroize();
    }
}
```

#### 9.5.3 Logging Scrubbing

```rust
/// Scrub sensitive data from logs
pub fn scrub_sensitive(input: &str) -> String {
    let patterns = [
        (r"0x[a-fA-F0-9]{64}", "[PRIVATE_KEY]"),
        (r"[a-zA-Z]{12,24}\s+[a-zA-Z]+", "[MNEMONIC]"),
        (r"password\s*=\s*\S+", "password=[REDACTED]"),
    ];

    let mut output = input.to_string();
    for (pattern, replacement) in patterns {
        let re = Regex::new(pattern).unwrap();
        output = re.replace_all(&output, replacement).to_string();
    }
    output
}
```

### 9.6 Dependency Security

#### 9.6.1 Dependency Policy

- **Minimal dependencies**: Only include what's necessary
- **Vetted sources**: Prefer well-maintained, audited crates
- **Pinned versions**: Lock file for reproducible builds
- **Regular audits**: Weekly `cargo-audit` in CI

#### 9.6.2 Supply Chain Security

| Control | Implementation | Frequency |
|---------|----------------|-----------|
| Dependency audit | cargo-audit | Every CI run |
| License check | cargo-deny | Every CI run |
| SBOM generation | cargo-sbom | Every release |
| Vulnerability scanning | GitHub Dependabot | Continuous |

### 9.7 Security Checklist

**Development:**
- [ ] All inputs validated
- [ ] No secrets in logs
- [ ] Memory zeroized for keys
- [ ] TLS certificates verified
- [ ] Dependencies audited

**Deployment:**
- [ ] Private keys encrypted at rest
- [ ] Configuration secrets in secure storage
- [ ] Network access restricted
- [ ] Monitoring for anomalies

**Operations:**
- [ ] Regular key rotation
- [ ] Incident response plan
- [ ] Security patches applied promptly

---

## 10. Performance & Scalability

### 10.1 Performance Targets

| Metric | Target | Current | Notes |
|--------|--------|---------|-------|
| SDK initialization | <100ms | ~80ms | Measured with cached metadata |
| Transaction signing | <50ms | ~30ms | SR25519/ECDSA |
| Transaction submission | <500ms | ~400ms | Including RPC round-trip |
| Cached query | <200ms | ~50ms | Local cache hit |
| Uncached query | <500ms | ~300ms | RPC round-trip |
| Memory per connection | <10MB | ~5MB | Including buffers |
| Throughput (single instance) | 100+ TPS | ~150 TPS | Benchmark conditions |

### 10.2 Performance Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  PERFORMANCE OPTIMIZATIONS                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │  Connection │  │   Request   │  │   Compute   │         │
│  │   Pooling   │  │   Batching  │  │   Caching   │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                │                │                 │
│         ▼                ▼                ▼                 │
│  ┌─────────────────────────────────────────────────┐       │
│  │             Async I/O (Tokio)                    │       │
│  └─────────────────────────────────────────────────┘       │
│                          │                                  │
│                          ▼                                  │
│  ┌─────────────────────────────────────────────────┐       │
│  │              Zero-Copy Parsing                   │       │
│  └─────────────────────────────────────────────────┘       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 10.3 Connection Pooling

#### 10.3.1 Pool Configuration

```rust
pub struct PoolConfig {
    /// Maximum connections per endpoint
    pub max_connections: usize,       // Default: 10
    /// Minimum idle connections
    pub min_idle: usize,              // Default: 1
    /// Connection timeout
    pub connect_timeout: Duration,     // Default: 30s
    /// Idle timeout before closing
    pub idle_timeout: Duration,        // Default: 5m
    /// Health check interval
    pub health_check_interval: Duration, // Default: 60s
}
```

#### 10.3.2 Pool Behavior

```
┌─────────────────────────────────────────────────────┐
│                CONNECTION POOL                       │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Request arrives                                    │
│       │                                             │
│       ▼                                             │
│  ┌─────────┐  Yes  ┌─────────┐                     │
│  │  Idle   │──────►│  Return │                     │
│  │  conn?  │       │  conn   │                     │
│  └────┬────┘       └─────────┘                     │
│       │ No                                          │
│       ▼                                             │
│  ┌─────────┐  Yes  ┌─────────┐                     │
│  │  Pool   │──────►│  Create │                     │
│  │  full?  │       │   new   │                     │
│  └────┬────┘       └─────────┘                     │
│       │ No                                          │
│       ▼                                             │
│  ┌─────────┐                                        │
│  │  Wait   │                                        │
│  │  queue  │                                        │
│  └─────────┘                                        │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 10.4 Caching Strategy

#### 10.4.1 Cache Layers

| Layer | Purpose | TTL | Size |
|-------|---------|-----|------|
| Metadata | Runtime metadata | Until upgrade | Per chain |
| State | Account balances, nonces | 5 minutes | 1000 entries |
| Gas | Gas price estimates | 15 seconds | 100 entries |

#### 10.4.2 Async Memoization

```rust
/// Memoize expensive async computations
pub struct AsyncMemo<K, V> {
    cache: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    ttl: Duration,
}

impl<K: Hash + Eq + Clone, V: Clone> AsyncMemo<K, V> {
    pub async fn get_or_compute<F, Fut>(&self, key: K, f: F) -> Result<V>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<V>>,
    {
        // Check cache first
        if let Some(entry) = self.cache.read().await.get(&key) {
            if !entry.is_expired() {
                return Ok(entry.value.clone());
            }
        }

        // Compute and cache
        let value = f().await?;
        self.cache.write().await.insert(key, CacheEntry::new(value.clone(), self.ttl));
        Ok(value)
    }
}
```

### 10.5 Request Batching

#### 10.5.1 Batch Configuration

```rust
pub struct BatchConfig {
    /// Maximum requests per batch
    pub max_batch_size: usize,        // Default: 100
    /// Maximum wait time for batch
    pub max_batch_wait: Duration,     // Default: 10ms
    /// Execute batch when full or timeout
    pub eager_execution: bool,        // Default: true
}
```

#### 10.5.2 Batch Execution

```rust
/// Execute multiple operations in parallel
pub async fn batch_execute<T, F, Fut>(
    operations: Vec<F>,
    config: BatchConfig,
) -> Vec<Result<T>>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let semaphore = Arc::new(Semaphore::new(config.max_batch_size));

    let futures: Vec<_> = operations
        .into_iter()
        .map(|op| {
            let permit = semaphore.clone();
            async move {
                let _permit = permit.acquire().await;
                op().await
            }
        })
        .collect();

    join_all(futures).await
}
```

### 10.6 Rate Limiting

#### 10.6.1 Rate Limiter Configuration

```rust
pub struct RateLimiter {
    /// Maximum requests per window
    pub max_requests: usize,
    /// Window duration
    pub window: Duration,
    /// Current state
    state: Arc<Mutex<RateLimiterState>>,
}
```

#### 10.6.2 Usage

```rust
let limiter = RateLimiter::new(100, Duration::from_secs(1));

// Will wait if rate limit exceeded
limiter.acquire().await?;
let result = sdk.query_balance(address).await?;
```

### 10.7 Scalability Strategy

#### 10.7.1 Horizontal Scaling

Since the SDK is stateless (except for in-memory cache), horizontal scaling is straightforward:

```
┌───────────────────────────────────────────────────┐
│              HORIZONTAL SCALING                    │
├───────────────────────────────────────────────────┤
│                                                   │
│  ┌─────────┐   ┌─────────┐   ┌─────────┐         │
│  │ App+SDK │   │ App+SDK │   │ App+SDK │         │
│  │   #1    │   │   #2    │   │   #N    │         │
│  └────┬────┘   └────┬────┘   └────┬────┘         │
│       │             │             │               │
│       └─────────────┼─────────────┘               │
│                     │                             │
│            ┌────────▼────────┐                    │
│            │  Load Balancer  │                    │
│            │ (user-managed)  │                    │
│            └────────┬────────┘                    │
│                     │                             │
│            ┌────────▼────────┐                    │
│            │   RPC Nodes     │                    │
│            └─────────────────┘                    │
│                                                   │
└───────────────────────────────────────────────────┘
```

#### 10.7.2 Vertical Scaling

Increase resources for single instance:

| Resource | Scaling Effect |
|----------|----------------|
| CPU cores | More parallel operations |
| Memory | Larger cache, more connections |
| Network | Higher throughput |

### 10.8 Performance Benchmarks

#### 10.8.1 Benchmark Suite

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench transaction_bench
```

#### 10.8.2 Benchmark Results (Reference)

| Benchmark | Iterations | Time (avg) | Time (p99) |
|-----------|------------|------------|------------|
| SDK initialization | 1000 | 78ms | 120ms |
| SR25519 signing | 10000 | 25μs | 45μs |
| ECDSA signing | 10000 | 180μs | 250μs |
| SCALE encoding | 10000 | 2μs | 5μs |
| RLP encoding | 10000 | 1.5μs | 3μs |
| Cache lookup | 100000 | 50ns | 200ns |

### 10.9 Bottleneck Analysis

| Bottleneck | Cause | Mitigation |
|------------|-------|------------|
| RPC latency | Network round-trip | Connection pooling, batching |
| Signing | Cryptographic computation | Parallel signing, caching |
| Serialization | Data encoding | Zero-copy parsing |
| Memory allocation | Frequent allocations | Object pooling, arena allocators |

---

## 11. Reliability & Observability

### 11.1 Reliability Patterns

#### 11.1.1 Circuit Breaker

```rust
pub struct CircuitBreaker {
    /// Number of failures before opening
    pub failure_threshold: u32,
    /// Duration to stay open
    pub recovery_timeout: Duration,
    /// Number of successes to close
    pub success_threshold: u32,
    /// Current state
    state: AtomicState,
}

pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub async fn call<F, T>(&self, f: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        match self.state.get() {
            CircuitState::Open => {
                if self.should_attempt_reset() {
                    self.state.set(CircuitState::HalfOpen);
                } else {
                    return Err(Error::CircuitOpen);
                }
            }
            _ => {}
        }

        match f.await {
            Ok(result) => {
                self.record_success();
                Ok(result)
            }
            Err(e) => {
                self.record_failure();
                Err(e)
            }
        }
    }
}
```

#### 11.1.2 Retry with Backoff

```rust
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Initial backoff duration
    pub initial_backoff: Duration,
    /// Maximum backoff duration
    pub max_backoff: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Jitter factor (0.0 - 1.0)
    pub jitter: f64,
}

pub async fn with_retry<F, T, Fut>(config: RetryConfig, f: F) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut attempts = 0;
    let mut backoff = config.initial_backoff;

    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < config.max_attempts && e.is_retryable() => {
                attempts += 1;
                let jitter = rand::random::<f64>() * config.jitter;
                let sleep_duration = backoff.mul_f64(1.0 + jitter);
                tokio::time::sleep(sleep_duration).await;
                backoff = std::cmp::min(
                    backoff.mul_f64(config.backoff_multiplier),
                    config.max_backoff,
                );
            }
            Err(e) => return Err(e),
        }
    }
}
```

#### 11.1.3 Health Checks

```rust
pub struct HealthCheck {
    /// Check interval
    pub interval: Duration,
    /// Timeout for health check
    pub timeout: Duration,
    /// Consecutive failures before unhealthy
    pub failure_threshold: u32,
}

impl ConnectionPool {
    async fn health_check_loop(&self) {
        let mut interval = tokio::time::interval(self.health_check.interval);

        loop {
            interval.tick().await;

            for conn in self.connections.read().await.iter() {
                let result = tokio::time::timeout(
                    self.health_check.timeout,
                    conn.ping(),
                ).await;

                match result {
                    Ok(Ok(_)) => conn.mark_healthy(),
                    _ => conn.mark_unhealthy(),
                }
            }

            // Remove unhealthy connections
            self.evict_unhealthy().await;
        }
    }
}
```

### 11.2 Observability Stack

```
┌─────────────────────────────────────────────────────────────┐
│                   OBSERVABILITY STACK                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌───────────┐     ┌───────────┐     ┌───────────┐         │
│  │  Metrics  │     │  Logging  │     │  Tracing  │         │
│  │(Prometheus)│    │ (tracing) │     │  (future) │         │
│  └─────┬─────┘     └─────┬─────┘     └─────┬─────┘         │
│        │                 │                 │                │
│        └─────────────────┼─────────────────┘                │
│                          │                                  │
│               ┌──────────▼──────────┐                       │
│               │   Export/Collect    │                       │
│               └──────────┬──────────┘                       │
│                          │                                  │
│          ┌───────────────┼───────────────┐                  │
│          │               │               │                  │
│    ┌─────▼─────┐   ┌─────▼─────┐   ┌─────▼─────┐           │
│    │  Grafana  │   │    ELK    │   │   Jaeger  │           │
│    │ Dashboard │   │   Stack   │   │  (future) │           │
│    └───────────┘   └───────────┘   └───────────┘           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 11.3 Metrics

#### 11.3.1 Available Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `apex_requests_total` | Counter | `chain`, `method`, `status` | Total requests |
| `apex_request_duration_seconds` | Histogram | `chain`, `method` | Request latency |
| `apex_connections_active` | Gauge | `chain`, `endpoint` | Active connections |
| `apex_cache_hits_total` | Counter | `cache_type` | Cache hit count |
| `apex_cache_misses_total` | Counter | `cache_type` | Cache miss count |
| `apex_circuit_breaker_state` | Gauge | `service` | Circuit breaker state |
| `apex_retry_attempts_total` | Counter | `operation` | Retry attempts |

#### 11.3.2 Metrics Endpoint

```rust
// Enable metrics
let sdk = ApexSDK::builder()
    .with_metrics_endpoint("0.0.0.0:9090")
    .build()
    .await?;

// Metrics available at http://localhost:9090/metrics
```

#### 11.3.3 Grafana Dashboard

Key panels:
- Request rate by chain
- Error rate by method
- Latency percentiles (p50, p95, p99)
- Connection pool utilization
- Cache hit ratio
- Circuit breaker status

### 11.4 Logging

#### 11.4.1 Log Levels

| Level | Use Case | Examples |
|-------|----------|----------|
| ERROR | Unrecoverable failures | Connection failed, invalid signature |
| WARN | Recoverable issues | Retry triggered, cache miss |
| INFO | Normal operations | Transaction submitted, connection established |
| DEBUG | Diagnostic details | Request/response payloads |
| TRACE | Granular tracing | Function entry/exit |

#### 11.4.2 Structured Logging

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(self), fields(chain = %chain))]
pub async fn submit_transaction(&self, tx: Transaction) -> Result<TxHash> {
    info!(tx_hash = %tx.hash(), "Submitting transaction");

    match self.adapter.submit(tx).await {
        Ok(hash) => {
            info!(tx_hash = %hash, "Transaction submitted successfully");
            Ok(hash)
        }
        Err(e) => {
            error!(error = %e, "Transaction submission failed");
            Err(e)
        }
    }
}
```

#### 11.4.3 Log Configuration

```rust
// JSON format for production
tracing_subscriber::fmt()
    .json()
    .with_env_filter("apex_sdk=info,apex_sdk_substrate=debug")
    .init();

// Pretty format for development
tracing_subscriber::fmt()
    .pretty()
    .with_env_filter("debug")
    .init();
```

### 11.5 Distributed Tracing (Planned)

#### 11.5.1 Trace Propagation

```rust
// Future implementation
pub async fn submit_transaction(&self, tx: Transaction) -> Result<TxHash> {
    let span = tracing::info_span!(
        "submit_transaction",
        otel.kind = "client",
        tx.hash = %tx.hash(),
    );

    async move {
        // Transaction submission with trace context
    }
    .instrument(span)
    .await
}
```

#### 11.5.2 Cross-Chain Tracing

Trace a transaction across multiple chains:
1. Start span in SDK
2. Propagate context to RPC call
3. Correlate with chain events
4. Track until finalization

### 11.6 Alerting

#### 11.6.1 Recommended Alerts

| Alert | Condition | Severity | Response |
|-------|-----------|----------|----------|
| High error rate | Error rate > 5% for 5 min | Critical | Check RPC endpoints |
| Connection pool exhausted | Active connections = max | Warning | Increase pool size |
| High latency | p99 > 2s for 10 min | Warning | Check network, endpoints |
| Circuit breaker open | State = open | Warning | Check downstream service |
| Low cache hit ratio | Hit ratio < 50% for 30 min | Info | Review cache config |

#### 11.6.2 Alertmanager Configuration

```yaml
groups:
  - name: apex_sdk
    rules:
      - alert: HighErrorRate
        expr: |
          sum(rate(apex_requests_total{status="error"}[5m])) /
          sum(rate(apex_requests_total[5m])) > 0.05
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"

      - alert: CircuitBreakerOpen
        expr: apex_circuit_breaker_state == 1
        for: 1m
        labels:
          severity: warning
        annotations:
          summary: "Circuit breaker is open"
```

### 11.7 SLOs and SLAs

#### 11.7.1 Service Level Objectives

| SLO | Target | Measurement |
|-----|--------|-------------|
| Availability | 99.9% | Successful requests / total requests |
| Latency (p50) | <200ms | Transaction submission time |
| Latency (p99) | <1s | Transaction submission time |
| Error rate | <0.1% | Error responses / total responses |

#### 11.7.2 Error Budget

Monthly error budget for 99.9% availability:
- Allowed downtime: 43.2 minutes
- Allowed errors: 0.1% of requests

---

## 12. Error Handling & Resilience

### 12.1 Error Taxonomy

#### 12.1.1 Error Categories

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Connection errors (retryable)
    #[error("Connection failed: {0}")]
    Connection(#[from] ConnectionError),

    // Transaction errors (may be retryable)
    #[error("Transaction failed: {0}")]
    Transaction(#[from] TransactionError),

    // Validation errors (not retryable)
    #[error("Validation failed: {0}")]
    Validation(String),

    // Cryptographic errors (not retryable)
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    // Configuration errors (not retryable)
    #[error("Configuration error: {0}")]
    Config(String),

    // Chain-specific errors
    #[error("Substrate error: {0}")]
    Substrate(#[from] SubstrateError),

    #[error("EVM error: {0}")]
    Evm(#[from] EvmError),
}

impl Error {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Connection(_) => true,
            Error::Transaction(e) => e.is_retryable(),
            _ => false,
        }
    }
}
```

#### 12.1.2 Transaction-Specific Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Nonce too low")]
    NonceTooLow,

    #[error("Nonce too high")]
    NonceTooHigh,

    #[error("Gas price too low")]
    GasPriceTooLow,

    #[error("Transaction underpriced")]
    Underpriced,

    #[error("Execution reverted: {0}")]
    Reverted(String),

    #[error("Timeout waiting for confirmation")]
    Timeout,
}

impl TransactionError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            TransactionError::NonceTooLow |
            TransactionError::Timeout |
            TransactionError::Underpriced
        )
    }
}
```

### 12.2 Error Propagation

#### 12.2.1 Error Context

```rust
use anyhow::Context;

pub async fn get_balance(&self, address: &Address) -> Result<Balance> {
    self.client
        .query_balance(address)
        .await
        .context(format!("Failed to get balance for {}", address))
}
```

#### 12.2.2 Error Transformation

```rust
// Convert chain-specific errors to unified errors
impl From<subxt::Error> for Error {
    fn from(e: subxt::Error) -> Self {
        match e {
            subxt::Error::Rpc(rpc_err) => {
                Error::Connection(ConnectionError::Rpc(rpc_err.to_string()))
            }
            subxt::Error::Metadata(meta_err) => {
                Error::Substrate(SubstrateError::Metadata(meta_err.to_string()))
            }
            _ => Error::Substrate(SubstrateError::Other(e.to_string())),
        }
    }
}
```

### 12.3 Failure Modes

| Failure Mode | Cause | Detection | Recovery |
|--------------|-------|-----------|----------|
| RPC unavailable | Node down, network issue | Connection timeout | Failover to backup |
| RPC rate limited | Too many requests | 429 response | Backoff, rate limit |
| Invalid nonce | Concurrent transactions | Nonce error | Refresh nonce, retry |
| Insufficient gas | Gas estimation wrong | Execution reverted | Increase gas, retry |
| Chain reorganization | Fork resolution | Block hash mismatch | Wait for finalization |

### 12.4 Resilience Patterns

#### 12.4.1 Bulkhead Pattern

Isolate failures to prevent cascade:

```rust
pub struct Bulkhead {
    /// Maximum concurrent requests per partition
    semaphores: HashMap<String, Arc<Semaphore>>,
}

impl Bulkhead {
    pub async fn execute<F, T>(&self, partition: &str, f: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        let semaphore = self.semaphores
            .get(partition)
            .ok_or(Error::UnknownPartition)?;

        let _permit = semaphore
            .acquire()
            .await
            .map_err(|_| Error::BulkheadRejected)?;

        f.await
    }
}
```

#### 12.4.2 Timeout Pattern

Prevent indefinite waiting:

```rust
pub async fn with_timeout<F, T>(duration: Duration, f: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    tokio::time::timeout(duration, f)
        .await
        .map_err(|_| Error::Timeout)?
}
```

#### 12.4.3 Fallback Pattern

Provide degraded service:

```rust
pub async fn get_gas_price(&self) -> Result<u128> {
    // Try primary source
    if let Ok(price) = self.oracle.get_gas_price().await {
        return Ok(price);
    }

    // Fallback to cached value
    if let Some(cached) = self.gas_cache.get() {
        return Ok(cached);
    }

    // Final fallback to default
    Ok(self.default_gas_price)
}
```

### 12.5 Graceful Degradation

| Scenario | Degraded Behavior |
|----------|-------------------|
| Primary RPC down | Use backup endpoint |
| Cache unavailable | Direct queries (slower) |
| Gas oracle down | Use cached/default price |
| Metrics endpoint down | Continue without metrics |

### 12.6 Chaos Engineering (Future)

Planned failure injection for testing:

```rust
#[cfg(feature = "chaos")]
pub struct ChaosConfig {
    /// Probability of injecting failure
    pub failure_rate: f64,
    /// Artificial latency
    pub latency: Duration,
    /// Simulate connection drops
    pub drop_connections: bool,
}
```

---

## 13. Technology Stack

### 13.1 Core Technologies

| Category | Technology | Version | Rationale |
|----------|------------|---------|-----------|
| Language | Rust | 1.85+ | Memory safety, performance, type system |
| Async Runtime | Tokio | 1.35+ | Industry standard, mature, performant |
| Serialization | Serde | 1.0 | De facto standard, wide ecosystem |
| Error Handling | thiserror/anyhow | 2.0/1.0 | Ergonomic, typed errors |

### 13.2 Blockchain Dependencies

#### 13.2.1 Substrate

| Crate | Version | Purpose |
|-------|---------|---------|
| subxt | 0.44.0 | Substrate client library |
| sp-core | 38.1.0 | Primitives (crypto, hashing) |
| sp-runtime | 44.0.0 | Runtime types |

**Rationale:** Official Parity libraries ensure compatibility with latest Substrate features.

**Trade-offs:**
- Heavy dependencies (long compile times)
- Frequent breaking changes
- Tight coupling to Substrate versions

#### 13.2.2 EVM

| Crate | Version | Purpose |
|-------|---------|---------|
| ethers | 2.0 | Ethereum client library |
| alloy-primitives | 1.4.1 | Core types |

**Rationale:** ethers-rs is the most mature and widely-used Rust Ethereum library.

**Trade-offs:**
- Large dependency tree
- alloy ecosystem is emerging (future migration consideration)

### 13.3 Utility Libraries

| Crate | Version | Purpose |
|-------|---------|---------|
| tracing | 0.1 | Structured logging |
| hex | 0.4 | Hex encoding |
| bs58 | 0.5 | Base58 encoding |
| url | 2.5 | URL parsing |
| rand | 0.9.2 | Random number generation |

### 13.4 Development Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| proptest | 1.4 | Property-based testing |
| criterion | 0.7.0 | Benchmarking |
| mockall | 0.13.1 | Mocking framework |

### 13.5 Technology Decisions & Trade-offs

#### 13.5.1 Rust vs Other Languages

| Consideration | Rust | Go | TypeScript |
|---------------|------|----|----|
| Memory safety | Compile-time | GC | GC |
| Performance | Native | Native | JIT |
| Type safety | Strong | Moderate | Strong (with TS) |
| Ecosystem | Growing | Mature | Very mature |
| Hire-ability | Difficult | Easy | Very easy |
| Binary size | Large | Large | N/A (runtime) |

**Decision:** Rust chosen for memory safety and performance in cryptographic operations.

**Trade-off:** Smaller developer pool, steeper learning curve.

#### 13.5.2 Async vs Sync

| Consideration | Async | Sync |
|---------------|-------|------|
| I/O performance | High | Low |
| Code complexity | Higher | Lower |
| Debugging | Harder | Easier |
| Ecosystem | Good | Universal |

**Decision:** Async-first for high-performance blockchain I/O.

**Trade-off:** Colored function problem, complex error handling.

#### 13.5.3 Library vs Service

| Consideration | Library | Service |
|---------------|---------|---------|
| Deployment | Embedded | Standalone |
| Scaling | User responsibility | Central |
| Latency | Lower | Higher (network hop) |
| Maintenance | User updates | Central updates |

**Decision:** Library-first for flexibility and minimal infrastructure.

**Trade-off:** Users must manage their own scaling and updates.

### 13.6 Dependency Policy

1. **Minimize dependencies**: Only include what's necessary
2. **Prefer well-maintained crates**: Check activity, downloads, security
3. **Pin major versions**: Use `^` semver for predictable updates
4. **Audit regularly**: Weekly `cargo-audit` in CI
5. **Document rationale**: Explain why each dependency is needed

### 13.7 Future Technology Considerations

| Technology | Status | Timeline | Rationale |
|------------|--------|----------|-----------|
| Alloy (EVM) | Evaluating | Q2 2025 | Modern, modular EVM library |
| OpenTelemetry | Planned | Q2 2025 | Standardized observability |
| WebAssembly | Exploring | Q4 2025 | Browser/edge deployment |

---

## 14. Integration & Interoperability

### 14.1 External System Integration

```
┌─────────────────────────────────────────────────────────────┐
│                EXTERNAL INTEGRATIONS                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐                           ┌─────────────┐  │
│  │  RPC Nodes  │                           │  Explorers  │  │
│  │  (Primary)  │                           │   (APIs)    │  │
│  └──────┬──────┘                           └──────┬──────┘  │
│         │                                         │         │
│         │    ┌─────────────────────────┐          │         │
│         └───►│                         │◄─────────┘         │
│              │       APEX SDK          │                    │
│         ┌───►│                         │◄─────────┐         │
│         │    └─────────────────────────┘          │         │
│         │                                         │         │
│  ┌──────┴──────┐                           ┌──────┴──────┐  │
│  │  Indexers   │                           │   Oracles   │  │
│  │  (Future)   │                           │  (Future)   │  │
│  └─────────────┘                           └─────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 14.2 RPC Node Integration

#### 14.2.1 Supported RPC Methods

**Substrate:**
- `author_submitExtrinsic`
- `chain_getBlockHash`
- `chain_getFinalizedHead`
- `state_getStorage`
- `state_getMetadata`
- `system_health`

**EVM:**
- `eth_sendRawTransaction`
- `eth_getTransactionReceipt`
- `eth_getBalance`
- `eth_call`
- `eth_estimateGas`
- `eth_gasPrice`

#### 14.2.2 RPC Provider Compatibility

| Provider | Type | Status | Notes |
|----------|------|--------|-------|
| OnFinality | Substrate | Supported | Free tier available |
| Dwellir | Substrate | Supported | Enterprise focus |
| Infura | EVM | Supported | Requires API key |
| Alchemy | EVM | Supported | Requires API key |
| QuickNode | Both | Supported | Enterprise focus |
| Ankr | Both | Supported | Free tier available |

### 14.3 Block Explorer Integration

#### 14.3.1 Supported Explorers

| Explorer | Chain | Features |
|----------|-------|----------|
| Subscan | Substrate | Account, transaction, event lookup |
| Polkascan | Substrate | Basic integration |
| Etherscan | EVM | Transaction, contract verification |
| Polygonscan | Polygon | Transaction lookup |
| BscScan | BSC | Transaction lookup |

#### 14.3.2 API Usage

```rust
// Get transaction details from explorer
let tx_details = sdk.explorer()
    .etherscan("YOUR_API_KEY")
    .get_transaction(tx_hash)
    .await?;
```

### 14.4 Indexer Integration (Planned)

#### 14.4.1 The Graph

```rust
// Future API
let results = sdk.indexer()
    .the_graph("https://api.thegraph.com/subgraphs/name/...")
    .query(r#"
        {
            transfers(first: 10, orderBy: timestamp, orderDirection: desc) {
                from
                to
                value
            }
        }
    "#)
    .await?;
```

#### 14.4.2 SubQuery

```rust
// Future API
let results = sdk.indexer()
    .subquery("https://api.subquery.network/sq/...")
    .query("SELECT * FROM transfers ORDER BY block_height DESC LIMIT 10")
    .await?;
```

### 14.5 Oracle Integration (Planned)

#### 14.5.1 Chainlink

```rust
// Future API
let price = sdk.oracle()
    .chainlink()
    .get_price("ETH/USD")
    .await?;
```

### 14.6 Wallet Integration

#### 14.6.1 Current Support

| Wallet | Type | Status |
|--------|------|--------|
| MetaMask | Browser (EVM) | Supported via user signing |
| Polkadot.js | Browser (Substrate) | Supported via user signing |

#### 14.6.2 Planned Support

| Wallet | Type | Timeline |
|--------|------|----------|
| WalletConnect | Multi-chain | Q2 2025 |
| Ledger | Hardware | Q3 2025 |
| Trezor | Hardware | Q3 2025 |

### 14.7 Protocol Integrations

#### 14.7.1 Cross-Chain Protocols

| Protocol | Purpose | Status |
|----------|---------|--------|
| XCM | Polkadot cross-chain | Partial |
| IBC | Cosmos interoperability | Planned (Q4 2025) |

#### 14.7.2 DeFi Protocols (Future)

| Protocol | Purpose | Timeline |
|----------|---------|----------|
| Uniswap | DEX integration | Q3 2025 |
| Aave | Lending integration | Q4 2025 |
| Acala | Substrate DeFi | Q2 2025 |

### 14.8 Integration Patterns

#### 14.8.1 Request/Response

Standard RPC pattern for queries and transactions:

```rust
// Simple request/response
let balance = sdk.get_balance(address).await?;
```

#### 14.8.2 Pub/Sub

Event subscription pattern:

```rust
// Subscribe to events
let mut events = sdk.subscribe_events(filter).await?;
while let Some(event) = events.next().await {
    handle_event(event);
}
```

#### 14.8.3 Webhook (Future)

Push notifications for events:

```rust
// Register webhook (future)
sdk.webhooks()
    .register("https://my-app.com/webhook", EventFilter::all())
    .await?;
```

---

## 15. Risks, Constraints & Assumptions

### 15.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Blockchain protocol breaking changes | High | High | Version pinning, comprehensive tests, migration guides |
| Dependency vulnerabilities | Medium | High | Regular audits, minimal dependencies, quick patches |
| Performance regression | Medium | Medium | Continuous benchmarking, performance budgets |
| Memory leaks | Low | High | Extensive testing, fuzzing, address sanitizer |
| Cryptographic implementation flaws | Low | Critical | Use audited libraries, security review |

### 15.2 Operational Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| RPC provider outages | High | Medium | Multiple endpoints, automatic failover |
| Rate limiting by providers | High | Low | Rate limiting, request batching |
| Chain reorganizations | Medium | Medium | Wait for finalization, reorg detection |
| Key compromise | Low | Critical | Encryption, secure key management, HSM support |

### 15.3 Business Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Competing solutions | High | Medium | Focus on unique value, community building |
| Blockchain ecosystem changes | Medium | High | Flexible architecture, modular design |
| Funding gaps | Medium | High | Diversified funding, grants, sponsorships |
| Key personnel loss | Medium | Medium | Documentation, knowledge sharing |

### 15.4 Constraints

#### 15.4.1 Technical Constraints

| Constraint | Description | Impact |
|------------|-------------|--------|
| Rust MSRV | Minimum Rust 1.85 | Limits user base |
| Async runtime | Tokio only | Lock-in to ecosystem |
| No WASM support | Cannot run in browser | Web deployment limited |
| Platform support | Linux/macOS/Windows | No mobile support |

#### 15.4.2 Resource Constraints

| Constraint | Description | Impact |
|------------|-------------|--------|
| Team size | 2-3 developers | Feature velocity limited |
| Budget | Limited funding | No paid infrastructure |
| Testing infrastructure | No live testnet | Integration testing limited |

#### 15.4.3 External Constraints

| Constraint | Description | Impact |
|------------|-------------|--------|
| RPC rate limits | Provider-imposed limits | Throughput ceiling |
| Chain finality times | Network-specific | Confirmation latency |
| Regulatory uncertainty | Evolving regulations | Compliance scope unclear |

### 15.5 Assumptions

#### 15.5.1 Technical Assumptions

| Assumption | Rationale | Risk if Invalid |
|------------|-----------|-----------------|
| RPC nodes are available | Public infrastructure exists | Service unavailable |
| Blockchain protocols are stable | Mature networks | Breaking changes |
| Rust ecosystem continues growing | Current trajectory | Harder to maintain |
| Async Rust is production-ready | Wide adoption | Reliability issues |

#### 15.5.2 Business Assumptions

| Assumption | Rationale | Risk if Invalid |
|------------|-----------|-----------------|
| Cross-chain demand grows | Market trends | Limited adoption |
| Developer community engages | Open-source model | No contributors |
| Enterprise demand materializes | Phase 3 roadmap | Revenue impact |

#### 15.5.3 User Assumptions

| Assumption | Rationale | Risk if Invalid |
|------------|-----------|-----------------|
| Users have Rust knowledge | Target audience | Poor adoption |
| Users manage their own infra | Library model | Support burden |
| Users handle key security | Responsibility model | Security incidents |

### 15.6 Risk Mitigation Strategy

**Priority 1 (Critical):**
- Cryptographic security review
- Dependency vulnerability scanning
- Key management best practices documentation

**Priority 2 (High):**
- Comprehensive integration testing
- Performance benchmarking
- Multi-endpoint failover

**Priority 3 (Medium):**
- Community building
- Documentation improvements
- Enterprise readiness

---

## 16. Future Evolution

### 16.1 Roadmap Summary

```
┌─────────────────────────────────────────────────────────────┐
│                    EVOLUTION ROADMAP                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Q1 2025           Q2 2025           Q3 2025           Q4+  │
│  ────────          ────────          ────────          ──── │
│                                                             │
│  ┌─────────┐      ┌─────────┐      ┌─────────┐      ┌─────┐ │
│  │ Phase 1 │      │ Phase 2 │      │ Phase 3 │      │  4  │ │
│  │Foundation│─────│Ecosystem│──────│Enterprise│─────│Innov│ │
│  │Stabilize│      │Expansion│      │ Ready   │      │ation│ │
│  └─────────┘      └─────────┘      └─────────┘      └─────┘ │
│                                                             │
│  • CLI complete   • Parachains     • Security audit  • Cosmos│
│  • 85% coverage   • L2 networks    • Enterprise feat • Solana│
│  • Docs updated   • Oracle integ   • Multi-language  • AI/ML │
│  • Bug fixes      • Performance    • Compliance      • Intent│
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 16.2 Phase 1: Foundation Stabilization (Q1 2025)

**Duration:** 8-12 weeks
**Goal:** Production-ready for early adopters

**Key Deliverables:**
- Complete CLI implementation (account, config, deploy)
- 85%+ test coverage
- Updated documentation reflecting actual functionality
- Zero P0 bugs

**Success Metrics:**
- 10+ production deployments
- <100ms SDK initialization
- All documented features working

### 16.3 Phase 2: Ecosystem Expansion (Q2 2025)

**Duration:** 12-16 weeks
**Goal:** Broad blockchain support

**Key Deliverables:**
- Full parachain support (Moonbeam, Astar, Acala, Phala, Bifrost)
- L2 network support (Arbitrum, Optimism, Base, zkSync)
- Oracle integration (Chainlink)
- Performance optimizations

**Success Metrics:**
- 5+ parachains supported
- 3+ L2 networks supported
- 50+ production deployments
- 1000+ GitHub stars

### 16.4 Phase 3: Enterprise Ready (Q3 2025)

**Duration:** 12 weeks
**Goal:** Enterprise-grade reliability

**Key Deliverables:**
- Professional third-party security audit
- Hardware wallet support (Ledger, Trezor)
- Multi-language SDKs (TypeScript, Python)
- Enterprise features (RBAC, audit logs, team management)
- Compliance (SOC 2, GDPR)

**Success Metrics:**
- Security audit passed
- Enterprise clients onboarded
- 99.9% availability for production apps
- Multi-language SDKs released

### 16.5 Phase 4: Innovation (Q4 2025+)

**Duration:** Ongoing
**Goal:** Cutting-edge features

**Key Initiatives:**
- Multi-ecosystem support (Cosmos/IBC, Solana, Near)
- AI/ML integration (gas optimization, anomaly detection)
- Intent-based transactions
- MEV protection
- Decentralized RPC network

### 16.6 Extensibility Points

The architecture is designed for extension in these areas:

#### 16.6.1 New Chain Support

```rust
// Implement core traits for new chain
pub struct CosmosAdapter {
    // ...
}

impl BlockchainAdapter for CosmosAdapter {
    type Address = CosmosAddress;
    type Balance = CosmosBalance;
    // ...
}
```

#### 16.6.2 Custom Middleware

```rust
// Add custom middleware to request pipeline
sdk.add_middleware(LoggingMiddleware::new());
sdk.add_middleware(MetricsMiddleware::new());
sdk.add_middleware(CustomAuthMiddleware::new());
```

#### 16.6.3 Plugin System (Future)

```rust
// Load plugins dynamically
sdk.load_plugin("my-custom-plugin")?;
```

### 16.7 Backward Compatibility

**Commitment:**
- Semantic versioning strictly followed
- Breaking changes only in major versions
- Deprecation warnings for 2 minor versions
- LTS versions supported for 1 year
- Migration guides for all breaking changes

**API Stability:**

| Component | Stability | Notes |
|-----------|-----------|-------|
| Core SDK API | Stable | No breaking changes in 1.x |
| CLI commands | Stable | Backward compatible |
| Configuration format | Stable | Migration support |
| Internal traits | Unstable | May change |

### 16.8 Migration Strategy

For major version upgrades:

1. **Announce**: 3 months before release
2. **Preview**: Beta releases for testing
3. **Document**: Comprehensive migration guide
4. **Codemods**: Automated migration tools where possible
5. **Support**: Extended support for previous major version

---

## 17. Appendices

### 17.1 Glossary

| Term | Definition |
|------|------------|
| **Adapter** | Component implementing core traits for specific blockchain |
| **Circuit Breaker** | Pattern to prevent cascade failures |
| **Extrinsic** | Substrate transaction |
| **Finality** | Irreversible block confirmation |
| **LRU** | Least Recently Used (cache eviction) |
| **Nonce** | Transaction sequence number |
| **RPC** | Remote Procedure Call |
| **SCALE** | Substrate codec |
| **TTL** | Time To Live |
| **XCM** | Cross-Consensus Message Format |

### 17.2 Reference Documents

| Document | Location | Description |
|----------|----------|-------------|
| API Reference | `docs/API.md` | Complete API documentation |
| CLI Guide | `docs/CLI_GUIDE.md` | Command-line usage |
| Quick Start | `docs/QUICK_START.md` | Getting started guide |
| Security | `docs/SECURITY.md` | Security policies |
| Roadmap | `docs/ROADMAP.md` | Development roadmap |
| Testing | `docs/TESTING_FRAMEWORK.md` | Testing approach |

### 17.3 Decision Log

| Date | Decision | Rationale | Alternatives Considered |
|------|----------|-----------|------------------------|
| 2025-10 | Use Rust | Memory safety, performance | Go, TypeScript |
| 2025-10 | Tokio runtime | Maturity, ecosystem | async-std |
| 2025-10 | Library-first | Flexibility | Hosted service |
| 2025-11 | subxt for Substrate | Official support | substrate-api-client |
| 2025-11 | ethers for EVM | Maturity | web3, alloy |
| 2025-11 | In-memory cache | Simplicity | Redis, RocksDB |

### 17.4 Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-12-12 | System Architecture Team | Initial version |

---

## Document Approval

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Technical Lead | | | |
| Security Review | | | |
| Product Owner | | | |

---

**End of Document**
