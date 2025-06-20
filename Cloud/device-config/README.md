# Device Configuration Service

This service manages device configurations and allows updating settings for remote devices.

## Overview

The Device Configuration Service provides an API for managing device configurations. It allows both retrieving current device settings and updating them remotely. Embedded devices periodically check this service to get their latest configuration.

## Features

- RESTful API for device configuration management
- Get configuration by device ID
- Update device configurations
- Azure authentication and authorization
- Cosmos DB integration for configuration storage
- Structured logging
- Docker containerization

## Project Structure

- `src/`
  - `app_state/` - Application state and dependency injection
  - `domain/` - Domain models and types (configuration schema)
  - `routes/` - API endpoint definitions
  - `services/` - Business logic and external service integrations
  - `utils/` - Utility functions and helpers
- `scripts/` - Development and deployment scripts

## API Endpoints

### GET /api/config/{device_id}

Retrieves the current configuration for a specific device.

Response:
```json
{
  "device_id": "device-123",
  "led_enabled": true,
  "reporting_interval_seconds": 60,
  "last_updated": "2023-06-20T12:34:56Z"
}
```

### PUT /api/config/{device_id}

Updates the configuration for a specific device.

Request:
```json
{
  "led_enabled": false,
  "reporting_interval_seconds": 120
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

## Deployment

The service is designed to be deployed as a container to Azure Container Apps. See the pipeline definition in `/Pipelines/build-device-config.yml`.

## Configuration

The service is configured through environment variables:

- `COSMOS_ENDPOINT` - Cosmos DB endpoint URL
- `COSMOS_KEY` - Cosmos DB access key
- `COSMOS_DATABASE` - Database name
- `COSMOS_CONTAINER` - Container name
- `RUST_LOG` - Log level (info, debug, etc.)

## Usage Example

### Updating device configuration

```bash
curl -X PUT http://localhost:3001/api/config/device-123 \
  -H "Content-Type: application/json" \
  -d '{"led_enabled": true, "reporting_interval_seconds": 30}'
```

### Retrieving device configuration

```bash
curl http://localhost:3001/api/config/device-123
```