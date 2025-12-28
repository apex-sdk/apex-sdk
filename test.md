I'll analyze your current test structure and create a comprehensive plan for adding property-based tests, benchmarks, and error injection tests. Let me first examine the current testing landscape. 

Read [](file:///home/eternity/dev/apex-sdk/apex-sdk-evm/tests/pool_connection_tests.rs#1-1), lines 1 to 80

Read [](file:///home/eternity/dev/apex-sdk/benches)

Searched for files matching `**/*.rs`, 20 matches

Read [](file:///home/eternity/dev/apex-sdk/apex-sdk-move/tests/property_tests.rs#1-1), lines 1 to 50

Read [](file:///home/eternity/dev/apex-sdk/Cargo.toml#1-1), lines 1 to 30

Now let me examine the existing benchmark and error injection patterns: 

Searched for regex `criterion|proptest|bench|timeout|network.*fail`, 20 results

Based on my analysis, here's a comprehensive, well-structured plan for adding the three types of advanced testing:

# 🧪 Advanced Testing Implementation Plan

## 📋 Current State Analysis

**✅ Already implemented:**
- Property tests in apex-sdk-move (18 test cases)
- Basic timeout handling in pool connections
- Integration test infrastructure
- `proptest` and `criterion` dependencies configured

**📦 Test dependencies available:**
- `proptest = "1.4"` (property-based testing)
- `criterion = "0.8.1"` (benchmarking)
- `wiremock` (HTTP mocking for error injection)

---

## 🎯 Phase 1: Property-Based Tests (Week 1-2)

### **1.1 EVM Adapter Property Tests**
```rust
// File: apex-sdk-evm/tests/property_tests.rs (NEW)
```

**Test Categories:**
- **Address validation** - Any valid hex string should parse consistently
- **Balance calculations** - Wei ↔ ETH conversions should be reversible
- **Transaction signing** - Same input always produces same signature
- **Gas estimation** - Estimates should be within reasonable bounds
- **RPC responses** - Valid JSON-RPC should always parse correctly

**Implementation Priority:**
1. **Critical Path Tests** (High Priority)
   - Transaction serialization/deserialization
   - Address format validation
   - Balance conversion accuracy

2. **Edge Case Tests** (Medium Priority)
   - Large number handling (U256 boundaries)
   - Empty/null response handling
   - Network timeout edge cases

### **1.2 Substrate Property Tests**
```rust
// File: apex-sdk-substrate/tests/property_tests.rs (NEW)
```

**Test Categories:**
- **SS58 address encoding** - Any valid public key should encode properly
- **Extrinsic construction** - All valid inputs should produce valid extrinsics
- **Metadata parsing** - Valid metadata should always parse consistently
- **Storage key generation** - Same input should always produce same storage key

### **1.3 Core Types Property Tests**
```rust
// File: apex-sdk-types/tests/property_tests.rs (NEW)
```

**Test Categories:**
- **Address conversion** - EVM ↔ Substrate address mappings
- **Transaction status mapping** - Status conversions should be bijective
- **Error type consistency** - Error conversions should preserve information

---

## 🚀 Phase 2: Benchmark Tests (Week 3-4)

### **2.1 Performance Benchmark Structure**
```rust
// File: benches/performance_benchmarks.rs (NEW)
```

**Benchmark Categories:**

#### **2.1.1 Connection Pool Benchmarks**
- Pool creation with varying endpoint counts
- Connection acquisition under load
- Round-robin load balancing performance
- Health check overhead measurement

#### **2.1.2 Transaction Processing Benchmarks** 
- Transaction signing speed
- Batch transaction processing
- Gas estimation performance
- Transaction serialization speed

#### **2.1.3 RPC Client Benchmarks**
- Single RPC call latency
- Concurrent RPC call throughput
- Connection reuse efficiency
- Response parsing speed

### **2.2 Memory Usage Benchmarks**
```rust
// File: benches/memory_benchmarks.rs (NEW)
```

**Memory Test Categories:**
- Connection pool memory overhead
- Transaction cache memory usage
- Metadata storage efficiency
- Memory leak detection over time

### **2.3 Cross-Chain Comparison Benchmarks**
```rust
// File: benches/cross_chain_benchmarks.rs (NEW)
```

**Comparison Categories:**
- EVM vs Substrate transaction speed
- Address conversion performance
- Error handling overhead
- Adapter switching costs

---

## ⚡ Phase 3: Error Injection Tests (Week 5-6)

### **3.1 Network Failure Simulation**
```rust
// File: apex-sdk-evm/tests/error_injection_tests.rs (NEW)
```

#### **3.1.1 Connection-Level Failures**
- **DNS resolution failures**
- **TCP connection timeouts** 
- **SSL/TLS handshake failures**
- **Connection drops mid-request**

#### **3.1.2 HTTP-Level Failures**
- **HTTP 500/502/503/504 errors**
- **Malformed HTTP responses**
- **Partial response data**
- **Response timeout scenarios**

#### **3.1.3 RPC-Level Failures**
- **Invalid JSON-RPC responses**
- **RPC error codes (-32000 series)**
- **Missing required fields**
- **Rate limiting (429 errors)**

### **3.2 Substrate-Specific Error Injection**
```rust
// File: apex-sdk-substrate/tests/error_injection_tests.rs (NEW)
```

#### **3.2.1 WebSocket Failures**
- **WS connection interruption**
- **Subscription feed drops**
- **Binary frame corruption**
- **Ping/pong timeout**

#### **3.2.2 Runtime Errors**
- **Runtime upgrade during call**
- **Extrinsic validation failures**
- **Storage proof verification errors**
- **Metadata version mismatches**

### **3.3 Cross-Chain Error Scenarios**
```rust
// File: integration-tests/tests/error_injection_tests.rs (NEW)
```

#### **3.3.1 Multi-Adapter Failures**
- **Primary adapter down, fallback working**
- **All adapters failing simultaneously**
- **Partial failures during batch operations**
- **State synchronization failures**

---

## 🏗️ Implementation Strategy

### **Phase 1 Implementation (Property Tests)**

**Week 1:**
```bash
# Create property test files
touch apex-sdk-evm/tests/property_tests.rs
touch apex-sdk-substrate/tests/property_tests.rs
touch apex-sdk-types/tests/property_tests.rs
```

**Week 2:**
- Implement 15-20 property tests per module
- Focus on critical data transformations
- Add custom property test generators

### **Phase 2 Implementation (Benchmarks)**

**Week 3:**
```bash
# Create benchmark directory structure
mkdir benches
touch benches/performance_benchmarks.rs
touch benches/memory_benchmarks.rs
touch benches/cross_chain_benchmarks.rs
```

**Week 4:**
- Set up criterion benchmark harness
- Implement performance regression detection
- Add benchmark CI integration

### **Phase 3 Implementation (Error Injection)**

**Week 5:**
```bash
# Create error injection test files
touch apex-sdk-evm/tests/error_injection_tests.rs
touch apex-sdk-substrate/tests/error_injection_tests.rs
touch integration-tests/tests/error_injection_tests.rs
```

**Week 6:**
- Implement chaos testing scenarios
- Add network fault injection
- Create error recovery validation

---

## 📊 Success Metrics

### **Property Test Metrics**
- **Coverage Target:** 95% of critical data transformations
- **Test Count:** 60+ property test cases
- **Execution Time:** <30 seconds for full property test suite

### **Benchmark Metrics**
- **Performance Baseline:** Document current performance characteristics
- **Regression Detection:** 5% performance degradation alerts
- **Memory Usage:** Track memory growth over time

### **Error Injection Metrics**
- **Recovery Rate:** 99%+ successful error recovery
- **Fault Tolerance:** System remains stable under 50%+ failure rates
- **Error Coverage:** Test all major failure modes

---

## 🔧 Implementation Details

### **Directory Structure**
```
apex-sdk/
├── benches/                    # NEW
│   ├── performance_benchmarks.rs
│   ├── memory_benchmarks.rs
│   └── cross_chain_benchmarks.rs
├── apex-sdk-evm/tests/
│   ├── property_tests.rs       # NEW
│   └── error_injection_tests.rs # NEW
├── apex-sdk-substrate/tests/
│   ├── property_tests.rs       # NEW
│   └── error_injection_tests.rs # NEW
├── apex-sdk-types/tests/
│   └── property_tests.rs       # NEW
└── integration-tests/tests/
    └── error_injection_tests.rs # NEW
```

### **New Dependencies**
```toml
# Add to [dev-dependencies]
tokio-test = "0.4"        # For async error injection
mock-instant = "0.3"      # For time-based testing
tempfile = "3.8"          # For temporary file testing
```

### **CI Integration**
```yaml
# Add to .github/workflows/
- property-tests.yml      # Nightly property test runs
- benchmarks.yml          # Performance regression detection  
- chaos-tests.yml         # Weekly error injection testing
```

This plan provides a systematic approach to implementing sophisticated testing that will significantly improve the reliability and performance visibility of your SDK.
