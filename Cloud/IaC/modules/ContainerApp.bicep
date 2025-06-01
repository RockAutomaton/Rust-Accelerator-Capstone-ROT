@description('The name of the Container App')
param appName string

@description('The location for the Container App')
param location string = 'UK South'

@description('The ID of the Container App Environment')
param environmentId string

@description('The workload profile name')
param workloadProfileName string = 'Consumption'

@description('The container registry server')
param registryServer string

@description('The container image name and tag')
param containerImage string

@description('The target port for the container')
param targetPort int = 8000

@description('Whether to enable external ingress')
param enableExternalIngress bool = true

@description('Whether to allow insecure traffic')
param allowInsecure bool = true

@description('The minimum number of replicas')
param minReplicas int = 0

@description('The maximum number of replicas')
param maxReplicas int = 10

@description('The cooldown period in seconds')
param cooldownPeriod int = 300

@description('The polling interval in seconds')
param pollingInterval int = 30

@description('The CPU allocation for the container')
param cpuAllocation string = '0.5'

@description('The memory allocation for the container')
param memoryAllocation string = '1Gi'

@description('The allowed IP ranges for ingress')
param allowedIpRanges array = []

@description('The allowed origins for CORS')
param allowedOrigins array = ['*']

@description('The allowed headers for CORS')
param allowedHeaders array = ['*']

@description('The max age for CORS')
param corsMaxAge int = 0

@description('Whether to allow credentials for CORS')
param allowCredentials bool = false

@secure()
@description('The secret key for the application')
param secretKey string = ''

@secure()
@description('The Azure client ID')
param azureClientId string = ''

@secure()
@description('The Azure client secret')
param azureClientSecret string = ''

@secure()
@description('The Azure tenant ID')
param azureTenantId string = ''

@description('The Cosmos DB endpoint')
param cosmosEndpoint string = ''



resource containerApp 'Microsoft.App/containerapps@2025-01-01' = {
  name: appName
  location: location
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    managedEnvironmentId: environmentId
    environmentId: environmentId
    workloadProfileName: workloadProfileName
    configuration: {
      activeRevisionsMode: 'Single'
      ingress: enableExternalIngress ? {
        external: true
        targetPort: targetPort
        exposedPort: 0
        transport: 'Auto'
        traffic: [
          {
            weight: 100
            latestRevision: true
          }
        ]
        allowInsecure: allowInsecure
        ipSecurityRestrictions: allowedIpRanges
        corsPolicy: {
          allowedOrigins: allowedOrigins
          allowedHeaders: allowedHeaders
          maxAge: corsMaxAge
          allowCredentials: allowCredentials
        }
        clientCertificateMode: 'Ignore'
        stickySessions: {
          affinity: 'none'
        }
      } : null
      registries: [
        {
          server: registryServer
          identity: 'system-environment'
        }
      ]
      maxInactiveRevisions: 100
      identitySettings: []
    }
    template: {
      containers: [
        {
          image: containerImage
          name: appName
          env: concat(
            secretKey != '' ? [
              {
                name: 'SECRET_KEY'
                value: secretKey
              }
            ] : [],
            azureClientId != '' ? [
              {
                name: 'AZURE_CLIENT_ID'
                value: azureClientId
              }
            ] : [],
            azureClientSecret != '' ? [
              {
                name: 'AZURE_CLIENT_SECRET'
                value: azureClientSecret
              }
            ] : [],
            azureTenantId != '' ? [
              {
                name: 'AZURE_TENANT_ID'
                value: azureTenantId
              }
            ] : [],
            cosmosEndpoint != '' ? [
              {
                name: 'COSMOS_ENDPOINT'
                value: cosmosEndpoint
              }
            ] : []
          )
          resources: {
            cpu: json(cpuAllocation)
            memory: memoryAllocation
          }
          probes: []
        }
      ]
      scale: {
        minReplicas: minReplicas
        maxReplicas: maxReplicas
        cooldownPeriod: cooldownPeriod
        pollingInterval: pollingInterval
      }
      volumes: []
    }
  }
}

// Outputs
output appId string = containerApp.id
output appName string = containerApp.name
output appUrl string = containerApp.properties.configuration.ingress.fqdn
output appSystemAssignedIdentityPrincipalId string = containerApp.identity.principalId
