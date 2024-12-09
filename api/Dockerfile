# Build stage
FROM rust:1.82-slim-bookworm as builder

WORKDIR /app

# Install necessary build dependencies including PostgreSQL client libraries
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        libpq-dev \
        && rm -rf /var/lib/apt/lists/*

# Install diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy only files needed for dependency resolution first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --bin api --release && \
    rm -f target/release/deps/api*

# Now copy the real source code
COPY . .

# Build the actual binary
RUN cargo build --bin api --release

# Runtime stage
FROM debian:bookworm-slim

ENV RUST_LOG=info

# Install runtime dependencies for PostgreSQL
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libpq5 \
        && rm -rf /var/lib/apt/lists/*

# Create a directory for the app
WORKDIR /app

# Copy both the api binary and diesel CLI from builder stage
COPY --from=builder /app/target/release/api /usr/local/bin/api
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

COPY configure.sh /app/configure.sh

# Create a non-root user
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /usr/local/bin/api /usr/local/bin/diesel
USER appuser

# Execute the binary using full path
CMD ["/usr/local/bin/api"]
