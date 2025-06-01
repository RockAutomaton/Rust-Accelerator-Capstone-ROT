@description('The name of the Log Analytics Workspace')
param workspaceName string

@description('The location for the Log Analytics Workspace')
param location string = 'uksouth'

@description('The SKU name for the Log Analytics Workspace')
param skuName string = 'PerGB2018'

@description('The retention period in days for the Log Analytics Workspace')
param retentionInDays int = 30

@description('Whether to enable public network access for ingestion')
param enablePublicNetworkAccessForIngestion bool = true

@description('Whether to enable public network access for query')
param enablePublicNetworkAccessForQuery bool = true

@description('The daily quota in GB for the Log Analytics Workspace. Use -1 for unlimited.')
param dailyQuotaGb int = -1

resource logAnalyticsWorkspace 'Microsoft.OperationalInsights/workspaces@2023-09-01' = {
  name: workspaceName
  location: location
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    sku: {
      name: skuName
    }
    retentionInDays: retentionInDays
    features: {
      legacy: 0
      searchVersion: 1
      enableLogAccessUsingOnlyResourcePermissions: true
    }
    workspaceCapping: {
      dailyQuotaGb: dailyQuotaGb
    }
    publicNetworkAccessForIngestion: enablePublicNetworkAccessForIngestion ? 'Enabled' : 'Disabled'
    publicNetworkAccessForQuery: enablePublicNetworkAccessForQuery ? 'Enabled' : 'Disabled'
  }
}

// Outputs
output workspaceId string = logAnalyticsWorkspace.id
output workspaceName string = logAnalyticsWorkspace.name
output workspaceCustomerId string = logAnalyticsWorkspace.properties.customerId
output workspaceSharedKey string = logAnalyticsWorkspace.listKeys().primarySharedKey
