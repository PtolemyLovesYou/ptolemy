# Build stage
FROM rust:1.82-slim-bookworm as builder

WORKDIR /app

# Install only necessary build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        && rm -rf /var/lib/apt/lists/*

# Copy only files needed for dependency resolution first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/serve.rs && \
    cargo build --bin serve --release && \
    rm -f target/release/deps/serve*

# Now copy the real source code
COPY . .

# Build the actual binary
RUN cargo build --bin serve --release

# Runtime stage
FROM debian:bookworm-slim

# Create a directory for the app
WORKDIR /usr/local/bin

# Copy the binary from builder stage - note the explicit path
COPY --from=builder /app/target/release/serve ./serve

# Create a non-root user
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /usr/local/bin/serve
USER appuser

# Execute the binary using full path
CMD ["/usr/local/bin/serve"]
