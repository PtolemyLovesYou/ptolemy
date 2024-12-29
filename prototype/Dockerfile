# Use a Python image with uv pre-installed
FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim

# Install the project into `/app`
WORKDIR /app

# Copy from the cache instead of linking since it's a mounted volume
ENV UV_LINK_MODE=copy

# Install the project's dependencies using the lockfile and settings
RUN --mount=type=cache,target=/root/.cache/uv \
    --mount=type=bind,source=./uv.lock,target=uv.lock \
    --mount=type=bind,source=./pyproject.toml,target=pyproject.toml \
    --mount=type=bind,source=./prototype/pyproject.toml,target=prototype/pyproject.toml \
    uv sync --frozen --no-install-project --package prototype

# Then, add the rest of the project source code and install it
# Installing separately from its dependencies allows optimal layer caching
COPY pyproject.toml uv.lock .
COPY ./prototype /app/prototype
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --package prototype

CMD ["uv", "run", "--package", "prototype", "--directory", "prototype", "-m", "streamlit", "run", "app.py", "--server.address=0.0.0.0"]
