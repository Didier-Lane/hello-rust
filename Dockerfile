FROM rust:1.93.1-alpine3.20 AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.23.3
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/hello-rust /usr/local/bin/
EXPOSE 8080
CMD ["hello-rust"]
