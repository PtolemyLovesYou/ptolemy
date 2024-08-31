FROM python:3.11-bullseye

WORKDIR /app

COPY . .

# setup poetry
RUN pip install --upgrade pip && \
    pip install poetry==1.8.3 && \
    poetry config virtualenvs.create false

# install dependencies
RUN poetry install \
    --no-dev \
    -E api

CMD ["gunicorn", "tvali_app.asgi", "-b", "0.0.0.0:8000", "--chdir", "/app/tvali_app/"]
