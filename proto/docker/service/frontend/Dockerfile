################################################################################
# Sycamore frontend server.
################################################################################
FROM rust_base AS base

RUN apt-get install -y libssl-dev pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
