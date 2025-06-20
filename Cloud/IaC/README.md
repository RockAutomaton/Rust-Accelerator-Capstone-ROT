# Infrastructure as Code (IaC)

Azure infrastructure definition using Bicep templates for the ROT system.

## Overview

This directory contains the infrastructure definition for deploying the ROT system to Azure. The infrastructure is defined using Bicep templates, which are compiled to ARM templates during deployment.

## Components

The infrastructure includes the following Azure resources:

- **Azure Container Apps** - Hosting for the microservices
- **Azure Container Registry** - Storage for container images
- **Azure Cosmos DB** - Database for telemetry and configuration data
- **Azure Log Analytics** - Monitoring and logging solution

## Structure

- `main.bicep` - Main entry point for the infrastructure deployment
- `modules/` - Reusable Bicep modules
  - `ContainerApp.bicep` - Definition for Container Apps
  - `ContainerAppEnvironment.bicep` - Environment configuration
  - `ContainerRegistry.bicep` - Container registry definition
  - `CosmosDB.bicep` - Cosmos DB account, database, and containers
  - `LogAnalytics.bicep` - Log analytics workspace

## Deployment

### Prerequisites

- Azure CLI
- Azure subscription
- Bicep CLI or Azure CLI with Bicep support

### Deploying manually

```bash
# Login to Azure
az login

# Set your subscription
az account set --subscription <your-subscription-id>

# Deploy the infrastructure
az deployment group create \
  --resource-group <resource-group-name> \
  --template-file main.bicep \
  --parameters appName=rot location=eastus
```

### Using CI/CD pipeline

The infrastructure is typically deployed using the CI/CD pipeline defined in `/Pipelines/deploy-infra.yml`.

## Parameters

The `main.bicep` file accepts the following parameters:

- `appName` - Base name for all resources (default: "rot")
- `location` - Azure region for resource deployment (default: resource group location)
- `containerRegistrySku` - SKU for Container Registry (default: "Basic")
- `cosmosThroughput` - RU/s for Cosmos DB containers (default: 400)

## Customization

### Adding new resources

1. Create a new Bicep module in the `modules/` directory
2. Add references to the module in `main.bicep`
3. Update parameter references as needed

### Modifying existing resources

1. Locate the relevant module in the `modules/` directory
2. Update the resource definition
3. Test the changes in a development environment before deploying to production

## Security

The infrastructure includes the following security measures:

- RBAC (Role-Based Access Control) for resource access
- Managed identities for service authentication
- Network security rules
- Resource locks to prevent accidental deletion

## Monitoring

The deployed infrastructure includes:

- Log Analytics workspace for centralized logging
- Container App insights for application monitoring
- Cosmos DB metrics and logs