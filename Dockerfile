FROM rust:1.91.0 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev pkg-config && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY firefleeb_api/Cargo.toml firefleeb_api/Cargo.toml
RUN mkdir -p firefleeb_api/src && echo "fn main() {}" > firefleeb_api/src/main.rs
RUN cargo fetch

COPY . .
RUN cargo build --release -p firefleeb_api

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libpq5 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/firefleeb_api /usr/local/bin/firefleeb_api

ENV RUST_LOG=info \
    PORT=8080

EXPOSE 8080

ENTRYPOINT ["firefleeb_api"]
