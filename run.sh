#!/bin/bash

export SERVER_LISTEN_ADDR="127.0.0.1"
export SERVER_LISTEN_PORT="8089"
export SERVER_WORKER_THREADS="10"
export SERVER_BASE_URL="http://localhost:8089"
export SERVER_DATA_ROOT="data"

export API_ADMIN_ACCESS_TOKEN="tamed-busman-want-vendetta"

export DOCKER_RUN_BASE_URL="http://localhost:8088"
export DOCKER_RUN_ACCESS_TOKEN="magmatic-handyman-confirm-cauldron"

export RUST_LOG=debug

#export RUSTFLAGS=-Awarnings
cargo run
