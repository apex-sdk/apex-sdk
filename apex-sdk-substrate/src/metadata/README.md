# Metadata Directory

This directory contains chain-specific metadata for typed API usage.

## Generated Files (Not in Git)

The following files are **generated locally** and **not committed** to git:

- `polkadot.rs` (~4MB) - Polkadot mainnet metadata
- `kusama.rs` (~4MB) - Kusama mainnet metadata
- `westend.rs` (~4MB) - Westend testnet metadata
- `westend_generated.rs` (~4MB) - Alternative westend metadata

These files are:
- ✅ Generated using `./scripts/generate_metadata.sh <chain>`
- ✅ Listed in `.gitignore` to prevent accidental commits
- ✅ Chain and runtime-version specific
- ✅ Need regeneration when the runtime upgrades

## Why Not Commit Them?

1. **Size**: ~4MB each, would bloat the repository
2. **Updates**: Runtime upgrades require new metadata
3. **Developer-specific**: Different devs may need different chains
4. **CI/CD**: Should be generated as part of build process when needed

## How to Generate

```bash
# From apex-sdk-substrate directory
./scripts/generate_metadata.sh westend
./scripts/generate_metadata.sh polkadot
./scripts/generate_metadata.sh kusama
```

## Using Generated Metadata

After generating metadata files, **uncomment the corresponding module** in `mod.rs`:

```rust
// Uncomment after running: ./scripts/generate_metadata.sh westend
#[cfg(feature = "typed-westend")]
#[path = "westend.rs"]
pub mod westend;
```

Then enable the feature in your `Cargo.toml`:

```toml
apex-sdk-substrate = { version = "0.1.4", features = ["typed-westend"] }
```

## Stub Files (In Git)

These small files **are** committed:
- `mod.rs` - Module declarations and documentation
- `.gitkeep` - Ensures the directory exists in git
- `README.md` - This file

## See Also

- [`METADATA_GENERATION.md`](../../../docs/METADATA_GENERATION.md) - Full guide
- [`generate_metadata.sh`](../../scripts/generate_metadata.sh) - Generation script
