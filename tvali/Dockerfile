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

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    software-properties-common \
    git \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

RUN pip install -U pip setuptools && \
    pip install "poetry==${POETRY_VERSION}"

FROM build

WORKDIR /app

ARG ENV=DEV

COPY poetry.lock pyproject.toml .

RUN poetry install \
    --no-cache \
    --no-root \
    --no-interaction \
    --no-ansi \
    --all-extras \
    $(if [ "$ENV" = "DEV" ]; then echo "--with dev"; fi)

COPY . .

CMD ["uvicorn", "tvali.api.main:app", "--host", "0.0.0.0", "--port", "8000"]
