FROM rust:1.77.2-buster as builder
WORKDIR /app
COPY .. .
RUN cargo install --path .

FROM debian:buster-slim as runner

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/load-balancer /usr/local/bin/load-balancer
COPY configuration /load-balancer/configuration

EXPOSE 8080

CMD ["load-balancer"]