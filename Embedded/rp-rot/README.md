# Embedded Device (rp-rot)

This directory contains the embedded firmware for the Raspberry Pi Pico with RP2040 microcontroller.

## Overview

The embedded device collects telemetry data from sensors and transmits it to the cloud backend. It also receives configuration updates from the cloud.

### Features

- Temperature sensing using the RP2040's internal temperature sensor
- Voltage monitoring
- WiFi connectivity through the CYW43 chipset
- LED status indicators
- Async Rust using the Embassy framework
- HTTP communication with cloud services

## Project Structure

- `src/`
  - `config/` - Device and WiFi configuration management
  - `drivers/` - Hardware abstraction for sensors and peripherals
  - `error/` - Error handling types
  - `network/` - Network connectivity and HTTP clients
  - `tasks/` - Async tasks for different device functions
  - `utils/` - Utility functions and helpers
- `scripts/` - Deployment and build scripts
- `cyw43-firmware/` - WiFi chipset firmware

## Building and Flashing

### Prerequisites

- Rust with `thumbv6m-none-eabi` target installed
- probe-rs for flashing
- A Raspberry Pi Pico board

### Building

```bash
cargo build --release
```

### Flashing

Use the provided scripts:

```bash
# Deploy firmware in debug mode
./scripts/deploy-debug.sh

# Deploy firmware in release mode
./scripts/deploy-firmware.sh
```

## Development

The firmware is built with Embassy, an async runtime for embedded Rust. The main program starts several tasks:

1. **Blinker task** - Controls LED status indicators
2. **Network task** - Manages WiFi connectivity
3. **Telemetry task** - Collects and sends sensor data
4. **Config fetch task** - Retrieves configuration updates

To modify sensor reading behavior, update the relevant code in `src/drivers/`.

## Troubleshooting

- Check LED status indicators for basic diagnostics
- For more detailed debugging, connect to the debug server
- View logs in the console when connected via USB