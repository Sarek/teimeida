FROM rust:alpine AS builder

RUN apk add musl-dev
WORKDIR /src
COPY . /src

RUN cargo build --bins --release

FROM scratch

COPY --from=builder /src/target/release/teimeida /
COPY --from=builder /src/assets /assets

EXPOSE 8080/tcp
VOLUME /data

CMD ["/teimeida"]
