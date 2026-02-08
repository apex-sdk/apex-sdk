window.BENCHMARK_DATA = {
  "lastUpdate": 1770586764073,
  "repoUrl": "https://github.com/apex-sdk/apex-sdk",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ba5a09a4f47d39848b72043c29c4d3df8f2a12a8",
          "message": "chore: update docs and fix ci (#42)\n\n* docs: update broken links\n\n* fix(benches): update file existence check before moving output.txt\n\n* docs: update README",
          "timestamp": "2025-11-24T23:13:48-05:00",
          "tree_id": "ec7b1d20782d2b040d28026553c6db82f6a1f6ec",
          "url": "https://github.com/kherldhussein/apex-sdk/commit/ba5a09a4f47d39848b72043c29c4d3df8f2a12a8"
        },
        "date": 1764044436564,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_builder_new",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "build_evm_to_evm_transaction",
            "value": 69,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "build_substrate_to_substrate_transaction",
            "value": 59,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "build_cross_chain_transaction",
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_with_data/32",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_with_data/256",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_with_data/1024",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_with_data/4096",
            "value": 1478,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_hash",
            "value": 1026,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "is_cross_chain_same",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "is_cross_chain_different",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_serialize",
            "value": 292,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_deserialize",
            "value": 439,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_amounts/1",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_amounts/1000",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_amounts/1000000",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_amounts/1000000000000",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "548e2134e8f72b8b11c184a2597d5cf37dea4271",
          "message": "ci(deps): bump checkout to 6 (#41)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 5 to 6.\n- [Release notes](https://github.com/actions/checkout/releases)\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\n- [Commits](https://github.com/actions/checkout/compare/v5...v6)\n\n---\nupdated-dependencies:\n- dependency-name: actions/checkout\n  dependency-version: '6'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-12-01T12:03:28-05:00",
          "tree_id": "8907ceabc84521b56c16b0e7d12832d43a1e65d4",
          "url": "https://github.com/kherldhussein/apex-sdk/commit/548e2134e8f72b8b11c184a2597d5cf37dea4271"
        },
        "date": 1764608971751,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_builder_new",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "build_evm_to_evm_transaction",
            "value": 69,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "build_substrate_to_substrate_transaction",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "build_cross_chain_transaction",
            "value": 69,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_with_data/32",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_with_data/256",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_with_data/1024",
            "value": 88,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_with_data/4096",
            "value": 1470,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_hash",
            "value": 1037,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "is_cross_chain_same",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "is_cross_chain_different",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_serialize",
            "value": 324,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_deserialize",
            "value": 432,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_amounts/1",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_amounts/1000",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_amounts/1000000",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_amounts/1000000000000",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7248cfdf52bc70bd03c01e5f34996bf65de7d55b",
          "message": "fix: ss58 (#44)\n\n* docs: update broken links\n\n* fix(benches): update file existence check before moving output.txt\n\n* docs: update README\n\n* fix(git): update .gitignore to include additional metadata files and profraw files\n\n* docs: update README and documentation links to reflect repository migration from kherldhussein to eurybits\n\n- Updated all instances of repository links in README files across apex-sdk-substrate, apex-sdk-types, cli, and examples to point to the new GitHub repository.\n- Added new badges for downloads, Substrate compatibility, and Rust version in README files.\n- Updated Discord links to the new community server.\n- Revised documentation files to ensure all links and references are consistent with the new repository.\n- Added a new example for Polkadot ecosystem showcasing multi-chain capabilities.\n\n* chore: update repository links and enhance community support in documentation\n\n* perf: optimize images for web deployment\n\n* perf: optimize images for web deployment\n\n* docs: update documentation and configuration for apexsdk.dev migration\n\n* feat: comprehensive test coverage improvements and repository migration\n\n- Add extensive unit tests for SDK core functionality\n- Implement comprehensive EVM adapter tests\n- Add Substrate adapter test coverage\n- Implement CLI command tests (balance, deploy, config)\n- Add connection pool and transaction processing tests\n- Update all repository references from kherldhussein to eurybits organization\n- Fix compilation errors and improve code quality\n- Target: Increase test coverage from 31.43% to 40%+\n\n* feat: add build script and package.json for documentation deployment\n\n* fix: ci\n\n* fix(cli): ignore network test\n\n* fix(test): commented out the module declarations for polkadot and kusama metadata until they are actually generated\n\n* chore(test): create integration test CI workflow\n\n* chore(test): create helper script\n\n* fix: resolve all CI failures and add integration test infrastructure\n\n  - Fix test failures (9 tests)\n  - Fix clippy warnings (5 warnings)\n  - Comment out ungenerated metadata modules\n  - Add gitignore for large generated files\n  - Add integration test CI workflow\n  - Add testing documentation\n  - Fix Cloudflare Pages build configuration\n\n* fix(cloudflare): add dist/ t0 .gitignore to keep generated files out of version control\n\n* doc: update readme",
          "timestamp": "2025-12-10T04:46:06-05:00",
          "tree_id": "a9bc4cdf0a5dafe2eff30c51f9df509c1931d830",
          "url": "https://github.com/eurybits/apex-sdk/commit/7248cfdf52bc70bd03c01e5f34996bf65de7d55b"
        },
        "date": 1765360487908,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160864,
            "range": "± 291",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140797,
            "range": "± 351",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "982c141d811b0edb631181b897dd3aaa8b5a4f9e",
          "message": "chore: refactor data (#45)",
          "timestamp": "2025-12-10T05:45:30-05:00",
          "tree_id": "ed9ea503cf3699de0b06d48a7e71ecf456bfe909",
          "url": "https://github.com/eurybits/apex-sdk/commit/982c141d811b0edb631181b897dd3aaa8b5a4f9e"
        },
        "date": 1765363882299,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161053,
            "range": "± 562",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140975,
            "range": "± 451",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0cf0d4c752d32b3a9758d229e638f5404ab76215",
          "message": "chore; add Rust CoC (#46)",
          "timestamp": "2025-12-10T06:51:38-05:00",
          "tree_id": "77ad8a99c8dc3c20ded935822f3acbee03d8bc22",
          "url": "https://github.com/eurybits/apex-sdk/commit/0cf0d4c752d32b3a9758d229e638f5404ab76215"
        },
        "date": 1765367830011,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160780,
            "range": "± 653",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140882,
            "range": "± 7240",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "44241fa8fa64969b9fc4211442f2e5be443fef00",
          "message": "docs: update (#47)\n\n* chore; add Rust Code of Conduct\n\n* docs: update documentation\n\n* Update README.md\n\n* refactor(docs): change support email \n\nUpdated contact email for domain and deployment issues.\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2025-12-10T13:22:38-05:00",
          "tree_id": "d042eb3975312ab32642c8aa63f988e9a5c3177e",
          "url": "https://github.com/eurybits/apex-sdk/commit/44241fa8fa64969b9fc4211442f2e5be443fef00"
        },
        "date": 1765391331031,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 26,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 157824,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 138136,
            "range": "± 5765",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b4791a926a26faa957a61e9a7a5c51218e79415d",
          "message": "chore(docs): subdomain redirect (#48)",
          "timestamp": "2025-12-11T12:44:40-05:00",
          "tree_id": "c764f596e785cbb8f9a18571c1e63abbc0dc20c2",
          "url": "https://github.com/eurybits/apex-sdk/commit/b4791a926a26faa957a61e9a7a5c51218e79415d"
        },
        "date": 1765475499045,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160840,
            "range": "± 432",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140848,
            "range": "± 367",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "32bc693e7953ed783c38b4b2b32dcd54312bc513",
          "message": "refactor docs (#49)\n\n* refactor: remove migration scripts\n\n* docs(sys_arch): refactor the system architecture",
          "timestamp": "2025-12-12T00:56:32-05:00",
          "tree_id": "873364604784a25c5c55f301e768f99a552bb393",
          "url": "https://github.com/eurybits/apex-sdk/commit/32bc693e7953ed783c38b4b2b32dcd54312bc513"
        },
        "date": 1765519374238,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160954,
            "range": "± 361",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141032,
            "range": "± 808",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dfc36d748f07fc61dc2c40fd0298458f7b8957b2",
          "message": "docs: overhaul and newsletter (#50)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic",
          "timestamp": "2025-12-15T02:22:41-05:00",
          "tree_id": "996d9d7120a08e3de5310e1a3a78b9a8937bb753",
          "url": "https://github.com/eurybits/apex-sdk/commit/dfc36d748f07fc61dc2c40fd0298458f7b8957b2"
        },
        "date": 1765783745913,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160975,
            "range": "± 464",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141000,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "273365e2ffff51aee7b567711672138b0a55a639",
          "message": "fix(docs): viewer (#52)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* docs: add Web3Forms API key injection step in deploy workflow\n\n* docs(viewer): add utility function to load external scripts dynamically\n\n* docs(subscribe): update subscription method to use Web3Forms API and remove Cloudflare function",
          "timestamp": "2025-12-15T22:55:28-05:00",
          "tree_id": "d08f905dada2e8cd69f145652a045914c2790d09",
          "url": "https://github.com/eurybits/apex-sdk/commit/273365e2ffff51aee7b567711672138b0a55a639"
        },
        "date": 1765857707397,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 59,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 59,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160968,
            "range": "± 371",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141006,
            "range": "± 384",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d6a457032bfd0705da432bd1d901c271099030f6",
          "message": "docs; fix subscription's errors  (#53)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* docs: add Web3Forms API key injection step in deploy workflow\n\n* docs(viewer): add utility function to load external scripts dynamically\n\n* docs(subscribe): update subscription method to use Web3Forms API and remove Cloudflare function\n\n* chore: update form key accessibility\n\n* doc; fix UI and subscriptions errors\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2025-12-18T00:58:22-05:00",
          "tree_id": "5c6c5a649fd380b46403ab07f80b2dbac94f3267",
          "url": "https://github.com/eurybits/apex-sdk/commit/d6a457032bfd0705da432bd1d901c271099030f6"
        },
        "date": 1766037869102,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160854,
            "range": "± 376",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141108,
            "range": "± 543",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dbe48a3493f533a1cc780c6cc8bec7a22752ed9f",
          "message": "docs: ui/ux overhaul (#54)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* docs: add Web3Forms API key injection step in deploy workflow\n\n* docs(viewer): add utility function to load external scripts dynamically\n\n* docs(subscribe): update subscription method to use Web3Forms API and remove Cloudflare function\n\n* chore: update form key accessibility\n\n* doc; fix UI and subscriptions errors\n\n* docs(research): add initial research papers directory and template files\n\n* docs: update SEO and sitemap for Apex SDK Protocol\n\n- Enhanced structured data in seo.js with additional schemas including Organization, FAQ, BreadcrumbList, HowTo, and Course.\n- Updated GitHub repository links from eurybits to carbobit.\n- Modified sitemap.xml to reflect new last modified dates and added new URLs for the CONTACT.md and research portal.\n- Improved viewer.html with updated meta tags for better SEO, including Open Graph and Twitter Card data.\n- Added contact information in the footer of viewer.html for better accessibility.\n\n* docs(examples): update Apex SDK documentation links to point to the correct repository\n\n* docs: update GitHub Issues link to point to the correct repository\n\n* chore(ci): add security-hardened build step to CI workflow and fix permissions in deploy docs\n\n* docs: update Apex SDK links to point to the correct repository\n\n* docs: update repository links and security contact information in documentation\n\n* chore(deps): update dependencies\n\n* docs: update security mail\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2025-12-21T07:26:02-05:00",
          "tree_id": "73fa98cfd1c321b8546a6e956ac7084c696a20ee",
          "url": "https://github.com/carbobit/apex-sdk/commit/dbe48a3493f533a1cc780c6cc8bec7a22752ed9f"
        },
        "date": 1766320388041,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160913,
            "range": "± 573",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140989,
            "range": "± 358",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "eec012aa00914924d3fbe549ca1593d1106cd918",
          "message": "fix: Jekyll build (#55)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(ci): remove unnecessary injection step after the build",
          "timestamp": "2025-12-21T07:54:16-05:00",
          "tree_id": "cd4d42c52d46ea6fbaeb8fc44f57432112bf964f",
          "url": "https://github.com/carbobit/apex-sdk/commit/eec012aa00914924d3fbe549ca1593d1106cd918"
        },
        "date": 1766322053171,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160933,
            "range": "± 510",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140970,
            "range": "± 588",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "abd3050ec2e8a98c0df32a9642b14317256e313c",
          "message": "fix(subscription): update configuration (#56)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(ci): remove unnecessary injection step after the build\n\n* fix: subscription failure\n\n* fix: subscription failure\n\n* refactor: docs\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2025-12-21T09:20:14-05:00",
          "tree_id": "cd3e8252cb3fc1ce4349ebd438e8c6784fbe98ea",
          "url": "https://github.com/carbobit/apex-sdk/commit/abd3050ec2e8a98c0df32a9642b14317256e313c"
        },
        "date": 1766327210135,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160961,
            "range": "± 472",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141012,
            "range": "± 512",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "85dad14a2e2f91b61d235fb50d58bea9ba6b8986",
          "message": "ci(deps): bump cache to v5 (#51)\n\nBumps [actions/cache](https://github.com/actions/cache) from 4 to 5.\n- [Release notes](https://github.com/actions/cache/releases)\n- [Changelog](https://github.com/actions/cache/blob/main/RELEASES.md)\n- [Commits](https://github.com/actions/cache/compare/v4...v5)\n\n---\nupdated-dependencies:\n- dependency-name: actions/cache\n  dependency-version: '5'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2025-12-24T02:57:23-05:00",
          "tree_id": "230a336242d9dc5f1080bca8cfb57bf04f691843",
          "url": "https://github.com/carbobit/apex-sdk/commit/85dad14a2e2f91b61d235fb50d58bea9ba6b8986"
        },
        "date": 1766563436849,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 56,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161000,
            "range": "± 401",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141024,
            "range": "± 418",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d4b6a6e2a9d29cc8085fc67ccb362629fc1f8926",
          "message": " sec: mitigate #59 and #61  (#60)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* security: mitigate RUSTSEC-2025-0137 ruint vulnerability\n\n- Update Alloy ecosystem dependencies to v1.2.1 (from v1.1.3)\n- Update alloy-primitives to v1.5.2 (from v1.5.1)\n- Add RUSTSEC-2025-0137 to deny.toml ignore list with documentation\n- Document risk assessment: LOW (function not used in app code)\n- All 232 tests pass, project builds successfully\n- Security rating improved from 7.5/10 to 8.2/10\n\nFixes: Unsoundness of safe reciprocal_mg10 function in ruint 1.17.0\nRisk: Memory corruption via out-of-bounds access (debug_assert optimized out)\nMitigation: Waiting for upstream fix, monitoring for updates\n\nRefs: https://rustsec.org/advisories/RUSTSEC-2025-0137\n\n* fix: add RUSTSEC-2025-0137 to audit ignore list",
          "timestamp": "2025-12-25T13:22:19-05:00",
          "tree_id": "c1f349dae48845ec5f51ac674481dcdea13b8184",
          "url": "https://github.com/carbobit/apex-sdk/commit/d4b6a6e2a9d29cc8085fc67ccb362629fc1f8926"
        },
        "date": 1766687347206,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 49,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 45,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 11,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 157852,
            "range": "± 728",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 137794,
            "range": "± 839",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0489a2c8be6eb452f5b46b51cb399cf900a24e89",
          "message": "refactor: impl testing infra  (#62)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* feat(docker): impl integration test infrastructure with EVM and Substrate nodes\n\n* feat(tests): update integration tests to run daily and add Docker-based tests\n\n* feat(tests): impl integration tests for EVM and Substrate nodes with Docker support\n\n* test(evm): impl comprehensive tests for transaction execution and wallet operations\n\n- Implemented transaction_executor_tests.rs to cover gas estimation, transaction building, signing, retry logic, and error handling.\n- Created wallet_operations_tests.rs to validate wallet creation methods, signing operations, wallet manager functionalities, and edge case handling.\n\n* test(substrate): impl comprehensive tests for transaction and XCM modules\n\n- Implement tests for transaction functionality including fee configuration, retry configuration, batch call building, extrinsic building, and transaction modes in `transaction_test.rs`.\n- Introduce tests for XCM functionality covering MultiLocation construction, asset representation, weight limits, junction types, and network IDs in `xcm_test.rs`.\n\n* test(types): impl comprehensive unit tests for Chain and Address functionality\n\n- Implement tests for various chain methods including name, chain type, smart contract support, and endpoints.\n- Add tests for chain ID validation for both EVM and Substrate chains.\n- Include tests for validation error handling and display.\n- Ensure coverage for address validation and edge cases.\n- Validate multiple RPC endpoints for different chains.\n\n* chore(docker): automate test builds with different env\n\n* fix(test): vec! with arrays\n\n* fix(test): vec! with arrays\n\n* fix(test): vec! with arrays\n\n* fix: clipy (#63)\n\n* Update root Cargo.toml\n\n* Update README.md\n\n* Update apex-sdk-evm Cargo.toml\n\n* Update apex-sdk-evm library code\n\n* Update apex-sdk-substrate Cargo.toml\n\n* Update apex-sdk-substrate library code\n\n* Update apex-sdk-substrate transaction module\n\n* Update apex-sdk-types Cargo.toml\n\n* Update apex-sdk-types library code\n\n* Update apex-sdk Cargo.toml\n\n* Fix clippy issues in transaction benchmarks\n\n* Fix clippy issues in advanced module\n\n* Update apex-sdk builder module\n\n* Add error recovery module\n\n* Add performance module\n\n* Update SDK core module\n\n* Update transaction module\n\n* Update CLI config module\n\n* Update CLI config command module\n\n* Update CLI deploy module\n\n* Update CLI documentation\n\n* Update docs config.js\n\n* Update account manager example\n\n* Update contract orchestration Cargo.toml\n\n* Update contract orchestration example\n\n* Update EVM integration tests\n\n* Fix substrate integration tests\n\n* Add EVM benchmarks\n\n* Add Substrate benchmarks\n\n* Add types benchmarks\n\n* Add Movement DeFi CLI template\n\n* Add docs deploy config\n\n* Add docs test config HTML\n\n* Add examples README\n\n* Add EVM contract call example\n\n* Add real transaction integration test\n\n* Add utility scripts\n\n* Add test documentation\n\n* Format transaction benchmarks after clippy fixes\n\n* chore: remove unsupported internal scripts\n\n* chore: update comment\n\n* chore(tests): remove redundant comments from test cases\n\n* chore(tests): remove redundant comments from test cases and improve code clarity\n\n* chore(tests): remove redundant comments from test cases for clarity\n\n* chore(docs): add examples for EVM transfers and contract interactions\n\n* chore(tests): impl EVM transfer example with detailed README and main.rs\n\n* chore(deps): update tracing and tracing-subscriber versions for consistency",
          "timestamp": "2025-12-29T02:38:40-05:00",
          "tree_id": "cb44071c795d0696181b76d06173e641760338c7",
          "url": "https://github.com/carbobit/apex-sdk/commit/0489a2c8be6eb452f5b46b51cb399cf900a24e89"
        },
        "date": 1766994497466,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160909,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140882,
            "range": "± 331",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 117,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 584,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5862,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 580,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c4fb2f8148b597b2dc157484868c7d1001472abd",
          "message": "chore: update core  (#66)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* feat(docker): impl integration test infrastructure with EVM and Substrate nodes\n\n* feat(tests): update integration tests to run daily and add Docker-based tests\n\n* feat(tests): impl integration tests for EVM and Substrate nodes with Docker support\n\n* test(evm): impl comprehensive tests for transaction execution and wallet operations\n\n- Implemented transaction_executor_tests.rs to cover gas estimation, transaction building, signing, retry logic, and error handling.\n- Created wallet_operations_tests.rs to validate wallet creation methods, signing operations, wallet manager functionalities, and edge case handling.\n\n* test(substrate): impl comprehensive tests for transaction and XCM modules\n\n- Implement tests for transaction functionality including fee configuration, retry configuration, batch call building, extrinsic building, and transaction modes in `transaction_test.rs`.\n- Introduce tests for XCM functionality covering MultiLocation construction, asset representation, weight limits, junction types, and network IDs in `xcm_test.rs`.\n\n* test(types): impl comprehensive unit tests for Chain and Address functionality\n\n- Implement tests for various chain methods including name, chain type, smart contract support, and endpoints.\n- Add tests for chain ID validation for both EVM and Substrate chains.\n- Include tests for validation error handling and display.\n- Ensure coverage for address validation and edge cases.\n- Validate multiple RPC endpoints for different chains.\n\n* chore(docker): automate test builds with different env\n\n* fix(test): vec! with arrays\n\n* fix(test): vec! with arrays\n\n* fix(test): vec! with arrays\n\n* fix: clipy (#63)\n\n* Update root Cargo.toml\n\n* Update README.md\n\n* Update apex-sdk-evm Cargo.toml\n\n* Update apex-sdk-evm library code\n\n* Update apex-sdk-substrate Cargo.toml\n\n* Update apex-sdk-substrate library code\n\n* Update apex-sdk-substrate transaction module\n\n* Update apex-sdk-types Cargo.toml\n\n* Update apex-sdk-types library code\n\n* Update apex-sdk Cargo.toml\n\n* Fix clippy issues in transaction benchmarks\n\n* Fix clippy issues in advanced module\n\n* Update apex-sdk builder module\n\n* Add error recovery module\n\n* Add performance module\n\n* Update SDK core module\n\n* Update transaction module\n\n* Update CLI config module\n\n* Update CLI config command module\n\n* Update CLI deploy module\n\n* Update CLI documentation\n\n* Update docs config.js\n\n* Update account manager example\n\n* Update contract orchestration Cargo.toml\n\n* Update contract orchestration example\n\n* Update EVM integration tests\n\n* Fix substrate integration tests\n\n* Add EVM benchmarks\n\n* Add Substrate benchmarks\n\n* Add types benchmarks\n\n* Add Movement DeFi CLI template\n\n* Add docs deploy config\n\n* Add docs test config HTML\n\n* Add examples README\n\n* Add EVM contract call example\n\n* Add real transaction integration test\n\n* Add utility scripts\n\n* Add test documentation\n\n* Format transaction benchmarks after clippy fixes\n\n* chore: remove unsupported internal scripts\n\n* chore: update comment\n\n* chore(tests): remove redundant comments from test cases\n\n* chore(tests): remove redundant comments from test cases and improve code clarity\n\n* chore(tests): remove redundant comments from test cases for clarity\n\n* chore(docs): add examples for EVM transfers and contract interactions\n\n* chore(tests): impl EVM transfer example with detailed README and main.rs\n\n* chore(deps): update tracing and tracing-subscriber versions for consistency\n\n* chore(docs): remove outdated test infrastructure details from README\n\n* chore(docs): update README examples for clarity and consistency\nfeat: expose get_transaction_receipt method in ProviderType\n\n* chore(tests): add basic implementations for liquidity and order handling\n\n* chore(docs): remove build.sh from .gitignore and add build script for documentation",
          "timestamp": "2025-12-29T04:05:20-05:00",
          "tree_id": "d03e4ceda971c9e20ba42517761b4e53f86da8c4",
          "url": "https://github.com/carbobit/apex-sdk/commit/c4fb2f8148b597b2dc157484868c7d1001472abd"
        },
        "date": 1766999717422,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 37,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 64,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161035,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140986,
            "range": "± 835",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 108,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 552,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5482,
            "range": "± 272",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 548,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "72e541d4184d077027222c601ec7990a7eb1a803",
          "message": "apex-sdk===v0.1.5 (#69)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(docs): resolve networks display and update footer copyright\n\n- Fix networks not showing on page load by reordering renderChains() before initChainsFilter()\n- Update footer copyright to '© 2026 Apex SDK Protocol by Carbobit'\n- Ensures all network cards display immediately without user interaction\n\nFixes display issues on landing page for v0.1.5 release\n\n* feat(evm): implement RPC URL tracking to EvmProvider\n\n- Add rpc_url field to EvmProvider struct\n- Implement rpc_url() getter method\n- Store endpoint URL for later retrieval by adapter\n\nRequired for proper endpoint() method implementation\n\n* fix(tests): update golden vectors and metrics tests for v0.1.5\n\n- Replace ChainType::Evm with ChainType::Ethereum across all tests\n- Update SubstrateEra from enum variant to struct initialization\n- Fix metrics API calls: record_operation_time() → record_duration()\n- Fix metrics API calls: get_performance_metrics() → get_metrics()\n\nAll 119 tests now passing\n\n* refactor(substrate): update signer implementations and enhance transaction fee estimation\n\n- Updated the SR25519 and ED25519 signer implementations to simplify the code and remove unnecessary account ID storage.\n- Introduced a new method in the TransactionExecutor to estimate fees from raw transaction bytes, using a dummy transfer for estimation.\n- Implemented the FeeEstimator trait for TransactionExecutor with a placeholder for fee estimation from raw bytes.\n- Added async signing capabilities to the Wallet struct by implementing the CoreSigner trait.\n- Created comprehensive tests for the connection pool module, covering configuration, connection management, health checking, and load balancing.\n\n* feat: improve transaction status tracking\n\n* refactor(core): removed unused methods\n\n* feat(substrate): enhance various nonce management\n\n* feat(cli): implement and fix EIP-1559 deployment flow\n\n* test(types): enhance testing and types implementation\n\n* refactor(sdk): enhance transaction and performance optimization\n\n* text(integration): update tests\n\n* docs: refactor and update the UI and documentation for v0.1.5\n\n* docs: update README\n\n* chore(CHANGELOG): restore CHANGELOG\n\n* release: v0.1.5\n\n* fix(ci): update workflows referenced apex-sdk-cli package and apex binary\n\n* fix(cli): remove unused dependencies\n\n* fix(evm): removed redundant (gas_price as u128) cast\n\n* fix(deps): remove unused dependencies\n\n* fix(substrate): removed redundant cast\n\n* fix(cli): removed redundant cast for gas price in estimated cost calculation\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-03T02:49:58-05:00",
          "tree_id": "a71d81eab365ee33ebfe0c7ae009eaefb1434b2d",
          "url": "https://github.com/carbobit/apex-sdk/commit/72e541d4184d077027222c601ec7990a7eb1a803"
        },
        "date": 1767427190358,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 56,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160847,
            "range": "± 626",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140915,
            "range": "± 366",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 110,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 546,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5509,
            "range": "± 243",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 543,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "58ada6e8ec35ee98a746fc4b7ac09355b5ad093a",
          "message": "fix(test): integration (#70)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* fix(test): update private key configuration",
          "timestamp": "2026-01-03T08:11:39-05:00",
          "tree_id": "16431aac0b2d44a0dac4a578d2914ecb20da482f",
          "url": "https://github.com/carbobit/apex-sdk/commit/58ada6e8ec35ee98a746fc4b7ac09355b5ad093a"
        },
        "date": 1767446515412,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 49,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 49,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 23,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 23,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 42,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 157825,
            "range": "± 762",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 137786,
            "range": "± 319",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 99,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 48,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 479,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 4822,
            "range": "± 274",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 478,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ba62f7870fb504a5d3ed24c31ffbd7d5e3097827",
          "message": "fix: compilation errors and docs (#71)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* fix: resolve compilation errors and improve docs UX\n\n* fix: resolve clippy fmt errors\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-03T09:35:15-05:00",
          "tree_id": "fa94d346d11b1b2a2f791536d19c7804b250b108",
          "url": "https://github.com/carbobit/apex-sdk/commit/ba62f7870fb504a5d3ed24c31ffbd7d5e3097827"
        },
        "date": 1767451492401,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 56,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160707,
            "range": "± 834",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140609,
            "range": "± 627",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 109,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 545,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5458,
            "range": "± 247",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 548,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "037e4bebae7b856c41bf9ea430f52851148666fc",
          "message": "docs(ui): fix (#72)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* fix: resolve compilation errors and improve docs UX\n\n* fix: resolve clippy fmt errors\n\n* docs(ui): fix\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-03T09:56:19-05:00",
          "tree_id": "a12b85567b9de58d6d113e1457f80438f962b230",
          "url": "https://github.com/carbobit/apex-sdk/commit/037e4bebae7b856c41bf9ea430f52851148666fc"
        },
        "date": 1767452777100,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160945,
            "range": "± 560",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140870,
            "range": "± 503",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 116,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 584,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 6096,
            "range": "± 249",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 586,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d0002093a6d9eefcac30596ab184f46d334dea97",
          "message": "fix: docs (#73)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* docs: fix\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-05T09:09:47-05:00",
          "tree_id": "1f30ea2a05636ac1582afc047d0a6011a4d06363",
          "url": "https://github.com/carbobit/apex-sdk/commit/d0002093a6d9eefcac30596ab184f46d334dea97"
        },
        "date": 1767622897962,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 66,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 65,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161035,
            "range": "± 343",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141027,
            "range": "± 398",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 127,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 65,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 672,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 6727,
            "range": "± 615",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 774,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7677e38a04c3fca4be9235ae71cd4fb5425d19be",
          "message": "docs: website modularization  (#78)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* Add CSS variables for consistent design system\n\n- Define color palette with primary, secondary, and neutral colors\n- Set typography scale and spacing units\n- Add responsive breakpoints and z-index layers\n- Establish foundation for modular CSS architecture\n\n* Add base CSS styles with modern reset and typography\n\n- Implement modern CSS reset for consistent cross-browser rendering\n- Set base typography with Inter font family and responsive scaling\n- Add smooth scroll behavior and focus management\n- Establish foundational styling for semantic elements\n\n* Add responsive layout system with CSS Grid and Flexbox\n\n- Implement flexible grid system for consistent page layouts\n- Add responsive container and spacing utilities\n- Create mobile-first responsive design patterns\n- Ensure accessibility with proper heading hierarchy\n\n* Add modular component styles for UI consistency\n\n- Navigation component with responsive mobile menu\n- Button system with hover states and accessibility\n- Card layouts for feature and network displays\n- Footer with grid layout and social link animations\n- Network explorer with proper brand styling\n- Form elements with modern interaction states\n\n* Add utility classes for rapid development\n\n- Spacing utilities with consistent margin/padding scale\n- Typography helpers for text alignment and sizing\n- Display utilities for responsive visibility control\n- Accessibility utilities for screen readers and focus management\n\n* Add modular CSS entry point for organized architecture\n\n- Single import file that loads all CSS modules in correct order\n- Eliminates need for multiple link tags in HTML\n- Provides clear dependency management for styles\n- Enables easier maintenance and updates\n\n* Add page-specific CSS overrides\n\n- Documentation viewer page enhancements\n- Special layout adjustments for content pages\n- Maintains compatibility with new modular system\n- Handles edge cases for complex page layouts\n\n* Add unified JavaScript application with modular architecture\n\n- Browser-compatible ApexSDK namespace for all components\n- Theme toggle with localStorage persistence\n- Mobile menu with smooth animations\n- Chain renderer for network status display\n- Error handling and performance monitoring\n- Documentation viewer integration\n\n* Add JavaScript utility modules for error handling and performance\n\n- Centralized error handling with user-friendly messages\n- Performance monitoring with Core Web Vitals tracking\n- Loading state management for better UX\n- Consistent error reporting across all components\n\n* Add Service Worker for enhanced performance and offline support\n\n- Cache static assets for faster loading\n- Network-first strategy for dynamic content\n- Offline fallback page for better UX\n- Background sync for improved reliability\n\n* Add offline fallback page for Service Worker\n\n- User-friendly offline experience\n- Consistent branding when network unavailable\n- Helpful messaging with retry functionality\n- Maintains visual consistency with main site\n\n* Update homepage to use modular architecture and fix messaging\n\n- Switch to modular CSS system for better maintainability\n- Remove 'production ready' claims for alpha release\n- Update CTA section with realistic development status messaging\n- Improve semantic HTML structure and accessibility\n- Add Service Worker integration for performance\n\n* Update documentation viewer with modular architecture\n\n- Migrate to modular CSS and JavaScript system\n- Enhance accessibility with proper ARIA attributes\n- Improve responsive design for mobile devices\n- Add Service Worker integration for offline docs viewing\n\n* Add reusable HTML includes for component management\n\n- Modular HTML components for navigation and footer\n- Consistent UI elements across all pages\n- Easier maintenance with centralized component files\n- Template system for dynamic content injection\n\n* Add JavaScript component modules for specific functionality\n\n- Individual modules for theme, menu, and documentation features\n- Clean separation of concerns for easier testing\n- Modular architecture supporting code reuse\n- Enhanced maintainability with focused components\n\n* Add feature-specific JavaScript modules\n\n- Chain renderer for blockchain network display\n- Documentation viewer for interactive content\n- Modular structure supporting feature isolation\n- Enhanced code organization and testability\n\n* Add ES6 modular JavaScript entry point\n\n- Modern import/export syntax for module system\n- Alternative architecture supporting Node.js environments\n- Clean dependency management with explicit imports\n- Future-ready codebase for advanced tooling integration",
          "timestamp": "2026-01-05T15:00:50-05:00",
          "tree_id": "2f50658250fc54a5133f4c109f926e7a24b0c4d5",
          "url": "https://github.com/carbobit/apex-sdk/commit/7677e38a04c3fca4be9235ae71cd4fb5425d19be"
        },
        "date": 1767643837146,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 56,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160818,
            "range": "± 373",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140910,
            "range": "± 533",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 109,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 545,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5439,
            "range": "± 279",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 541,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "01f0e78830afcce7d1f0a25ff79fd4e1610182d0",
          "message": "docs: fix logo (#79)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* Add CSS variables for consistent design system\n\n- Define color palette with primary, secondary, and neutral colors\n- Set typography scale and spacing units\n- Add responsive breakpoints and z-index layers\n- Establish foundation for modular CSS architecture\n\n* Add base CSS styles with modern reset and typography\n\n- Implement modern CSS reset for consistent cross-browser rendering\n- Set base typography with Inter font family and responsive scaling\n- Add smooth scroll behavior and focus management\n- Establish foundational styling for semantic elements\n\n* Add responsive layout system with CSS Grid and Flexbox\n\n- Implement flexible grid system for consistent page layouts\n- Add responsive container and spacing utilities\n- Create mobile-first responsive design patterns\n- Ensure accessibility with proper heading hierarchy\n\n* Add modular component styles for UI consistency\n\n- Navigation component with responsive mobile menu\n- Button system with hover states and accessibility\n- Card layouts for feature and network displays\n- Footer with grid layout and social link animations\n- Network explorer with proper brand styling\n- Form elements with modern interaction states\n\n* Add utility classes for rapid development\n\n- Spacing utilities with consistent margin/padding scale\n- Typography helpers for text alignment and sizing\n- Display utilities for responsive visibility control\n- Accessibility utilities for screen readers and focus management\n\n* Add modular CSS entry point for organized architecture\n\n- Single import file that loads all CSS modules in correct order\n- Eliminates need for multiple link tags in HTML\n- Provides clear dependency management for styles\n- Enables easier maintenance and updates\n\n* Add page-specific CSS overrides\n\n- Documentation viewer page enhancements\n- Special layout adjustments for content pages\n- Maintains compatibility with new modular system\n- Handles edge cases for complex page layouts\n\n* Add unified JavaScript application with modular architecture\n\n- Browser-compatible ApexSDK namespace for all components\n- Theme toggle with localStorage persistence\n- Mobile menu with smooth animations\n- Chain renderer for network status display\n- Error handling and performance monitoring\n- Documentation viewer integration\n\n* Add JavaScript utility modules for error handling and performance\n\n- Centralized error handling with user-friendly messages\n- Performance monitoring with Core Web Vitals tracking\n- Loading state management for better UX\n- Consistent error reporting across all components\n\n* Add Service Worker for enhanced performance and offline support\n\n- Cache static assets for faster loading\n- Network-first strategy for dynamic content\n- Offline fallback page for better UX\n- Background sync for improved reliability\n\n* Add offline fallback page for Service Worker\n\n- User-friendly offline experience\n- Consistent branding when network unavailable\n- Helpful messaging with retry functionality\n- Maintains visual consistency with main site\n\n* Update homepage to use modular architecture and fix messaging\n\n- Switch to modular CSS system for better maintainability\n- Remove 'production ready' claims for alpha release\n- Update CTA section with realistic development status messaging\n- Improve semantic HTML structure and accessibility\n- Add Service Worker integration for performance\n\n* Update documentation viewer with modular architecture\n\n- Migrate to modular CSS and JavaScript system\n- Enhance accessibility with proper ARIA attributes\n- Improve responsive design for mobile devices\n- Add Service Worker integration for offline docs viewing\n\n* Add reusable HTML includes for component management\n\n- Modular HTML components for navigation and footer\n- Consistent UI elements across all pages\n- Easier maintenance with centralized component files\n- Template system for dynamic content injection\n\n* Add JavaScript component modules for specific functionality\n\n- Individual modules for theme, menu, and documentation features\n- Clean separation of concerns for easier testing\n- Modular architecture supporting code reuse\n- Enhanced maintainability with focused components\n\n* Add feature-specific JavaScript modules\n\n- Chain renderer for blockchain network display\n- Documentation viewer for interactive content\n- Modular structure supporting feature isolation\n- Enhanced code organization and testability\n\n* Add ES6 modular JavaScript entry point\n\n- Modern import/export syntax for module system\n- Alternative architecture supporting Node.js environments\n- Clean dependency management with explicit imports\n- Future-ready codebase for advanced tooling integration\n\n* fix(docs): ui\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-05T17:34:02-05:00",
          "tree_id": "9acdbdad9d266310ac1609f21e2a33f5f8089f8e",
          "url": "https://github.com/carbobit/apex-sdk/commit/01f0e78830afcce7d1f0a25ff79fd4e1610182d0"
        },
        "date": 1767653043003,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 67,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 67,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160772,
            "range": "± 470",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140591,
            "range": "± 385",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 134,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 66,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 740,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 7054,
            "range": "± 1615",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 706,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2543f529874081587545f24a297fe04ff733d888",
          "message": "ci(deps): bump upload-artifact v6 (#67)\n\nBumps [actions/upload-artifact](https://github.com/actions/upload-artifact) from 4 to 6.\n- [Release notes](https://github.com/actions/upload-artifact/releases)\n- [Commits](https://github.com/actions/upload-artifact/compare/v4...v6)\n\n---\nupdated-dependencies:\n- dependency-name: actions/upload-artifact\n  dependency-version: '6'\n  dependency-type: direct:production\n  update-type: version-update:semver-major\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\nCo-authored-by: carbobits <carbobit@gmail.com>\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-07T01:09:09-05:00",
          "tree_id": "6bd306a7e6e4c96bfdd0fcbc59c93d0c773660e4",
          "url": "https://github.com/carbobit/apex-sdk/commit/2543f529874081587545f24a297fe04ff733d888"
        },
        "date": 1767767358113,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 53,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161106,
            "range": "± 698",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141165,
            "range": "± 539",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 118,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 582,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5874,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 580,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "44955985415b20b37db125503d8ec88987f99dd5",
          "message": "deps(deps): update dirs requirement v6.0 (#74)\n\nUpdates the requirements on [dirs](https://github.com/soc/dirs-rs) to permit the latest version.\n- [Commits](https://github.com/soc/dirs-rs/commits)\n\n---\nupdated-dependencies:\n- dependency-name: dirs\n  dependency-version: 6.0.0\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-07T01:11:05-05:00",
          "tree_id": "7b7d1eba9851053aabd97edfdd05804d9f63435e",
          "url": "https://github.com/carbobit/apex-sdk/commit/44955985415b20b37db125503d8ec88987f99dd5"
        },
        "date": 1767767405660,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160894,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140887,
            "range": "± 365",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 110,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 557,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5479,
            "range": "± 258",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 545,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3f754f5338e262e26c6a5cee52e73fa04d4317bb",
          "message": "deps(deps): update dialoguer requirement v0.12 (#75)\n\nUpdates the requirements on [dialoguer](https://github.com/console-rs/dialoguer) to permit the latest version.\n- [Release notes](https://github.com/console-rs/dialoguer/releases)\n- [Changelog](https://github.com/console-rs/dialoguer/blob/main/CHANGELOG-OLD.md)\n- [Commits](https://github.com/console-rs/dialoguer/compare/v0.11.0...v0.12.0)\n\n---\nupdated-dependencies:\n- dependency-name: dialoguer\n  dependency-version: 0.12.0\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-07T01:11:55-05:00",
          "tree_id": "eaad8b0428f14eac2db74f8972a37eb56f900d56",
          "url": "https://github.com/carbobit/apex-sdk/commit/3f754f5338e262e26c6a5cee52e73fa04d4317bb"
        },
        "date": 1767767721040,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160790,
            "range": "± 387",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140790,
            "range": "± 407",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 116,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 24,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 586,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5887,
            "range": "± 251",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 580,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b3d646105923863a95004bc27aff39eed3fcce0e",
          "message": "deps(deps): update colored requirement v3.0 (#76)\n\nUpdates the requirements on [colored](https://github.com/mackwic/colored) to permit the latest version.\n- [Release notes](https://github.com/mackwic/colored/releases)\n- [Changelog](https://github.com/colored-rs/colored/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/mackwic/colored/compare/v2.1.0...v3.0.0)\n\n---\nupdated-dependencies:\n- dependency-name: colored\n  dependency-version: 3.0.0\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-07T01:12:37-05:00",
          "tree_id": "09c351c861f9f6d05b0dd3bc871859fc0c488b51",
          "url": "https://github.com/carbobit/apex-sdk/commit/b3d646105923863a95004bc27aff39eed3fcce0e"
        },
        "date": 1767767780662,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160991,
            "range": "± 434",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141057,
            "range": "± 879",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 107,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 544,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5791,
            "range": "± 242",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 569,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7942204e1c41e6b5d0baba66824bcafe0503966a",
          "message": "deps(deps): update reqwest requirement v0.13 (#77)\n\nUpdates the requirements on [reqwest](https://github.com/seanmonstar/reqwest) to permit the latest version.\n- [Release notes](https://github.com/seanmonstar/reqwest/releases)\n- [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)\n- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.12.0...v0.13.1)\n\n---\nupdated-dependencies:\n- dependency-name: reqwest\n  dependency-version: 0.13.1\n  dependency-type: direct:production\n...\n\nSigned-off-by: dependabot[bot] <support@github.com>\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-07T01:13:25-05:00",
          "tree_id": "735dcb61b5c60a93ef7fabf11481504994059abe",
          "url": "https://github.com/carbobit/apex-sdk/commit/7942204e1c41e6b5d0baba66824bcafe0503966a"
        },
        "date": 1767768964013,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160926,
            "range": "± 267",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140814,
            "range": "± 504",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 115,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 579,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5828,
            "range": "± 280",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 578,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4fedee2cc45e8214e07dbb2d7ffbddd351ad7a7f",
          "message": "fix: enable reqwest ntegration (#90)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* fix: enable json feature for reqwest in integration tests\n\n* feat: add reqwest\n\n* fix: update dependencies for colored, dialoguer, dirs, and reqwest\n\n* fix: add RUSTSEC-2026-0002 (lru unsound IterMut) to audit ignore list\n\n- Addresses CI security audit failure for lru 0.12.5 and 0.13.0\n- These are transitive dependencies from subxt and alloy\n- Low risk: unsound behavior requires specific usage patterns to trigger\n- Will be resolved when upstream dependencies update to fixed lru versions\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-08T01:46:39-05:00",
          "tree_id": "6195d50c401f63af74570eed75fc34f250c8298f",
          "url": "https://github.com/carbobit/apex-sdk/commit/4fedee2cc45e8214e07dbb2d7ffbddd351ad7a7f"
        },
        "date": 1767855511044,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160954,
            "range": "± 534",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141141,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 111,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 545,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5406,
            "range": "± 286",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 545,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e54b1e64e7a0f0c079caaba07dc8a8b7973ca14a",
          "message": "fix: advisories and integration tests (#93)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* fix: enable json feature for reqwest in integration tests\n\n* feat: add reqwest\n\n* fix: update dependencies for colored, dialoguer, dirs, and reqwest\n\n* fix: add RUSTSEC-2026-0002 (lru unsound IterMut) to audit ignore list\n\n- Addresses CI security audit failure for lru 0.12.5 and 0.13.0\n- These are transitive dependencies from subxt and alloy\n- Low risk: unsound behavior requires specific usage patterns to trigger\n- Will be resolved when upstream dependencies update to fixed lru versions\n\n* fix: resolve remaining CI issues\n\n- Add RUSTSEC-2026-0002 (lru unsound IterMut) to deny.toml ignore list\n- Add explicit integration test skip when INTEGRATION_TESTS env var not set\n- Prevents integration tests from running in main CI pipeline\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-08T09:11:27-05:00",
          "tree_id": "bf716d917532658eb61db36dc439e255ee881d55",
          "url": "https://github.com/carbobit/apex-sdk/commit/e54b1e64e7a0f0c079caaba07dc8a8b7973ca14a"
        },
        "date": 1767882159726,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 53,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161065,
            "range": "± 649",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141051,
            "range": "± 323",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 117,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 585,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5841,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 590,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "51efda3e99037c9785e6952ea5091c1072283cbf",
          "message": "fix: resolve signer type complexity (#94)\n\n* chore(docs): update logos and add new ones\n\n* docs(README): update links\n\n* docs(viewer): improve UI/UX\n\n* docs(viewer): improve UI/UX\n\n* docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\n\n* docs(css): update main stylesheet with new design elements and animations\n\n* docs(search): update search manifest with new categories and paths\n\n* docs(subscribe): update API endpoint and add new subscription handling logic\n\n* Revert \"docs(subscribe): update API endpoint and add new subscription handling logic\"\n\nThis reverts commit 8c1b76092fd71d852af33e6e530af594f7d5cb8c.\n\n* Revert \"docs(search): update search manifest with new categories and paths\"\n\nThis reverts commit 581eaa118c7c0a5c8524b3c9cbb7db47a42b5e0b.\n\n* Revert \"docs(css): update main stylesheet with new design elements and animations\"\n\nThis reverts commit fbe3410ef26484073511a56fa3c2ded72b1dc4c6.\n\n* Revert \"docs(js): update module initialization and add new modules for advanced visualization, metrics, workflow simulator, and personalization\"\n\nThis reverts commit 235e7bbe0f4c95db1d13c32d38856bab005d7311.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit 070e644d74c5c28abee8f49147afd051f87caa7a.\n\n* Revert \"docs(viewer): improve UI/UX\"\n\nThis reverts commit f12ba946d40df93c8f846529b867b243405d6a57.\n\n* Revert \"docs(README): update links\"\n\nThis reverts commit b7dcfc536af992ae90dde40f946664ac30ae951d.\n\n* fix(test): update private key configuration\n\n* fix: resolve type complexity in EVM signer",
          "timestamp": "2026-01-08T13:42:46-05:00",
          "tree_id": "03fd4cd8e2fd0b27289e189cd6ff58cf0f85554a",
          "url": "https://github.com/carbobit/apex-sdk/commit/51efda3e99037c9785e6952ea5091c1072283cbf"
        },
        "date": 1767898370898,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 49,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 49,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 157573,
            "range": "± 306",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 137509,
            "range": "± 316",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 97,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 48,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 477,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 4743,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 476,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "distinct": true,
          "id": "0f28a787c4e5e674ee66e4cc21b192cc81ed5095",
          "message": "Merge branch 'main' of github.com:carbobit/apex-sdk",
          "timestamp": "2026-01-08T14:04:08-05:00",
          "tree_id": "03fd4cd8e2fd0b27289e189cd6ff58cf0f85554a",
          "url": "https://github.com/carbobit/apex-sdk/commit/0f28a787c4e5e674ee66e4cc21b192cc81ed5095"
        },
        "date": 1767899665295,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 73,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 77,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161028,
            "range": "± 403",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141081,
            "range": "± 336",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 143,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 76,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 763,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 7899,
            "range": "± 832",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 725,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fd1509fe361ded13d467c19be43bb62ebdabcc38",
          "message": "fix: EIP-2718 transaction encoding (#95)\n\n* feat(deps): add alloy-eips dependency for EIP-2718 encoding\n\n* feat: add alloy-eips dependency for EIP-2718 encoding\n\n* fix(signer): use encoded_2718() for proper EIP-2718 transaction encoding in signer\n\n* fix(transaction): use encoded_2718() for proper EIP-2718 encoding in transaction executor",
          "timestamp": "2026-01-08T23:01:46-05:00",
          "tree_id": "09785831f2ee97fb20e73cdfc756d3b66beeea6e",
          "url": "https://github.com/carbobit/apex-sdk/commit/fd1509fe361ded13d467c19be43bb62ebdabcc38"
        },
        "date": 1767931906046,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160581,
            "range": "± 469",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140820,
            "range": "± 2217",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 114,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 586,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5888,
            "range": "± 277",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 591,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5bc44e085ab419c938d597c1725fb8502edc0a4f",
          "message": "feat: implement Substrate nonce (#96)\n\n* test(integration): optimize EVM test performance\n\n* feat(nonce): implement Substrate Nonce Manager with local caching and network reconciliation\n\n* feat(storage): create Debug and Clone traits to StorageClient struct\n\n* feat(nonce): refactor nonce retrieval to use StorageClient for improved querying",
          "timestamp": "2026-01-09T15:38:58-05:00",
          "tree_id": "4abb5605c38ac394dc46c3d326ed5e097ae2452d",
          "url": "https://github.com/carbobit/apex-sdk/commit/5bc44e085ab419c938d597c1725fb8502edc0a4f"
        },
        "date": 1767991733442,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 55,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160837,
            "range": "± 294",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140833,
            "range": "± 442",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 116,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 584,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5846,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 582,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "20f615754ac4712cb9a987f735a8b07691216f96",
          "message": "feat(v0.1.5): block querying & transaction pipeline (#97)\n\n* fix(docs): resolve networks display and update footer copyright\n\n- Fix networks not showing on page load by reordering renderChains() before initChainsFilter()\n- Update footer copyright to '© 2026 Apex SDK Protocol by Carbobit'\n- Ensures all network cards display immediately without user interaction\n\nFixes display issues on landing page for v0.1.5 release\n\n* feat(evm): implement RPC URL tracking to EvmProvider\n\n- Add rpc_url field to EvmProvider struct\n- Implement rpc_url() getter method\n- Store endpoint URL for later retrieval by adapter\n\nRequired for proper endpoint() method implementation\n\n* fix(tests): update golden vectors and metrics tests for v0.1.5\n\n- Replace ChainType::Evm with ChainType::Ethereum across all tests\n- Update SubstrateEra from enum variant to struct initialization\n- Fix metrics API calls: record_operation_time() → record_duration()\n- Fix metrics API calls: get_performance_metrics() → get_metrics()\n\nAll 119 tests now passing\n\n* refactor(substrate): update signer implementations and enhance transaction fee estimation\n\n- Updated the SR25519 and ED25519 signer implementations to simplify the code and remove unnecessary account ID storage.\n- Introduced a new method in the TransactionExecutor to estimate fees from raw transaction bytes, using a dummy transfer for estimation.\n- Implemented the FeeEstimator trait for TransactionExecutor with a placeholder for fee estimation from raw bytes.\n- Added async signing capabilities to the Wallet struct by implementing the CoreSigner trait.\n- Created comprehensive tests for the connection pool module, covering configuration, connection management, health checking, and load balancing.\n\n* feat: improve transaction status tracking\n\n* refactor(core): removed unused methods\n\n* feat(substrate): enhance various nonce management\n\n* feat(cli): implement and fix EIP-1559 deployment flow\n\n* test(types): enhance testing and types implementation\n\n* refactor(sdk): enhance transaction and performance optimization\n\n* text(integration): update tests\n\n* docs: refactor and update the UI and documentation for v0.1.5\n\n* docs: update README\n\n* chore(CHANGELOG): restore CHANGELOG\n\n* release: v0.1.5\n\n* fix(ci): update workflows referenced apex-sdk-cli package and apex binary\n\n* fix(cli): remove unused dependencies\n\n* fix(evm): removed redundant (gas_price as u128) cast\n\n* fix(deps): remove unused dependencies\n\n* fix(substrate): removed redundant cast\n\n* fix(cli): removed redundant cast for gas price in estimated cost calculation\n\n* chore: update CHANGELOG for v0.1.5 release\n\n* feat: enhance BlockInfo with comprehensive metadata\n\n* fix: update mock provider with enhanced BlockInfo fields\n\n* feat: add BlockInfo metadata to EVM provider\n\n* feat: implement BlockQuery module for comprehensive block retrieval\n\n* feat: add finality-aware block caching with dual-key storage\n\n* feat: replace mock block data with real subxt v0.44 implementation\n\n* test: add comprehensive block performance benchmarks\n\n* test: add 13 unit tests for block caching and serialization\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-12T23:29:30-05:00",
          "tree_id": "dd83a0a7f54e0a9c8afd918e3602d5f041349a1e",
          "url": "https://github.com/carbobit/apex-sdk/commit/20f615754ac4712cb9a987f735a8b07691216f96"
        },
        "date": 1768279168402,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160919,
            "range": "± 794",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140636,
            "range": "± 498",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 107,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 587,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5464,
            "range": "± 262",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 546,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5af6e4f0a6fe41aca39d8cd545276108df33a394",
          "message": "fix: strip ARM64 binary (#98)\n\n* fix(ci): remove unnecessary injection step after the build\n\n* fix: subscription failure\n\n* fix: subscription failure\n\n* refactor: docs\n\n* fix(aarch64): update the strip binary command\n\n---------\n\nCo-authored-by: Kherld <50875687+kherldhussein@users.noreply.github.com>",
          "timestamp": "2026-01-13T00:49:17-05:00",
          "tree_id": "909081ad88ce38e344ce2d3f08046d64562eab61",
          "url": "https://github.com/carbobit/apex-sdk/commit/5af6e4f0a6fe41aca39d8cd545276108df33a394"
        },
        "date": 1768283953633,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 55,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160893,
            "range": "± 524",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140864,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 109,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 554,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5584,
            "range": "± 729",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 546,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ceba4172227731bbdd19347c515b4a30d6665051",
          "message": "chore: fix CI errors (#99)\n\n* chore: prepare CLI for crates.io publication\n\n* feat(cli): create a [[bin]] section specifying apex-cli as the binary name\n\n* refactor(ci): update workflow files",
          "timestamp": "2026-01-15T10:37:44-05:00",
          "tree_id": "ee89be66428ac2a934e3e4c97570cfafe004166b",
          "url": "https://github.com/carbobit/apex-sdk/commit/ceba4172227731bbdd19347c515b4a30d6665051"
        },
        "date": 1768492071737,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160854,
            "range": "± 4678",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140953,
            "range": "± 744",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 110,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 584,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5481,
            "range": "± 284",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 541,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "320f4687baee4aeba2b0fb72a7145f6e3dd5aad9",
          "message": "chore: migrate org (#100)",
          "timestamp": "2026-01-16T16:11:22-05:00",
          "tree_id": "74b9e535e04959d693fbab240ed9237f98beebc6",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/320f4687baee4aeba2b0fb72a7145f6e3dd5aad9"
        },
        "date": 1768598483231,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160810,
            "range": "± 376",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140828,
            "range": "± 381",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 115,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 581,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5837,
            "range": "± 247",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 581,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f8b702587644044761e609832e816c4b33080916",
          "message": "chore: workflow & docs (#101)\n\n* docs(README): update README for clarity and conciseness\n\n* chore: remove outdated documentation files\n\n* docs(README): streamline quick navigation section and remove outdated links\n\n* chore(dependabot): add automated label and reviewers for GitHub Actions and Cargo updates\n\n* feat(labeler): create PR labeler\n\n* feat(release): create release notes config\n\n* feat(release): enhance release notes generation with installation instructions and links\n\n* feat(workflows): create GitHub Actions for benchmark testing, auto-merge, PR labeling, release notifications, and stale issue management",
          "timestamp": "2026-01-18T03:08:57-05:00",
          "tree_id": "b45d709f5fa3a4f8406b2d9f39bf64711d5105e8",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/f8b702587644044761e609832e816c4b33080916"
        },
        "date": 1768724369101,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 51,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 80,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160978,
            "range": "± 523",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140879,
            "range": "± 485",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 116,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 82,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 586,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5829,
            "range": "± 264",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 581,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4367757b5e7b70aba2be758011d209029076e70a",
          "message": "impl tx broadcasting (#107)\n\n* feat: implement core broadcasting logic\n\n* refactor: improve formatting of validate_extrinsic_format function\n\n* test(substrate): impl tsactional tests",
          "timestamp": "2026-01-20T00:49:06-05:00",
          "tree_id": "8282f75a50e7bab5edf28d0b36c8b60526a8fc00",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/4367757b5e7b70aba2be758011d209029076e70a"
        },
        "date": 1768888754063,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 11,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 13,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160875,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140831,
            "range": "± 356",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 116,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 23,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 585,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5829,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 572,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "790f6aa43b59d7205112886db7356b77de2e4baa",
          "message": "feat:  impl error for wallet and key (#108)\n\n* core(deps): add zeroize dependency with derive feature\n\n* feat(wallet): enhance mnemonic generation and seed handling with error checks\n\n* feat(config): improve interactive config initialization with error handling and default values\n\n* feat(config): enhance error handling for endpoints and add validation tests\n\n* docs: update README for clarity and improved presentation of Apex SDK features\n\n* feat(docs): create apex banner image to enhance visual presentation",
          "timestamp": "2026-01-20T16:23:27-05:00",
          "tree_id": "b0fbe4aa17b56ff8f3e47dcb8099cfed6a31cf5f",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/790f6aa43b59d7205112886db7356b77de2e4baa"
        },
        "date": 1768944823818,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161056,
            "range": "± 322",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141029,
            "range": "± 398",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 116,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 584,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5847,
            "range": "± 242",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 584,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c4fea64030b548a9fd902bd9a63633d0b3ada3de",
          "message": "feat: impl subscription based tx monitoring (#109)\n\n* feat(monitor): implement transaction monitoring with subscription-based handling\n\n* feat(monitor): add transaction monitoring with fallback polling strategy",
          "timestamp": "2026-01-21T00:08:21-05:00",
          "tree_id": "a1ff1f5b23c9ed0366f03a88bc90349025dc56d7",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/c4fea64030b548a9fd902bd9a63633d0b3ada3de"
        },
        "date": 1768972709266,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160976,
            "range": "± 503",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140833,
            "range": "± 308",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 108,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 541,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5392,
            "range": "± 267",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 548,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "28f05330a4d3ed644528bd65bd94545dda46bbbf",
          "message": "feat: enhance fee estimation (#110)\n\n* docs(README): update Apex SDK link\n\n* feat(fee-estimator): implement dynamic fee estimation module for Substrate transactions\n\n* feat(fee-estimator): add fee estimation capabilities to Substrate adapter\n\n* feat(fee-estimator): create dynamic fee estimation methods with multiple strategies\n\n* test(estimation): impl integration tests for enhanced fee estimation functionality\n\n* feat(example): create fee estimation example for Substrate\n\n* feat(example): create Cargo.toml for substrate-fees example",
          "timestamp": "2026-01-21T15:19:13-05:00",
          "tree_id": "7924ce27b5089862ffa5645c37d35ebaeae476d5",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/28f05330a4d3ed644528bd65bd94545dda46bbbf"
        },
        "date": 1769027524490,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161271,
            "range": "± 349",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141304,
            "range": "± 304",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 114,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 541,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5542,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 542,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8c658d5624075e8821517c67f3ed4535735448d6",
          "message": "feat: impl metrics and observability (#111)\n\n* feat(dash): impl dashboards for metrics observability and performance analysis\n\n* feat(metrics): impl health monitoring and performance profiling modules\n\n- Implemented health check and status monitoring in `health.rs` with comprehensive health status tracking for SDK components.\n- Created a performance profiling module in `profiling.rs` that supports OpenTelemetry-based tracing and operation timing.\n- Introduced a Prometheus metrics exporter in `prometheus_exporter.rs` for HTTP server integration and automatic metric registration.\n- Developed telemetry configuration and initialization in `telemetry.rs` to support structured logging and tracing.\n- Updated `lib.rs` to include new modules and provide a unified access point for observability features.\n\n* feat(docs): enhance docs\n\n* feat(metrics): add apex-sdk-metrics to workspace members\n\n* chore(deps): remove unused dependencies",
          "timestamp": "2026-01-27T12:48:25-05:00",
          "tree_id": "81c93f8019b4c3fdf93c172343b08d1a8d90378d",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/8c658d5624075e8821517c67f3ed4535735448d6"
        },
        "date": 1769536741844,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 56,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160777,
            "range": "± 366",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 140744,
            "range": "± 413",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 116,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 59,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 584,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5863,
            "range": "± 314",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 589,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "24a04f3fb4e0d5d51c32fe872e7636cd68b95043",
          "message": "release: V0.1.6 (#112)\n\n* feat(examples): update apex-sdk version to 0.1.6 in example projects\n\n* feat: update version to 0.1.6 across all packages and dependencies\n\n* feat: update version to 0.1.6 in Cargo.toml, CLI, and integration tests\n\n* feat: update latest version to v0.1.6 in documentation\n\n* feat: release V0.1.6",
          "timestamp": "2026-01-28T01:48:25-05:00",
          "tree_id": "d6f684899ce348414620096c4e3b5969c64a0b2e",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/24a04f3fb4e0d5d51c32fe872e7636cd68b95043"
        },
        "date": 1769583537947,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_creation/create_evm_transfer",
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/evm_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_evm_sdk",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161099,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/evm_sign_simulation",
            "value": 141086,
            "range": "± 321",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 112,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_evm_transaction",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 553,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5463,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/evm_bulk/10",
            "value": 551,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_chain_ids",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d96e32a931b44831e7f1ae944c6c86b3a008882c",
          "message": "feat: revive integration (#116)\n\n* refactor(evm) remove EVM Transfer and Parachain Assets Examples\n\n- Deleted the `evm-transfer` example, including its Cargo.toml, README.md, and main.rs files.\n- Removed the `parachain-assets` example, including its Cargo.toml, README.md, and main.rs files.\n- Cleaned up integration tests related to EVM functionality, including tests for connection, balance retrieval, transaction sending, and contract interaction.\n- Removed integration tests for the overall architecture of the Apex SDK, ensuring no remnants of the deleted examples remain.\n\n* refactor(evm): remove comprehensive tests for transaction execution and wallet operations from the apex-sdk-evm test suite\n\n* chore(docs): impl IndexNow integration and update software version in SEO script\n\n- Implemented IndexNow functionality to notify search engines of URL changes.\n- Added a check to prevent duplicate notifications within 24 hours.\n- Updated software version from 0.1.6 to 0.1.5 in structured data.\n- Included placeholder for Google Analytics 4 script.\n\n* feat(cli): update CLI to support Revive accounts and balance checking\n\n* feat(substrate): implement AssetManager for pallet-assets interaction\n\n-  add NftManager for NFT collection and item management\n\n- add address parsing utility for storage queries\n\n* feat(revive): integrate Revive support into Apex SDK\n\n- Replace EVM references with Revive in Cargo.toml and lib.rs\n- Update builder.rs to configure Revive endpoint and adapter\n- Modify sdk.rs to handle Revive transactions and remove EVM transaction logic\n- Refactor advanced.rs and transaction_benchmarks.rs to remove EVM benchmarks\n- Adjust working_integration_tests.rs to focus on Substrate and Revive\n\n* feat(revive): update telemetry initialization to include revive metrics\n\n* feat(integration-tests): replace apex-sdk-evm dependency with apex-sdk-revive and remove EVM transaction tests\n\n* feat(revive): implement Revive adapter with contract management and balance querying\n\n* feat(revive): update dependencies and modify README for Revive integration\n\n* feat(substrate): impl method to convert wallet to subxt-compatible signer\n\n* feat(revive): enhance ReviveAdapter with connection checks and transaction management methods\n\n* feat(adk): refine Revive transaction handling and enhance transaction builder with deployment support\n\n* feat(cli): enhance CLI with Revive contract deployment and balance checking functionality\n\n* feat(docs): enhance Revive contract deployment with transaction and read capabilities\n\n* feat(tests): add Revive integration tests and refine transaction test descriptions\n\n* refactor(tests): streamline logging in Phase 5 integration tests\n\n* fix: ci",
          "timestamp": "2026-02-08T01:08:50-05:00",
          "tree_id": "e7a443879de11b2febbbbe1878e01549e6bb2160",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/d96e32a931b44831e7f1ae944c6c86b3a008882c"
        },
        "date": 1770531422480,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 27,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 26,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 161123,
            "range": "± 845",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 596,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5964,
            "range": "± 397",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "44cf55e216fd3bfb3ef48b1c47fc2ab4bb924141",
          "message": "feat: revive docs (#117)\n\n* refactor(evm) remove EVM Transfer and Parachain Assets Examples\n\n- Deleted the `evm-transfer` example, including its Cargo.toml, README.md, and main.rs files.\n- Removed the `parachain-assets` example, including its Cargo.toml, README.md, and main.rs files.\n- Cleaned up integration tests related to EVM functionality, including tests for connection, balance retrieval, transaction sending, and contract interaction.\n- Removed integration tests for the overall architecture of the Apex SDK, ensuring no remnants of the deleted examples remain.\n\n* refactor(evm): remove comprehensive tests for transaction execution and wallet operations from the apex-sdk-evm test suite\n\n* chore(docs): impl IndexNow integration and update software version in SEO script\n\n- Implemented IndexNow functionality to notify search engines of URL changes.\n- Added a check to prevent duplicate notifications within 24 hours.\n- Updated software version from 0.1.6 to 0.1.5 in structured data.\n- Included placeholder for Google Analytics 4 script.\n\n* feat(cli): update CLI to support Revive accounts and balance checking\n\n* feat(substrate): implement AssetManager for pallet-assets interaction\n\n-  add NftManager for NFT collection and item management\n\n- add address parsing utility for storage queries\n\n* feat(revive): integrate Revive support into Apex SDK\n\n- Replace EVM references with Revive in Cargo.toml and lib.rs\n- Update builder.rs to configure Revive endpoint and adapter\n- Modify sdk.rs to handle Revive transactions and remove EVM transaction logic\n- Refactor advanced.rs and transaction_benchmarks.rs to remove EVM benchmarks\n- Adjust working_integration_tests.rs to focus on Substrate and Revive\n\n* feat(revive): update telemetry initialization to include revive metrics\n\n* feat(integration-tests): replace apex-sdk-evm dependency with apex-sdk-revive and remove EVM transaction tests\n\n* feat(revive): implement Revive adapter with contract management and balance querying\n\n* feat(revive): update dependencies and modify README for Revive integration\n\n* feat(substrate): impl method to convert wallet to subxt-compatible signer\n\n* feat(revive): enhance ReviveAdapter with connection checks and transaction management methods\n\n* feat(adk): refine Revive transaction handling and enhance transaction builder with deployment support\n\n* feat(cli): enhance CLI with Revive contract deployment and balance checking functionality\n\n* feat(docs): enhance Revive contract deployment with transaction and read capabilities\n\n* feat(tests): add Revive integration tests and refine transaction test descriptions\n\n* refactor(tests): streamline logging in Phase 5 integration tests\n\n* fix: ci\n\n* feat(docs): implement proper docs website\n\n* chore: update docs ci\n\n* chore: update deps",
          "timestamp": "2026-02-08T01:55:12-05:00",
          "tree_id": "bd28ec18d0b33e8ed0b57551b25a3f1f89d9bc27",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/44cf55e216fd3bfb3ef48b1c47fc2ab4bb924141"
        },
        "date": 1770534189518,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 27,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 28,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160945,
            "range": "± 402",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 599,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 6039,
            "range": "± 451",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c39af9742f021331d2aeb6b3402f4bb1588b6b8a",
          "message": "fix:  workers failure (#118)\n\n* fix: add workers/**\n\n* fix: broken mdx\n\n* fix ci\n\n* fix ci\n\n* fix: resolve ci failure\n\n* fix: resolve ci failure\n\n* fix: resolve ci failure\n\n* fix: resolve ci failure\n\n* fix: resolve ci failure\n\n* fix: resolve ci failure",
          "timestamp": "2026-02-08T07:03:12-05:00",
          "tree_id": "7983236dee63a5f898305a591cb6db410bb9f8f7",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/c39af9742f021331d2aeb6b3402f4bb1588b6b8a"
        },
        "date": 1770552663140,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 27,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 27,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160940,
            "range": "± 382",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 602,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 6156,
            "range": "± 422",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "kherld.hussein@gmail.com",
            "name": "kh3rld",
            "username": "kh3rld"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4755a5f246bf239c851ea45c9661940cdad81101",
          "message": "docs: overhaul for sys chain std lib (#119)",
          "timestamp": "2026-02-08T16:31:24-05:00",
          "tree_id": "692231698a916421c485a782469f2d0d090966d7",
          "url": "https://github.com/apex-sdk/apex-sdk/commit/4755a5f246bf239c851ea45c9661940cdad81101"
        },
        "date": 1770586763325,
        "tool": "cargo",
        "benches": [
          {
            "name": "transaction_creation/create_substrate_transfer",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/0",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/1",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "address_validation/substrate_validation/2",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_substrate_sdk",
            "value": 28,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sdk_initialization/create_multi_chain_sdk",
            "value": 28,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "transaction_signing/substrate_sign_simulation",
            "value": 160841,
            "range": "± 339",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_creation",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "amount_operations/amount_arithmetic",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/chain_type_detection",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/multi_chain_transaction_creation",
            "value": 59,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cross_chain_operations/cross_chain_address_handling",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "hybrid_chain_operations/hybrid_chain_type_check",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/10",
            "value": 593,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "bulk_transaction_creation/substrate_bulk/100",
            "value": 5891,
            "range": "± 381",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/dot_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/ksm_to_planck",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/eth_to_wei",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/planck_to_dot",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decimal_conversions/wei_to_eth",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/get_rpc_endpoints",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "chain_metadata_operations/check_smart_contract_support",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}