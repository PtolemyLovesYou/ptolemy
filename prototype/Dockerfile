FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim AS base

ARG RUST_VERSION=1.82.0
ARG MATURIN_VERSION=1.8.1
ARG TARGETARCH

WORKDIR /app

# Install necessary build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        libssl-dev \
        pkg-config \
        curl \
        tar \
        build-essential \
        patchelf \
        && rm -rf /var/lib/apt/lists/*

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain ${RUST_VERSION} --profile minimal && \
    . $HOME/.cargo/env

ENV PATH="/root/.cargo/bin:${PATH}"

# Install maturin
RUN case ${TARGETARCH} in \
    arm64) ARCH="aarch64" ;; \
    amd64) ARCH="x86_64" ;; \
    *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    curl -L "https://github.com/PyO3/maturin/releases/download/v${MATURIN_VERSION}/maturin-${ARCH}-unknown-linux-musl.tar.gz" | tar xz -C /usr/local/bin

RUN chmod +x /usr/local/bin/maturin

FROM base as builder

ARG CFFI_VERSION=1.17.1

ENV BUILD_PROTOBUFS=0 \
    UV_LINK_MODE=copy

COPY ./pyproject.toml ./uv.lock .
COPY ./Cargo.toml ./Cargo.lock .
COPY ./ptolemy ./ptolemy
COPY ./api ./api

RUN uv venv .venv

# Needs cffi :')
RUN uv pip install cffi==${CFFI_VERSION}

# Build ptolemy
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/root/.cache/uv \
    maturin develop --uv --features client -m /app/ptolemy/Cargo.toml

# Install the project's dependencies using the lockfile and settings
RUN --mount=type=cache,target=/root/.cache/uv \
    --mount=type=bind,source=./prototype/pyproject.toml,target=prototype/pyproject.toml \
    uv sync --frozen --no-install-project --package prototype --inexact

COPY ./prototype ./prototype

RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --package prototype --inexact

# Use a Python image with uv pre-installed
FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim

# Install the project into `/app`
WORKDIR /app

# Copy venv from build stage
COPY --from=builder /app/.venv /app/.venv

# Then, add the rest of the project source code and install it
COPY pyproject.toml uv.lock .
COPY ./ptolemy ./ptolemy
COPY ./prototype ./prototype

ENV STREAMLIT_BROWSER_GATHER_USAGE_STATS=false \
    STREAMLIT_BROWSER_ADDRESS=0.0.0.0

CMD ["uv", "run", "--package", "prototype", "--directory", "prototype", "-m", "streamlit", "run", "app.py"]
