# Build stage
FROM rust:1.87 AS chef
USER root
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Runtime with modern Ubuntu
FROM ubuntu:22.04 AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/device-comms /usr/local/bin

EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/device-comms"]