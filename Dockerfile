# https://hub.docker.com/r/docker/dockerfile
# syntax=docker/dockerfile:1.21

ARG RUST_VERSION="${RUST_VERSION:-1.93.1-alpine3.20}"
ARG ALPINE_VERSION="${ALPINE_VERSION:-3.23.3}"

# build stage
FROM rust:${RUST_VERSION} AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

# final stage
FROM alpine:${ALPINE_VERSION}
COPY --from=builder /app/target/release/hello-rust /usr/local/bin/
EXPOSE 8080
CMD ["hello-rust"]
