# Device Monitoring Service Dockerfile
# 
# Multi-stage Docker build for the device monitoring service that provides
# a REST API for retrieving IoT device telemetry data from Azure Cosmos DB.
# 
# Build Process:
# 1. Chef stage: Sets up Rust toolchain and cargo-chef for dependency caching
# 2. Planner stage: Analyzes dependencies and creates a recipe for efficient builds
# 3. Builder stage: Compiles the application with optimized dependencies
# 4. Runtime stage: Creates minimal runtime image with only necessary components

# Build stage - Chef setup
FROM rust:1.87 AS chef
USER root
# Install system dependencies required for building
RUN apt-get update && apt-get install -y pkg-config libssl-dev
# Install cargo-chef for efficient dependency caching
RUN cargo install cargo-chef --locked
WORKDIR /app

# Planner stage - Dependency analysis
FROM chef AS planner
COPY . .
# Analyze dependencies and create a recipe for efficient builds
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage - Application compilation
FROM chef AS builder
# Copy the dependency recipe from planner stage
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies first (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json
# Copy source code and build the application
COPY . .
RUN cargo build --release

# Runtime stage - Minimal production image
FROM ubuntu:22.04 AS runtime
WORKDIR /app
# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/device-monitor /usr/local/bin

# Expose the service port (matches the port in lib.rs)
EXPOSE 8001
# Set the entry point to run the device monitoring service
ENTRYPOINT ["/usr/local/bin/device-monitor"]