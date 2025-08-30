# Build stage
FROM rust:1.82-bookworm AS builder

ENV BUILD_PROTOBUFS=0

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends cmake

COPY Cargo.toml Cargo.lock ./
COPY ./api ./api
COPY ./ptolemy ./ptolemy
COPY ./ptolemy-py ./ptolemy-py

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build -p api --bin api --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -m -u 1001 appuser

COPY --from=builder /app/target/release/api /usr/local/bin/

RUN mkdir -p /ptolemy/data

RUN chmod 755 /usr/local/bin/api && \
    chown appuser:appuser /usr/local/bin/api && \
    chown appuser:appuser /ptolemy/data

USER appuser

WORKDIR /app

CMD ["/usr/local/bin/api"]
