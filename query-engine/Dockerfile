FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim AS base

WORKDIR /app

COPY pyproject.toml uv.lock ./
COPY ./query-engine ./query-engine
RUN uv init ptolemy-py

RUN uv sync --frozen --directory query-engine --inexact

CMD ["uv", "run", "--directory", "query-engine", "main.py"]
