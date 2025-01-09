# Build stage
FROM rust:1.82-slim-bookworm as builder

WORKDIR /app

# Install necessary build dependencies including PostgreSQL client libraries
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        libpq-dev \
        && rm -rf /var/lib/apt/lists/*

ENV BUILD_PROTOBUFS=0

# Install diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy workspace Cargo
COPY Cargo.toml Cargo.lock ./

# Copy member Cargo.toml files
COPY ./api/Cargo.toml ./api/Cargo.toml
COPY ./ptolemy/Cargo.toml ./ptolemy/Cargo.toml

# Create dummy executable files
RUN mkdir  ./api/src ./ptolemy/src \
    && echo "#![allow(dead_code)]\nfn main() {}" > ./api/src/main.rs \
    && echo "#![allow(dead_code)]\nfn _core() {}" > ./ptolemy/src/lib.rs

# Create a dummy main.rs to build dependencies
RUN cargo build --bin api --release && \
    rm -f target/release/deps/api*

# Now copy the real source code
COPY ./api ./api
COPY ./ptolemy ./ptolemy

# Build the actual binary
RUN cargo build --bin api --release

# Runtime stage
FROM debian:bookworm-slim

# set default clickhouse
ENV DB=clickhouse

# Install runtime dependencies for PostgreSQL
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libpq5 \
        && rm -rf /var/lib/apt/lists/*

# Create a directory for the app
WORKDIR /usr/local/bin

# Copy the binary from builder stage - note the explicit path
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /app/target/release/api /usr/local/bin/api

# Create a non-root user
RUN useradd -m -u 1001 appuser

# Chown the binary
RUN chmod +x /usr/local/bin/api \
    && chown appuser:appuser /usr/local/bin/api

# Before switching to appuser, copy and set up the entrypoint script
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

# Execute the binary using full path
# CMD ["/usr/local/bin/api"]
