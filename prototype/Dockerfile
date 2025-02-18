# Base stage
FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim AS base

ARG RUST_VERSION=1.82.0
ARG MATURIN_VERSION=1.8.1
ARG TARGETARCH

WORKDIR /app

# Combined RUN commands and cleaned up after installation
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        protobuf-compiler \
        libssl-dev \
        pkg-config \
        curl \
        tar \
        build-essential \
        patchelf && \
    rm -rf /var/lib/apt/lists/* && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain ${RUST_VERSION} --profile minimal && \
    case ${TARGETARCH} in \
        arm64) ARCH="aarch64" ;; \
        amd64) ARCH="x86_64" ;; \
        *) echo "Unsupported architecture: ${TARGETARCH}" && exit 1 ;; \
    esac && \
    curl -L "https://github.com/PyO3/maturin/releases/download/v${MATURIN_VERSION}/maturin-${ARCH}-unknown-linux-musl.tar.gz" | tar xz -C /usr/local/bin && \
    chmod +x /usr/local/bin/maturin

ENV PATH="/root/.cargo/bin:${PATH}"

# Builder stage
FROM base as builder

WORKDIR /app

ARG CFFI_VERSION=1.17.1

ENV BUILD_PROTOBUFS=0 \
    UV_LINK_MODE=copy

RUN uv venv .venv && \
    uv pip install cffi==${CFFI_VERSION}

COPY Cargo.toml Cargo.lock ./
COPY pyproject.toml uv.lock ./

COPY ./api/ ./api/
COPY ./ptolemy/ ./ptolemy/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/root/.cache/uv \
    maturin develop --uv -m /app/ptolemy/Cargo.toml

COPY ./prototype ./prototype

RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --package prototype --inexact

# Runtime stage
FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim

WORKDIR /app

COPY --from=builder /app/.venv /app/.venv
COPY --from=builder /app/ptolemy /app/ptolemy
COPY --from=builder /app/prototype /app/prototype
COPY pyproject.toml uv.lock ./

ENV STREAMLIT_BROWSER_GATHER_USAGE_STATS=false \
    STREAMLIT_BROWSER_ADDRESS=0.0.0.0

CMD ["uv", "run", "--no-sync", "--frozen", "--package", "prototype", "--directory", "prototype", "-m", "streamlit", "run", "app.py"]
