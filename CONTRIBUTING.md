# Contributing to Apex SDK

We love your input! We want to make contributing to Apex SDK as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

## Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Write documentation for public APIs

## Testing

```bash
# Run all tests
cargo test --all-features

# Run tests for a specific crate
cargo test -p apex-sdk

# Run with logging
RUST_LOG=debug cargo test
```

## Continuous Integration

Our CI/CD pipeline automatically runs on all pull requests and ensures code quality through:

### Automated Checks
- **Code Formatting**: `cargo fmt` ensures consistent code style
- **Linting**: `cargo clippy` catches common mistakes and enforces best practices
- **Testing**: Full test suite runs on Linux, macOS, and Windows
- **Rust Versions**: Tests against stable, beta, nightly, and MSRV (1.75.0)
- **Documentation**: Ensures all public APIs are documented
- **Security Audit**: Scans dependencies for known vulnerabilities with `cargo audit`
- **Code Coverage**: Tracks test coverage with `cargo tarpaulin`
- **Unused Dependencies**: Identifies unused dependencies with `cargo udeps`
- **License Check**: Verifies all dependencies use approved licenses

### Running CI Checks Locally

Before submitting a PR, you can run the same checks locally:

```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --all-features --all-targets -- -D warnings

# Build
cargo build --all-features

# Tests
cargo test --all-features

# Documentation
cargo doc --all-features --no-deps

# Security audit
cargo install cargo-audit
cargo audit

# Coverage (optional)
cargo install cargo-tarpaulin
cargo tarpaulin --all-features --workspace --timeout 300

# Check for unused dependencies
cargo install cargo-udeps
cargo +nightly udeps --all-features
```

### Using Make Commands

We provide convenient Make targets for common tasks:

```bash
make setup    # Install development dependencies
make build    # Build all crates
make test     # Run all tests
make lint     # Run clippy linter
make format   # Format code
make docs     # Generate documentation
make clean    # Clean build artifacts
```

## Pull Request Process

1. Ensure all CI checks pass (they run automatically on your PR)
2. Update the README.md with details of changes if applicable
3. Update the CHANGELOG.md with a note describing your changes
4. Request review from maintainers
5. The PR will be merged once you have the sign-off of at least one maintainer

### CI/CD Status

All pull requests must pass!

Please monitor the status of these checks on your pull request page.

## License

Apache 2.0

## Questions?

Need help or have questions? Here are the best ways to reach us:

- **Discord**: [Join our community](https://discord.gg/zCDFsBaZJN) for real-time help and discussion
- **GitHub Issues**: [Open an issue](https://github.com/eurybits/apex-sdk/issues) for bug reports and feature requests
- **GitHub Discussions**: [Start a discussion](https://github.com/eurybits/apex-sdk/discussions) for general questions and ideas
