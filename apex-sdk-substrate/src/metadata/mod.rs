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
//! apex-sdk-substrate = { version = "0.1.3", features = ["typed-westend"] }
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

// Typed metadata modules
// Note: These files are generated locally and not committed to git due to their size (~4MB each)
// Generate metadata using: ./scripts/generate_metadata.sh <chain-name>
// Uncomment the modules below after generating the metadata files

// #[cfg(feature = "typed-polkadot")]
// #[path = "polkadot.rs"]
// pub mod polkadot;

// #[cfg(feature = "typed-kusama")]
// #[path = "kusama.rs"]
// pub mod kusama;

// #[cfg(feature = "typed-westend")]
// #[path = "westend.rs"]
// pub mod westend;

// #[cfg(feature = "typed")]
// pub mod westend_generated;

// Re-export the most commonly used metadata (when modules are uncommented)
// #[cfg(feature = "typed-westend")]
// pub use westend::*;

// Dynamic API fallback when typed metadata is not available
#[cfg(not(feature = "typed"))]
pub mod dynamic {
    use subxt::dynamic::Value;

    /// Helper to create dynamic runtime calls when typed API is unavailable
    pub fn create_dynamic_call(pallet: &str, call_name: &str) -> &'static str {
        // This is a simplified helper - in practice you'd use subxt's dynamic API
        "dynamic_call_placeholder"
    }
}
