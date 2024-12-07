FROM golang:1.23-alpine

WORKDIR /app

ENV GOOSE_MIGRATION_DIR=/app/migrations

COPY ./migrations .
COPY ./go.mod .
COPY ./go.sum .

# install go dependencies
RUN go mod download

CMD ["tail", "-f", "/dev/null"]
