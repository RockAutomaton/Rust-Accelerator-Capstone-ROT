# CI/CD Pipelines

This directory contains CI/CD pipeline definitions for the ROT system.

## Overview

These pipeline definitions automate the building, testing, and deployment of the ROT system components. They are designed to work with Azure DevOps pipelines or similar CI/CD systems.

## Pipeline Definitions

### Microservice Build Pipelines

- `build-device-comms.yml` - Builds and publishes the Device Communications Service
- `build-device-config.yml` - Builds and publishes the Device Configuration Service
- `build-device-monitor.yml` - Builds and publishes the Device Monitor Service
- `build-rot-fe.yml` - Builds and publishes the ROT Frontend

### Infrastructure Deployment

- `deploy-infra.yml` - Deploys the Azure infrastructure using Bicep templates

## Pipeline Structure

Each service build pipeline typically includes the following stages:

1. **Build** - Compiles the application
2. **Test** - Runs unit and integration tests
3. **Containerize** - Creates a Docker container
4. **Publish** - Pushes the container to Azure Container Registry

The infrastructure deployment pipeline:

1. Validates the Bicep templates
2. Compiles Bicep to ARM templates
3. Deploys the infrastructure to Azure

## Usage

### Triggering Pipelines

Pipelines are typically triggered by:

- Push to main branch
- Pull request to main branch
- Manual trigger

### Pipeline Variables

The pipelines expect the following variables to be defined:

- `azureSubscription` - Azure subscription identifier
- `containerRegistry` - Azure Container Registry name
- `resourceGroup` - Target resource group for deployment

## Customization

### Adding a New Service Pipeline

1. Create a new YAML file based on an existing service pipeline
2. Update the service name, build steps, and test commands
3. Configure the triggers and conditions

### Modifying Deployment Parameters

To change deployment parameters:

1. Edit the `deploy-infra.yml` file
2. Update the parameters passed to the Bicep deployment

## Security

The pipelines use service connections with appropriate permissions for:

- Container Registry push access
- Infrastructure deployment
- Secrets management

## Best Practices

- Keep build steps consistent across services
- Use specific versions for build tools
- Cache dependencies to speed up builds
- Run tests before deployment
- Version all artifacts