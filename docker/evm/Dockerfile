# Dockerfile for EVM test node (Hardhat)
FROM node:22-alpine

WORKDIR /app

# Install Hardhat and dependencies
RUN npm init -y && \
    npm pkg set type="module" && \
    npm install --save-dev hardhat @nomicfoundation/hardhat-toolbox

# Create Hardhat config
COPY hardhat.config.js ./

# Expose Hardhat RPC port
EXPOSE 8545

# Run Hardhat node
CMD ["npx", "hardhat", "node", "--hostname", "0.0.0.0"]
