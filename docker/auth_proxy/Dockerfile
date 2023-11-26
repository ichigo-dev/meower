################################################################################
# Reverse proxy server for authentication.
################################################################################
FROM rust:1.73.0 AS base

RUN apt-get update

RUN cargo install --locked cargo-watch cargo-udeps cargo-crev cargo-audit
