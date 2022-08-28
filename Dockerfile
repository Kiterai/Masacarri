FROM rust:1.63-slim as rustbuilder
WORKDIR /usr/src/masacarri
COPY ./masacarri .

RUN apt update && apt install libssl-dev libpq-dev pkg-config -y && cargo install --path .

FROM node:16-buster-slim as nodebuilder
WORKDIR /usr/src/app
COPY ./masacarri-front .

RUN npm install && npm run build-only

FROM debian:buster-slim
WORKDIR /usr/local/share/masacarri
RUN apt-get update && apt-get install -y libssl1.1 libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=rustbuilder /usr/local/cargo/bin/masacarri /usr/local/bin/masacarri
COPY --from=rustbuilder /usr/local/cargo/bin/masacarri_cli /usr/local/bin/masacarri_cli
COPY --from=nodebuilder /usr/src/app/dist /usr/local/share/masacarri-front/dist
COPY ./entry-point.sh .
CMD ["sh", "./entry-point.sh"]
