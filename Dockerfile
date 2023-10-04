FROM rust:alpine AS builder

RUN apk add musl-dev
WORKDIR /src
COPY . /src

RUN cargo build --bins --release

FROM scratch

COPY --from=builder /src/target/release/teimeida /
COPY --from=builder /src/assets /assets
COPY --from=builder /src/templates /templates
COPY --from=builder /src/config /config

EXPOSE 8080/tcp
VOLUME /data
VOLUME /config

CMD ["/teimeida"]
