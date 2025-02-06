FROM rust:1.82-slim-bookworm as builder

WORKDIR /app

RUN cargo init dummy

WORKDIR /app/dummy

RUN cargo add duckdb
RUN cargo build --release
