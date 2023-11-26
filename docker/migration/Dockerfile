################################################################################
# Database migration.
################################################################################
FROM rust:1.73.0 AS base

RUN apt-get update

RUN cargo install --locked sea-orm-cli cargo-udeps cargo-crev cargo-audit
