FROM rust:1.85.0-bookworm AS builder
WORKDIR /app
COPY .. .
RUN cargo install --path .


FROM debian:bookworm-slim AS runner
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

EXPOSE 8081

CMD ["server"]