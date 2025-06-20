# Device Configuration Service Dockerfile
# 
# This Dockerfile uses a multi-stage build process to create an optimized
# container image for the device configuration service. It includes:
# - Dependency caching with cargo-chef for faster builds
# - Release build optimization
# - Minimal runtime image based on Ubuntu 22.04

# Build stage 1: Setup Rust environment and cargo-chef
FROM rust:1.87 AS chef
USER root
# Install system dependencies required for building
RUN apt-get update && apt-get install -y pkg-config libssl-dev
# Install cargo-chef for dependency caching
RUN cargo install cargo-chef --locked
WORKDIR /app

# Build stage 2: Plan dependencies for caching
FROM chef AS planner
COPY . .
# Generate dependency recipe for cargo-chef
RUN cargo chef prepare --recipe-path recipe.json

# Build stage 3: Build the application
FROM chef AS builder
# Copy dependency recipe from planner stage
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies first (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json
# Copy source code and build the application
COPY . .
RUN cargo build --release

# Runtime stage: Create minimal production image
FROM ubuntu:22.04 AS runtime
WORKDIR /app
# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
# Copy the built binary from builder stage
COPY --from=builder /app/target/release/device-config /usr/local/bin

# Expose the port the service runs on
EXPOSE 8001
# Set the entry point to run the service
ENTRYPOINT ["/usr/local/bin/device-config"]