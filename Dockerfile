FROM rust:alpine as builder

RUN apk add musl-dev

COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=builder target/release/loggrep /loggrep/loggrep

RUN chown -R guest /loggrep

USER guest
WORKDIR ./loggrep

CMD ["./loggrep"]
