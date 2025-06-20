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
- Structured logging and request tracing
- Docker containerization
- CORS support for cross-origin requests

## Project Structure

- `src/`
  - `app_state/` - Application state and dependency injection
  - `domain/` - Domain models and types (configuration schema)
  - `routes/` - API endpoint definitions
  - `services/` - Business logic and external service integrations
  - `utils/` - Utility functions and helpers
- `scripts/` - Development and deployment scripts

## API Endpoints

### GET /device-config/get/{device_id}

Retrieves the current configuration for a specific device.

Response:
```json
[
  {
    "device_id": "device-123",
    "config": {
      "sampling_rate": "1000",
      "threshold": "25.5",
      "wifi_ssid": "MyNetwork"
    }
  }
]
```

### POST /device-config/update

Updates the configuration for a specific device.

Request:
```json
{
  "device_id": "device-123",
  "config": {
    "sampling_rate": "2000",
    "threshold": "30.0",
    "wifi_ssid": "NewNetwork"
  }
}
```

Response:
```
Config ingested
```

## Local Development

### Prerequisites

- Rust toolchain
- Docker and Docker Compose
- Azure Cosmos DB connection string
- Azure authentication credentials

### Running locally

```bash
# Run with environment variables
COSMOS_ENDPOINT=<your-cosmos-endpoint> \
AZURE_CLIENT_ID=<your-client-id> \
AZURE_CLIENT_SECRET=<your-client-secret> \
AZURE_TENANT_ID=<your-tenant-id> \
SECRET_KEY=<your-secret-key> \
cargo run

# Or use the Docker script
./scripts/local-docker.sh
```

## Deployment

The service is designed to be deployed as a container to Azure Container Apps. See the pipeline definition in `/Pipelines/build-device-config.yml`.

## Configuration

The service is configured through environment variables:

- `COSMOS_ENDPOINT` - Cosmos DB endpoint URL
- `AZURE_CLIENT_ID` - Azure AD application client ID
- `AZURE_CLIENT_SECRET` - Azure AD application client secret
- `AZURE_TENANT_ID` - Azure AD tenant ID
- `SECRET_KEY` - Rocket secret key for session management
- `RUST_LOG` - Log level (info, debug, etc.)

## Usage Example

### Updating device configuration

```bash
curl -X POST http://localhost:8002/device-config/update \
  -H "Content-Type: application/json" \
  -d '{
    "device_id": "device-123",
    "config": {
      "sampling_rate": "1000",
      "threshold": "25.5"
    }
  }'
```

### Retrieving device configuration

```bash
curl http://localhost:8002/device-config/get/device-123
```

## Port Configuration

- **Development**: Port 8002
- **Docker**: Port 8002 (mapped to host)
- **Container Apps**: Port 8002

## Database Schema

The service uses Cosmos DB with the following structure:
- **Database**: `device-config`
- **Container**: `config`
- **Partition Key**: `device_id`
- **Document Structure**: Device configuration with key-value pairs