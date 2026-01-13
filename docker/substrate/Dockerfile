# Dockerfile for Substrate test node (contracts-node)
FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && \
    apt-get install -y curl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Download substrate-contracts-node binary from releases
# Using v0.42.0 (latest as of 2024)
ARG VERSION=v0.42.0
RUN curl -L "https://github.com/paritytech/substrate-contracts-node/releases/download/${VERSION}/substrate-contracts-node-linux.tar.gz" -o /tmp/substrate-contracts-node.tar.gz && \
    mkdir -p /tmp/substrate && \
    tar -xzf /tmp/substrate-contracts-node.tar.gz -C /tmp/substrate && \
    find /tmp/substrate -type f -name "substrate-contracts-node" -exec mv {} /usr/local/bin/ \; && \
    chmod +x /usr/local/bin/substrate-contracts-node && \
    rm -rf /tmp/substrate-contracts-node.tar.gz /tmp/substrate

# Expose RPC and WebSocket ports
EXPOSE 9944 9933

# Run in development mode with temporary storage
CMD ["substrate-contracts-node", \
     "--dev", \
     "--tmp", \
     "--rpc-external", \
     "--rpc-cors", "all", \
     "--rpc-methods", "Unsafe"]
