# Build stage
FROM rust:1.82-slim-bookworm as builder

ARG SERVICE_NAME

WORKDIR /app

# Install only necessary build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        && rm -rf /var/lib/apt/lists/*

ENV BUILD_PROTOBUFS=0

# Copy workspace Cargo
COPY Cargo.toml Cargo.lock ./

# Copy member Cargo.toml files
COPY ./ptolemy-core/Cargo.toml ./ptolemy-core/Cargo.toml
COPY ./api/Cargo.toml ./api/Cargo.toml
COPY ./observer/Cargo.toml ./observer/Cargo.toml
COPY ./ptolemy/Cargo.toml ./ptolemy/Cargo.toml

# Create dummy executable files
RUN mkdir ./ptolemy-core/src ./api/src ./observer/src ./ptolemy/src \
    && echo "fn main() {}" > ./ptolemy-core/src/lib.rs \
    && echo "fn main() {}" > ./api/src/main.rs \
    && echo "fn main() {}" > ./observer/src/observer.rs \
    && echo "fn _core() {}" > ./ptolemy/src/lib.rs

# Create a dummy main.rs to build dependencies
RUN cargo build --bin ${SERVICE_NAME} --release && \
    rm -f target/release/deps/${SERVICE_NAME}*

# Now copy the real source code
COPY ./api ./api
COPY ./observer ./observer
COPY ./ptolemy ./ptolemy
COPY ./ptolemy-core ./ptolemy-core

# Build the actual binary
RUN cargo build --bin ${SERVICE_NAME} --release

# Runtime stage
FROM debian:bookworm-slim

ARG SERVICE_NAME
ENV SERVICE_NAME=${SERVICE_NAME}

# Create a directory for the app
WORKDIR /usr/local/bin

# Copy the binary from builder stage - note the explicit path
COPY --from=builder /app/target/release/${SERVICE_NAME} /usr/local/bin/${SERVICE_NAME}

# Create a non-root user
RUN useradd -m -u 1001 appuser

# Chown the binary
RUN chmod +x /usr/local/bin/${SERVICE_NAME} \
    && chown appuser:appuser /usr/local/bin/${SERVICE_NAME}

USER appuser

# Execute the binary using full path
CMD ["/bin/sh", "-c", "exec /usr/local/bin/${SERVICE_NAME}"]
