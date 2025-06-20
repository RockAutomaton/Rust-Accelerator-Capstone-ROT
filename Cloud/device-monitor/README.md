# Device Monitor Service

This service provides an API to query telemetry data from IoT devices.

## Overview

The Device Monitor Service allows retrieving historical telemetry data from the database. It's primarily used by the frontend to display device status, metrics, and charts.

## Features

- RESTful API for querying telemetry data
- Filtering by device ID, time range, and metrics
- Azure authentication and authorization
- Cosmos DB integration for data retrieval
- Structured logging
- Docker containerization

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

### GET /api/telemetry

Retrieves telemetry data with optional filtering.

Query parameters:
- `device_id` - Filter by specific device
- `start_time` - Start of time range (ISO 8601 format)
- `end_time` - End of time range (ISO 8601 format)
- `limit` - Maximum number of records to return

Response:
```json
[
  {
    "device_id": "device-123",
    "timestamp": "2023-06-20T12:34:56Z",
    "temperature": 25.5,
    "voltage": 3.3
  },
  {
    "device_id": "device-123",
    "timestamp": "2023-06-20T12:35:56Z",
    "temperature": 25.6,
    "voltage": 3.29
  }
]
```

### GET /api/telemetry/latest/{device_id}

Retrieves the latest telemetry entry for a specific device.

Response:
```json
{
  "device_id": "device-123",
  "timestamp": "2023-06-20T12:35:56Z",
  "temperature": 25.6,
  "voltage": 3.29
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

The service is designed to be deployed as a container to Azure Container Apps. See the pipeline definition in `/Pipelines/build-device-monitor.yml`.

## Configuration

The service is configured through environment variables:

- `COSMOS_ENDPOINT` - Cosmos DB endpoint URL
- `COSMOS_KEY` - Cosmos DB access key
- `COSMOS_DATABASE` - Database name
- `COSMOS_CONTAINER` - Container name
- `RUST_LOG` - Log level (info, debug, etc.)

## Usage Example

### Querying telemetry data

```bash
curl "http://localhost:3002/api/telemetry?device_id=device-123&limit=10"
```

### Getting latest telemetry

```bash
curl "http://localhost:3002/api/telemetry/latest/device-123"
```