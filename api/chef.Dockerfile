# Base stage with cargo-chef
FROM rust:1.82-slim-bookworm as chef-base
WORKDIR /app

ARG CARGO_CHEF_VERSION=0.1.68

# Install necessary build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libssl-dev \
        pkg-config \
        && rm -rf /var/lib/apt/lists/*

# Download and install cargo-chef
ADD https://github.com/LukeMathWalker/cargo-chef/releases/download/v${CARGO_CHEF_VERSION}/cargo-chef-x86_64-unknown-linux-musl.tar.gz /tmp/cargo-chef.tar.gz
RUN tar -xzf /tmp/cargo-chef.tar.gz -C /usr/local/cargo/bin && \
    chmod +x /usr/local/cargo/bin/cargo-chef && \
    rm /tmp/cargo-chef.tar.gz

# Chef planner stage
FROM chef-base as planner
# Copy workspace files for recipe creation
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Chef builder stage
FROM chef-base as cacher
# Copy the recipe from planner
COPY --from=planner /app/recipe.json recipe.json
# Compute dependencies
RUN cargo chef cook --release --recipe-path recipe.json

# Builder stage
FROM rust:1.82-slim-bookworm as builder

ARG SCCACHE_VERSION=0.9.1

WORKDIR /app

# Install necessary build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        libpq-dev \
        libssl-dev \
        pkg-config \
        && rm -rf /var/lib/apt/lists/*

ENV BUILD_PROTOBUFS=0 \
    PTOLEMY_ENV=PROD

# Create mount points for cargo registry caching
RUN mkdir -p /usr/local/cargo/registry /usr/local/cargo/git

# Download and install sccache binary
ADD https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz /tmp/sccache.tar.gz
RUN tar -xzf /tmp/sccache.tar.gz -C /tmp && \
    mv /tmp/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache /usr/local/cargo/bin/sccache && \
    chmod +x /usr/local/cargo/bin/sccache && \
    rm -rf /tmp/sccache.tar.gz /tmp/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache \
    SCCACHE_DIR=/sccache

# Copy dependencies from chef cacher
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Install diesel CLI
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install diesel_cli --no-default-features --features postgres

# Copy the real source code
COPY . .

# Build the actual binary with conditional release flag
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    /bin/sh -c 'if [ "$PTOLEMY_ENV" = "PROD" ]; then \
        cargo build --bin api --release; \
    else \
        cargo build --bin api; \
    fi'

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libpq5 \
        && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /app/target/*/api /usr/local/bin/api

RUN useradd -m -u 1001 appuser

RUN chmod +x /usr/local/bin/api \
    && chown appuser:appuser /usr/local/bin/api

COPY ./api/entrypoint.sh /entrypoint.sh
RUN chmod 755 /entrypoint.sh && \
    chown appuser:appuser /entrypoint.sh

USER appuser

ENV API_PORT=8000 \
    PTOLEMY_ENV=PROD \
    PTOLEMY_USER=admin \
    PTOLEMY_PASS=admin

WORKDIR /app

ENTRYPOINT ["/entrypoint.sh"]
