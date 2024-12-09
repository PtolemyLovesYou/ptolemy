# Build stage
FROM python:3.12.6-slim-bookworm AS build

WORKDIR /app

ENV PYTHONFAULTHANDLER=1 \
    PYTHONUNBUFFERED=1 \
    PYTHONHASHSEED=random \
    PIP_NO_CACHE_DIR=off \
    PIP_DISABLE_PIP_VERSION_CHECK=on \
    PIP_DEFAULT_TIMEOUT=100 \
    # Poetry config
    POETRY_NO_INTERACTION=1 \
    POETRY_VIRTUALENVS_CREATE=false \
    POETRY_CACHE_DIR='/var/cache/pypoetry' \
    POETRY_HOME='/usr/local' \
    POETRY_VERSION=1.8.4

RUN pip install -U pip setuptools && \
    pip install "poetry==${POETRY_VERSION}"

ARG ENV=DEV

COPY poetry.lock pyproject.toml .

RUN poetry install \
    --no-cache \
    --no-root \
    --no-interaction \
    --no-ansi \
    --all-extras \
    $(if [ "$ENV" = "DEV" ]; then echo "--with dev"; fi)

# Runtime
FROM python:3.12.6-slim-bookworm AS runtime

# Final image
FROM runtime

WORKDIR /app

COPY --from=build /usr/local /usr/local

COPY . .
