# ROT Frontend

A web application built with Rust (Yew) and WebAssembly for monitoring and managing IoT devices.

## Overview

This frontend application provides a user interface for the ROT (Rust of Things) system. It displays telemetry data from connected devices in charts and tables, and allows for device configuration management.

## Features

- Real-time telemetry visualization with charts
- Device configuration management
- Responsive design using Tailwind CSS
- Written in Rust and compiled to WebAssembly
- SPA (Single Page Application) architecture

## Project Structure

- `src/`
  - `components/` - Reusable UI components
  - `domain/` - Domain models and types
  - `services/` - API client code
  - `views/` - Page components
- `static/` - Static assets
- `index.html` - HTML template
- `Dockerfile` - Container definition
- `nginx.conf` - Nginx configuration for hosting
- `tailwind.config.js` - Tailwind CSS configuration

## Views

### Telemetry View

Displays telemetry data from devices in charts and tables. Features include:
- Temperature and voltage charts
- Historical data exploration
- Data filtering by time range

### Configuration View

Allows managing device configurations:
- Toggle LED state
- Adjust reporting interval
- View configuration history

## Development

### Prerequisites

- Rust toolchain
- wasm-pack
- Node.js and npm (for Tailwind CSS)

### Setup

```bash
# Install dependencies
npm install

# Build Tailwind CSS
npm run build:css
```

### Running locally

```bash
# Development build with hot reloading
trunk serve

# Or use Docker
./scripts/local-docker.sh
```

### Building for production

```bash
# Build optimized WASM bundle
trunk build --release
```

## Deployment

The frontend is deployed as a static site in a container with Nginx. See the pipeline definition in `/Pipelines/build-rot-fe.yml`.

## Architecture

This application is built with:

- **Yew** - A Rust framework for creating web applications with WebAssembly
- **Tailwind CSS** - A utility-first CSS framework
- **Trunk** - A WASM web application bundler for Rust

The frontend communicates with the Device Monitor and Device Configuration services to retrieve and display data.

## Customization

### Adding new visualizations

1. Create a new component in `src/components/`
2. Update the relevant view in `src/views/`
3. Add any required API client functions in `src/services/`

### Styling

The application uses Tailwind CSS for styling. To modify the theme:

1. Edit `tailwind.config.js`
2. Run `npm run build:css` to regenerate CSS