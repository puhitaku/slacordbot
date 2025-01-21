FROM rust:1.79-alpine as builder
WORKDIR /app
COPY . .
RUN apk update && apk add g++ && cargo build --release

FROM busybox:stable-musl
WORKDIR /app
COPY --from=builder /app/target/release/slacordbot /usr/local/bin/slacordbot
CMD ["slacordbot"]
