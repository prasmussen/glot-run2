#!/bin/bash

export SERVER_LISTEN_ADDR="127.0.0.1"
export SERVER_LISTEN_PORT="8089"
export SERVER_WORKER_THREADS="10"

export API_ACCESS_TOKEN="tamed-busman-want-vendetta"

export RUST_LOG=debug

export RUSTFLAGS=-Awarnings
cargo run
