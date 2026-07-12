# Bitcoin Mempool Tracker

A Rust backend that proxies JSON-RPC calls to a local Bitcoin Core node and exposes them as a REST API via [Axum](https://github.com/tokio-rs/axum).

> **Status:** Work in progress. Several route handlers are still being wired up and do not yet compile end-to-end (see [Known Issues](#known-issues)).

## Overview

The service sits between a client and a Bitcoin Core node's JSON-RPC interface, translating simple HTTP GET requests into `bitcoind` RPC calls and returning the JSON response.

## Tech Stack

- **Rust** (2024 edition)
- **[Axum](https://github.com/tokio-rs/axum)** – HTTP routing/server
- **[Tokio](https://tokio.rs/)** – async runtime
- **[Reqwest](https://github.com/seanmonstar/reqwest)** – HTTP client used to call the Bitcoin RPC node
- **Serde / serde_json** – (de)serialization
- **dotenv** – environment variable loading
- **tower-http** – CORS support

## Project Structure

```
backend/
├── src/
│   ├── main.rs                     # Merges all route modules and starts the server
│   ├── model/                      # RPC response structs per endpoint
│   │   ├── block_count.rs
│   │   ├── estimate_smart_fee.rs
│   │   ├── mempool_info.rs
│   │   ├── raw_mempool.rs
│   │   └── raw_transaction.rs
│   └── routes/                     # Route handlers that call the RPC node
│       ├── block_count_route.rs
│       ├── estimate_smart_fee_route.rs
│       ├── mempool_info_route.rs
│       ├── raw_mempool_route.rs
│       └── raw_transaction_route.rs
└── Cargo.toml
```

## API Endpoints

| Method | Path                                    | Bitcoin RPC method   | Description                              |
|--------|------------------------------------------|-----------------------|-------------------------------------------|
| GET    | `/model/block_count_route`               | `getblockcount`       | Current block height                      |
| GET    | `/model/mempool_info`                    | `getmempoolinfo`      | Summary stats about the node's mempool    |
| GET    | `/model/estimate_smart_fee_route/:target`| `estimatesmartfee`    | Estimated fee rate for confirmation within `target` blocks |
| GET    | `/model/raw_transaction`                 | `getrawtransaction`   | Raw transaction data                      |
| GET    | `/model/raw_mempool`                     | `getrawmempool`       | Full list of mempool transaction IDs      |

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- A reachable Bitcoin Core node with the JSON-RPC server enabled (defaults to `http://127.0.0.1:8332` in each route file)

## Setup

1. Clone the repo and move into the backend crate:

   ```sh
   cd backend
   ```

2. Create a `.env` file in the project root with your own values (do **not** commit this file):

   ```
   GET_BLOCK = "your-getblock.io-endpoint"
   BITCOIN = "http://127.0.0.1:8332"
   ```

3. Build and run:

   ```sh
   cargo run
   ```

   The server listens on `0.0.0.0:3000`.

## Known Issues

This project is under active development. As of now:

- `raw_transaction_route.rs` and `raw_mempool_route.rs` have handler signatures/bodies that don't yet match Axum's extractor conventions and will not compile.
- `mempool_info_route.rs` returns the RPC struct directly instead of wrapping it in `axum::Json`, so it won't satisfy `IntoResponse`.
- The Bitcoin RPC node URL is currently hardcoded as a constant in each route file rather than read from the `.env` values.
- No authentication is implemented for the Bitcoin Core RPC calls (Bitcoin Core normally requires RPC user/password credentials).

## Security Note

The `.env` file in this repo is currently tracked by git and should be removed from version control and rotated if it contains real credentials. Add `.env` to `.gitignore` and use `.env.example` for documenting required variables instead.

## License

No license specified yet.
