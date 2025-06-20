# Debug Server

A simple debugging utility for the ROT embedded firmware.

## Overview

This debug server provides a way to test and debug the embedded firmware without requiring a physical device. It simulates device behavior and API responses for development purposes.

## Features

- Simulates device telemetry data
- Responds to configuration requests
- Logs all API interactions
- Configurable response behavior
- Low overhead for development environments

## Usage

### Running the server

```bash
cargo run
```

By default, the server listens on `localhost:8080`.

### Endpoints

The debug server provides the following endpoints:

#### POST /api/telemetry

Accepts telemetry data from simulated devices. Logs the received data and returns a success response.

#### GET /api/config/{device_id}

Returns a simulated configuration for the specified device ID.

## Configuration

The debug server can be configured by modifying constants in the source code:

- `DEFAULT_PORT` - The port the server listens on
- `DEFAULT_HOST` - The host address to bind to
- `SIMULATED_DELAY_MS` - Optional delay to simulate network latency

## Development

This server is intended for development and testing purposes only. It does not provide authentication or data persistence, and should not be used in production environments.

### Adding new simulated behavior

To add new simulated behavior:

1. Add a new route handler in `src/main.rs`
2. Implement the desired response logic
3. Update the router to include the new endpoint

## Integration with embedded development

When developing the embedded firmware:

1. Run this debug server
2. Configure the embedded firmware to point to this server instead of the production endpoints
3. Monitor the debug server's output to verify correct behavior