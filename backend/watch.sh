#!/bin/bash
docker-compose up --detach
cargo watch -s "./reset.sh && cargo clippy && cargo build && RUST_BACKTRACE=1 cargo test && RUST_BACKTRACE=1 cargo run"