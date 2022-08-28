FROM rust:1.63-slim as rust-builder
WORKDIR /usr/src/masacarri
COPY ./masacarri .

RUN apt update
RUN apt install libssl-dev libpq-dev pkg-config
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=rust-builder /usr/local/cargo/bin/masacarri /usr/local/bin/masacarri
CMD ["masacarri"]
