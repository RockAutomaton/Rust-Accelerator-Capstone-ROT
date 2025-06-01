
param namePrefix string = 'rot-poc2'
param location string = resourceGroup().location
param contributorPrincipalIds string[]
@secure()
param secretKey string
@secure()
param azureClientSecret string
@secure()
param azureTenantId string
@secure()
param azureClientId string

module logAnalytics 'modules/LogAnalytics.bicep' = {
  name: 'deployLogAnalytics'
  params: {
    workspaceName: '${namePrefix}-la'
    location: 'uksouth'
    skuName: 'PerGB2018'
    retentionInDays: 30
    enablePublicNetworkAccessForIngestion: true
    enablePublicNetworkAccessForQuery: true
    dailyQuotaGb: -1
  }
}

module cosmosDb 'modules/CosmosDB.bicep' = {
  name: 'deployCosmosDb'
  params: {
    cosmosDbAccountName: '${namePrefix}-cosmos'
    location: location
    databaseName: 'device-data'
    telemetryContainerName: 'telemetry'
    contributorPrincipalIds: contributorPrincipalIds
  }
}

module containerAppEnvironment 'modules/ContainerAppEnvironment.bicep' = {
  name: 'deployContainerAppEnvironment'
  params: {
    environmentName: '${namePrefix}-app-env'
    location: location
    logAnalyticsCustomerId: logAnalytics.outputs.workspaceId
    zoneRedundant: true
    enableMtls: true
    enablePeerTrafficEncryption: true
    workloadProfileType: 'Consumption'
  }
}

module containerApp 'modules/ContainerApp.bicep' = {
  name: 'deployContainerApp'
  params: {
    appName: '${namePrefix}-app'
    environmentId: containerAppEnvironment.outputs.environmentId
    registryServer: 'rotpoc2.azurecr.io'
    containerImage: 'rotpoc2.azurecr.io/rot-poc-devicecomms:latest'
    secretKey: secretKey
    azureClientId: azureClientId
    azureClientSecret: azureClientSecret
    azureTenantId: azureTenantId
    cosmosEndpoint: cosmosDb.outputs.connectionString
    allowedIpRanges: [
      {
        name: 'IP-1'
        ipAddressRange: '1.2.3.4'
        action: 'Allow'
      }
    ]
  }
}
