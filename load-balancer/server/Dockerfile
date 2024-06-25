FROM rust:1.77.2-buster as builder
WORKDIR /app
COPY .. .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

EXPOSE 8081

CMD ["server"]