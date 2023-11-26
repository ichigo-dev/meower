################################################################################
# Sycamore frontend server.
################################################################################
FROM rust:1.73.0 AS base

RUN apt-get update \
    && apt-get install -y libssl-dev pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk grass cargo-udeps cargo-crev cargo-audit
