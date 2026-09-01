# AGENTS.md — rate

## What this is
Rust-based currency exchange rate service (mrate) that fetches and serves real-time exchange rates for 30+ currencies. Runs as a systemd service.

## Stack
- Rust (edition 2024)
- Axum (web framework)
- rusqlite (SQLite database)
- reqwest (HTTP client)
- tokio (async runtime)
- tower-http (CORS)

## Build
```bash
cargo build --release
```

## Run
```bash
cargo run --release
```
Or install as systemd service: `systemctl start mrate`

## Structure
- `src/main.rs` — entry point
- `src/lib.rs` — library exports
- `src/rate.rs` — rate logic
- `src/exchange.rs` — exchange rate fetching
- `src/database.rs` — SQLite operations
- `src/help.rs` — help text
- `src/test.rs` — tests
- `rate.db` — SQLite database
- `Cargo.toml` — Rust dependencies
- `Makefile` — build shortcuts
- `ask.sh` — task menu

## Conventions
- No comments in code unless asked.
- Verify: `cargo check && cargo build`
