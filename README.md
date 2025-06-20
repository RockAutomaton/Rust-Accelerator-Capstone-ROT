# ROT (Rust of Things)

A full-stack IoT project built with Rust that enables remote monitoring and configuration of embedded devices.

## System Architecture

### 1. Embedded Device (Raspberry Pi Pico with RP2040)

- Located in `/Embedded/rp-rot/`
- Uses Embassy framework for async Rust on embedded devices
- Collects telemetry data (temperature, voltage) from sensors
- Connects to WiFi and sends data to cloud backend
- Receives configuration updates from the cloud
- Key components:
  - Temperature and voltage sensors
  - LED indicators for status
  - WiFi communication via cyw43 chipset

### 2. Cloud Backend

Consists of three microservices:

#### a. Device Communications Service (`/Cloud/device-comms/`)

- Receives telemetry data from embedded devices
- Stores data in Azure Cosmos DB


#### b. Device Configuration Service (`/Cloud/device-config/`)

- Manages device configurations
- Allows updating settings (like LED state)
- Devices pull their configurations from this service

#### c. Device Monitor Service (`/Cloud/device-monitor/`)

- Provides an API to query telemetry data
- Used by the frontend to display device status and metrics

### 3. Frontend (`/Cloud/rot-fe/`)

- Web application built with Yew (Rust WASM framework) and Tailwind CSS
- Displays telemetry data with charts
- Allows configuration management
- Responsive UI with navigation between views

### 4. Infrastructure

- Azure-based cloud infrastructure defined in Bicep templates (`/Cloud/IaC/`)
- Uses:
  - Azure Container Apps for hosting microservices
  - Azure Cosmos DB for data storage
  - Azure Container Registry for container images
  - Azure Log Analytics for monitoring

## Getting Started

### Setting Up the Embedded Device

1. See `/Docs/Embedded/Setup.md` for detailed instructions
2. Use the scripts in `/Embedded/rp-rot/scripts/` for deployment

### Running the Cloud Services Locally

Each service can be run locally using Docker:

```bash
# Device Communications Service
cd Cloud/device-comms
./scripts/local-docker.sh

# Device Configuration Service
cd Cloud/device-config
./scripts/local-docker.sh

# Device Monitor Service
cd Cloud/device-monitor
./scripts/local-docker.sh

# Frontend
cd Cloud/rot-fe
./scripts/local-docker.sh
```

### Deploying to Azure

Use the Azure deployment pipeline in `/Pipelines/deploy-infra.yml`

## Technologies Used

- **Rust** - Used across the entire stack
- **Embassy** - Async runtime for embedded Rust
- **Yew** - Rust/WASM framework for frontend
- **Tailwind CSS** - Styling for the frontend
- **Azure** - Cloud infrastructure
- **Cosmos DB** - Data storage
- **Azure Container Apps** - Service hosting
