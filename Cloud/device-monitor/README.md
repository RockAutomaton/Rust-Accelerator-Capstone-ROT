# Device Monitor Service

This service provides a REST API for retrieving and monitoring telemetry data from IoT devices stored in Azure Cosmos DB.

## Overview

The Device Monitor Service allows retrieving historical telemetry data from the database for monitoring and analysis purposes. It's designed to work with the device-comms service that ingests telemetry data, providing a read-only interface for data retrieval.

## Features

- RESTful API for querying telemetry data by device ID
- Azure authentication and authorization using service principal
- Cosmos DB integration for data retrieval
- Structured logging and request tracing
- CORS support for web client integration
- Docker containerization with multi-stage builds
- Comprehensive integration tests

## Project Structure

- `src/`
  - `app_state/` - Application state and dependency injection
  - `domain/` - Domain models and telemetry data structures
  - `routes/` - API endpoint definitions and handlers
  - `services/` - External service integrations (Cosmos DB, Azure Auth)
  - `utils/` - Utility functions and tracing/logging helpers
- `scripts/` - Development and deployment scripts
- `tests/` - Integration tests for API endpoints

## API Endpoints

### GET /iot/data/read/{device_id}

Retrieves all telemetry data for a specific device from the database.

**Path Parameters:**
- `device_id` - The unique identifier of the device to monitor

**Response:**
```json
[
  {
    "id": "device-123-1640995200",
    "device_id": "device-123",
    "telemetry_data": {
      "temperature": "23.5",
      "humidity": "45.2",
      "status": "online"
    },
    "timestamp": 1640995200
  },
  {
    "id": "device-123-1640995260",
    "device_id": "device-123",
    "telemetry_data": {
      "temperature": "24.1",
      "humidity": "44.8",
      "status": "online"
    },
    "timestamp": 1640995260
  }
]
```

**Error Responses:**
- `404 Not Found` - Device not found or no telemetry data available
- `400 Bad Request` - Invalid device ID format
- `500 Internal Server Error` - Database connection or query error

## Local Development

### Prerequisites

- Rust toolchain (1.87 or later)
- Docker and Docker Compose
- Azure Cosmos DB instance
- Azure service principal with Cosmos DB access

### Environment Variables

Create a `.env` file in the project root with the following variables:

```env
# Azure Cosmos DB Configuration
COSMOS_ENDPOINT=https://your-cosmos-account.documents.azure.com:443/

# Azure Authentication (Service Principal)
AZURE_CLIENT_ID=your-service-principal-client-id
AZURE_CLIENT_SECRET=your-service-principal-client-secret
AZURE_TENANT_ID=your-azure-tenant-id

# Rocket Configuration
SECRET_KEY=your-64-character-secret-key-for-rocket-sessions
```

### Running Locally

```bash
# Run with cargo
cargo run

# Or use the Docker script for containerized development
./scripts/local-docker.sh
```

The service will be available at `http://localhost:8001`

### Testing

```bash
# Run all tests
cargo test

# Run only integration tests
cargo test --test api
```

## Deployment

The service is designed to be deployed as a container to Azure Container Apps. See the pipeline definition in `/Pipelines/build-device-monitor.yml`.

### Docker Build

```bash
# Build the Docker image
docker build -t device-monitor .

# Run the container
docker run -p 8001:8001 --env-file .env device-monitor
```

## Configuration

The service is configured through environment variables:

### Required Environment Variables

- `COSMOS_ENDPOINT` - Azure Cosmos DB endpoint URL
- `AZURE_CLIENT_ID` - Azure AD service principal client ID
- `AZURE_CLIENT_SECRET` - Azure AD service principal client secret
- `AZURE_TENANT_ID` - Azure AD tenant ID
- `SECRET_KEY` - Rocket secret key for session management (64 hex characters)

### Optional Environment Variables

- `RUST_LOG` - Log level (default: "info", options: "debug", "warn", "error")

## Usage Examples

### Querying telemetry data for a device

```bash
# Get all telemetry for a specific device
curl "http://localhost:8001/iot/data/read/device-123"

# Get telemetry for a device with special characters in ID
curl "http://localhost:8001/iot/data/read/sensor-001"
```

### Example Response

```json
[
  {
    "id": "sensor-001-1640995200",
    "device_id": "sensor-001",
    "telemetry_data": {
      "temperature": "23.5",
      "humidity": "45.2",
      "battery_level": "85"
    },
    "timestamp": 1640995200
  }
]
```

## Architecture

The service follows a clean architecture pattern:

1. **Routes Layer** - HTTP request handling and validation
2. **Domain Layer** - Business logic and data models
3. **Services Layer** - External service integrations
4. **App State** - Dependency injection and shared resources

### Data Flow

1. HTTP request arrives at `/iot/data/read/{device_id}`
2. Route handler validates the device ID
3. Cosmos DB service queries for telemetry data
4. Results are serialized and returned as JSON
5. Request/response are logged with tracing information

## Monitoring and Observability

The service includes comprehensive logging and tracing:

- Structured logging with configurable levels
- Request/response correlation with unique IDs
- Performance metrics (latency tracking)
- Error tracking and context preservation
- Azure integration for centralized logging

## Security

- Azure service principal authentication for Cosmos DB access
- CORS configuration for controlled cross-origin access
- Input validation and sanitization
- Secure environment variable handling