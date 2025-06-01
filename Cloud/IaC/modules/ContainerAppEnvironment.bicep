@description('The name of the Container App Environment')
param environmentName string

@description('The location for the Container App Environment')
param location string = 'UK South'

@description('The Log Analytics Workspace customer ID for app logs')
param logAnalyticsCustomerId string

@description('Whether to enable zone redundancy')
param zoneRedundant bool = false

@description('Whether to enable mTLS for peer authentication')
param enableMtls bool = false

@description('Whether to enable encryption for peer traffic')
param enablePeerTrafficEncryption bool = false

@description('The workload profile type')
param workloadProfileType string = 'Consumption'

resource containerAppEnvironment 'Microsoft.App/managedEnvironments@2025-01-01' = {
  name: environmentName
  location: location
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    appLogsConfiguration: {
      destination: 'log-analytics'
      logAnalyticsConfiguration: {
        customerId: logAnalyticsCustomerId
      }
    }
    zoneRedundant: zoneRedundant
    kedaConfiguration: {}
    daprConfiguration: {}
    customDomainConfiguration: {}
    workloadProfiles: [
      {
        workloadProfileType: workloadProfileType
        name: workloadProfileType
      }
    ]
    peerAuthentication: {
      mtls: {
        enabled: enableMtls
      }
    }
    peerTrafficConfiguration: {
      encryption: {
        enabled: enablePeerTrafficEncryption
      }
    }
  }
}

// Outputs
output environmentId string = containerAppEnvironment.id
output environmentName string = containerAppEnvironment.name
output environmentDefaultDomain string = containerAppEnvironment.properties.defaultDomain
output environmentStaticIp string = containerAppEnvironment.properties.staticIp
output environmentSystemAssignedIdentityPrincipalId string = containerAppEnvironment.identity.principalId
