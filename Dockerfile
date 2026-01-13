# Multi-stage build for Apex SDK CLI

# Stage 1: Build
FROM rust:1.85-slim as builder

# Install deps
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY apex-sdk/Cargo.toml apex-sdk/
COPY apex-sdk-core/Cargo.toml apex-sdk-core/
COPY apex-sdk-evm/Cargo.toml apex-sdk-evm/
COPY apex-sdk-substrate/Cargo.toml apex-sdk-substrate/
COPY apex-sdk-types/Cargo.toml apex-sdk-types/
COPY cli/Cargo.toml cli/
COPY integration-tests/Cargo.toml integration-tests/

# Copy source code
COPY apex-sdk apex-sdk
COPY apex-sdk-core apex-sdk-core
COPY apex-sdk-evm apex-sdk-evm
COPY apex-sdk-substrate apex-sdk-substrate
COPY apex-sdk-types apex-sdk-types
COPY cli cli
COPY integration-tests integration-tests

# Build the CLI binary
RUN cargo build --release --bin apex-cli

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 apex

# Copy binary from builder
COPY --from=builder /app/target/release/apex-cli /usr/local/bin/apex

# Set user
USER apex
WORKDIR /home/apex

# Set entrypoint
ENTRYPOINT ["apex"]
CMD ["--help"]
