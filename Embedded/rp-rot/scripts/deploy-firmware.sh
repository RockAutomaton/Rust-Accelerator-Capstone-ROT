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

if ! command -v elf2uf2-rs &> /dev/null; then
    log "ERROR: elf2uf2-rs is not installed"
    exit 1
fi

log "Starting firmware deployment..."

# Build the firmware
log "Building firmware..."
if ! cargo build --release; then
    log "ERROR: Failed to build firmware"
    exit 1
fi
log "Build successful"

# Copy the firmware to the RP2040
log "Deploying firmware to RP2040..."
if ! elf2uf2-rs -d target/thumbv6m-none-eabi/release/rp-rot; then
    log "ERROR: Failed to deploy firmware"
    exit 1
fi
log "Firmware deployment successful"

