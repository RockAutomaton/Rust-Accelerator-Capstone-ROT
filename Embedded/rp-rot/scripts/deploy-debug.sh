#!/bin/bash

# Exit on any error
set -e

# Function for logging
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1"
}

# Check if required tools are installed
if ! command -v cargo &> /dev/null; then
    log "ERROR: cargo is not installed"
    exit 1
fi

if ! command -v probe-rs &> /dev/null; then
    log "Installing probe-rs..."
    cargo install probe-rs
fi

log "Starting debug deployment..."

# Build the firmware in debug mode
log "Building firmware in debug mode..."
if ! cargo build; then
    log "ERROR: Failed to build firmware"
    exit 1
fi
log "Build successful"

# List available probes
log "Checking for available debug probes..."
if ! probe-rs list; then
    log "ERROR: No debug probes found. Please ensure your device is connected and in bootloader mode."
    exit 1
fi

# Deploy and start debug session
log "Deploying firmware and starting debug session..."
if ! probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/rp-rot; then
    log "ERROR: Failed to start debug session"
    exit 1
fi
log "Debug session started successfully" 