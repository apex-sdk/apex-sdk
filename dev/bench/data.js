window.BENCHMARK_DATA = {
  "lastUpdate": 1764044436769,
  "repoUrl": "https://github.com/kherldhussein/apex-sdk",
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
      }
    ]
  }
}