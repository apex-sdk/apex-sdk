window.BENCHMARK_DATA = {
  "lastUpdate": 1766687347803,
  "repoUrl": "https://github.com/carbobit/apex-sdk",
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
      }
    ]
  }
}