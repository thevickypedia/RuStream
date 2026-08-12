FROM rust:1.97-alpine AS builder

RUN apk add --no-cache build-base perl

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY README.md ./
COPY src ./src

RUN cargo build --release --locked

FROM alpine:3.22

WORKDIR /app

RUN addgroup -S rustream && adduser -S rustream -G rustream \
    && apk add --no-cache ffmpeg \
    && mkdir -p /data/media \
    && chown -R rustream:rustream /data/media

COPY --from=builder /build/target/release/rustream /usr/local/bin/rustream

USER rustream

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/rustream"]
