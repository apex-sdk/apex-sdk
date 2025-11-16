//! Typed metadata modules for different Substrate chains
//!
//! This module contains generated Rust types from chain metadata using `subxt codegen`.
//! These types provide compile-time type safety for transactions and storage queries.
//!
//! ## Usage
//!
//! To use typed metadata, enable the corresponding feature flag:
//!
//! ```toml
//! [dependencies]
//! apex-sdk-substrate = { version = "0.1.0", features = ["typed-westend"] }
//! ```
//!
//! Then use the generated types:
//!
//! ```rust,ignore
//! use apex_sdk_substrate::metadata::westend;
//!
//! let tx = westend::tx().balances().transfer_keep_alive(dest, amount);
//! ```
//!
//! ## Generating Metadata
//!
//! Use the provided script to generate metadata for a specific chain:
//!
//! ```bash
//! cd apex-sdk-substrate
//! ./scripts/generate_metadata.sh westend
//! ```
//!
//! See `METADATA_GENERATION.md` for detailed instructions.

// Typed metadata modules are currently disabled
// To enable, generate metadata using scripts/generate_metadata.sh
// and uncomment the appropriate module below

// #[cfg(feature = "typed-polkadot")]
// #[path = "polkadot.rs"]
// pub mod polkadot;

// #[cfg(feature = "typed-kusama")]
// #[path = "kusama.rs"]
// pub mod kusama;

// #[cfg(feature = "typed-westend")]
// #[path = "westend.rs"]
// pub mod westend;
