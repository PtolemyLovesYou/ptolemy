# Build stage
FROM rust:1.82-bookworm as builder

ENV BUILD_PROTOBUFS=0 \
    COMPILE_GQL=0 \
    PTOLEMY_ENV=PROD

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY ./api ./api
COPY ./ptolemy ./ptolemy

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build -p api --bin api --release

# Runtime stage
FROM debian:bookworm-slim

ENV API_PORT=8000 \
    PTOLEMY_ENV=PROD \
    PTOLEMY_USER=admin \
    PTOLEMY_PASS=admin

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
