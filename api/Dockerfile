FROM rust:1.82-slim-bookworm as base

ARG SCCACHE_VERSION=0.9.1

WORKDIR /app

ENV BUILD_PROTOBUFS=0 \
    PTOLEMY_ENV=PROD \
    RUSTC_WRAPPER=/usr/local/cargo/bin/sccache \
    SCCACHE_DIR=/sccache \
    CARGO_NET_GIT_FETCH_WITH_CLI=true

# Install necessary build dependencies
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        libpq-dev \
        libssl-dev \
        pkg-config \
        && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y --no-install-recommends curl && \
    curl -L https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz | \
    tar xz --strip-components=1 -C /usr/local/cargo/bin sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache && \
    chmod +x /usr/local/cargo/bin/sccache && \
    apt-get purge -y curl && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

# Install sccache in one layer
RUN apt-get update && apt-get install -y --no-install-recommends curl && \
    curl -L https://github.com/mozilla/sccache/releases/download/v${SCCACHE_VERSION}/sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz | \
    tar xz --strip-components=1 -C /usr/local/cargo/bin sccache-v${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache && \
    chmod +x /usr/local/cargo/bin/sccache && \
    apt-get purge -y curl && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

# Install diesel CLI
RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install diesel_cli --no-default-features --features postgres

FROM base as builder

# Copy dependency files first
COPY Cargo.* ./
COPY ./dummy /app/dummy/

# build dependencies
RUN --mount=type=bind,source=./api,target=/app/api \
    --mount=type=bind,source=./ptolemy,target=/app/ptolemy \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=./target \
    cargo build -p dummy --release

# Copy source and build
COPY ./dummy /app/dummy/
COPY ./api /app/api/
COPY ./ptolemy /app/ptolemy/

RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --bin api --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -m -u 1001 appuser

COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/
COPY --from=builder /app/target/release/api /usr/local/bin/
COPY api/entrypoint.sh /

RUN mkdir -p /ptolemy/data

RUN chmod 755 /entrypoint.sh /usr/local/bin/api && \
    chown appuser:appuser /entrypoint.sh /usr/local/bin/api && \
    chown appuser:appuser /ptolemy/data

USER appuser

ENV API_PORT=8000 \
    PTOLEMY_ENV=PROD \
    PTOLEMY_USER=admin \
    PTOLEMY_PASS=admin \
    PTOLEME_DATA=/ptolemy/data

WORKDIR /app

ENTRYPOINT ["/entrypoint.sh"]
