# Build stage
FROM rust:1.82-slim-bookworm as builder-base

WORKDIR /app

# Install necessary build dependencies including PostgreSQL client libraries
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        libpq-dev \
        libssl-dev \
        pkg-config \
        && rm -rf /var/lib/apt/lists/*

ENV BUILD_PROTOBUFS=0

ENV PTOLEMY_ENV=PROD

# Create mount points for cargo registry caching
RUN mkdir -p /usr/local/cargo/registry /usr/local/cargo/git

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install sccache

FROM builder-base as builder

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV SCCACHE_DIR=/sccache

# Install diesel CLI with caching
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
   --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install diesel_cli --no-default-features --features postgres

# Copy workspace Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy member Cargo.toml files
COPY ./api/Cargo.toml ./api/Cargo.toml
COPY ./ptolemy/Cargo.toml ./ptolemy/Cargo.toml

# Copy the real source code
COPY ./api ./api
COPY ./ptolemy ./ptolemy

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
# Modify the COPY command to handle both release and debug builds
COPY --from=builder /app/target/*/api /usr/local/bin/api

RUN useradd -m -u 1001 appuser

RUN chmod +x /usr/local/bin/api \
    && chown appuser:appuser /usr/local/bin/api

COPY ./api/entrypoint.sh /entrypoint.sh
RUN chmod 755 /entrypoint.sh && \
    chown appuser:appuser /entrypoint.sh

USER appuser

ENV API_PORT=8000
ENV OBSERVER_PORT=50051
ENV PTOLEMY_ENV=PROD
ENV PTOLEMY_USER=admin
ENV PTOLEMY_PASS=admin

WORKDIR /app

ENTRYPOINT ["/entrypoint.sh"]
