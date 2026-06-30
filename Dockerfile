FROM rust:alpine AS builder
WORKDIR /aurorite

COPY ./Cargo.toml /aurorite
COPY ./Cargo.lock /aurorite
COPY ./crates /aurorite/crates

RUN cargo build --release

FROM alpine:3.24
WORKDIR /aurorite

RUN apk update && apk add weasyprint && apk cache clean && rm -f /sbin/apk && rm -rf /etc/apk && rm -rf /lib/apk && rm -rf /usr/share/apk && rm -rf /var/lib/apk && rm -rf /var

COPY --from=builder /aurorite/target/release/aurorite-server .

ENTRYPOINT ["./aurorite-server"]