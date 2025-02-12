FROM rust:alpine as builder

WORKDIR /app/src
RUN USER=root

RUN apk add --no-cache pkgconfig build-base musl-dev libressl-dev
COPY ./ ./
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
RUN apk update && apk add openssl ca-certificates

COPY --from=node-builder /usr/src/dist /app/dist
COPY --from=builder /app/src/target/release/app_name /app/app_name
COPY --from=builder /app/src/config /app/config

ENTRYPOINT ["/app/app_name"]
CMD ["start"]