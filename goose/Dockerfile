FROM golang:1.23-alpine

WORKDIR /app

# set default clickhouse
ENV DB=clickhouse

RUN apk update && \
    apk upgrade && \
    apk add bash && \
    rm -rf /var/cache/apk/*

ADD https://github.com/pressly/goose/releases/download/v3.23.0/goose_linux_x86_64 /bin/goose
RUN chmod +x /bin/goose

COPY . .

CMD ["tail", "-f", "/dev/null"]
