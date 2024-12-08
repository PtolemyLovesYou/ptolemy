FROM golang:1.23-alpine

WORKDIR /app

ENV GOOSE_MIGRATION_DIR=/app/migrations

RUN apk update && \
    apk upgrade && \
    apk add bash && \
    rm -rf /var/cache/apk/*

ADD https://github.com/pressly/goose/releases/download/v3.23.0/goose_linux_x86_64 /bin/goose
RUN chmod +x /bin/goose

COPY ./migrations .

CMD ["tail", "-f", "/dev/null"]
