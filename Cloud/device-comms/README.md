# Device Communications Service

This service receives telemetry data from embedded devices and stores it in Azure Cosmos DB.

## Overview

The Device Communications Service acts as the entry point for telemetry data from IoT devices. It provides an HTTP API for devices to submit their readings, authenticates requests, and persists the data in Cosmos DB for later analysis.

## Features

- RESTful API for telemetry data ingestion
- Azure authentication and authorization
- Cosmos DB integration for data storage
- Structured logging
- Docker containerization
- Error handling and validation

## Project Structure

- `src/`
  - `app_state/` - Application state and dependency injection
  - `domain/` - Domain models and types
  - `routes/` - API endpoint definitions
  - `services/` - Business logic and external service integrations
  - `utils/` - Utility functions and helpers
- `scripts/` - Development and deployment scripts
- `tests/` - Integration tests

## API Endpoints

### POST /api/telemetry

Accepts telemetry data from devices with the following structure:

```json
{
  "device_id": "device-123",
  "timestamp": "2023-06-20T12:34:56Z",
  "temperature": 25.5,
  "voltage": 3.3
}
```

## Local Development

### Prerequisites

- Rust toolchain
- Docker and Docker Compose
- Azure Cosmos DB emulator or connection string

### Running locally

```bash
# Run with environment variables
COSMOS_ENDPOINT=<your-cosmos-endpoint> \
COSMOS_KEY=<your-cosmos-key> \
COSMOS_DATABASE=<your-database-name> \
COSMOS_CONTAINER=<your-container-name> \
cargo run

# Or use the Docker script
./scripts/local-docker.sh
```

### Testing

```bash
cargo test
```

## Deployment

The service is designed to be deployed as a container to Azure Container Apps. See the pipeline definition in `/Pipelines/build-device-comms.yml`.

## Configuration

The service is configured through environment variables:

- `COSMOS_ENDPOINT` - Cosmos DB endpoint URL
- `COSMOS_KEY` - Cosmos DB access key
- `COSMOS_DATABASE` - Database name
- `COSMOS_CONTAINER` - Container name
- `RUST_LOG` - Log level (info, debug, etc.)

## Troubleshooting

- Check logs for detailed error information
- Verify Cosmos DB connectivity
- Ensure proper authentication configuration