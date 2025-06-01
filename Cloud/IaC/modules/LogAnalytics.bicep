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

resource logAnalyticsWorkspace 'Microsoft.OperationalInsights/workspaces@2025-02-01' = {
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


resource ContainerAppConsoleLogs 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerAppConsoleLogs'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerAppConsoleLogs'
      displayName: 'ContainerAppConsoleLogs'
    }
    retentionInDays: 30
  }
}

resource ContainerAppConsoleLogs_CL 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerAppConsoleLogs_CL'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerAppConsoleLogs_CL'
      displayName: 'ContainerAppConsoleLogs_CL'
      columns: [
        {
          name: '_timestamp_d'
          type: 'real'
          displayName: '_timestamp_d'
        }
        {
          name: 'time_t'
          type: 'datetime'
          displayName: 'time_t'
        }
        {
          name: 'ContainerAppName_s'
          type: 'string'
          displayName: 'ContainerAppName_s'
        }
        {
          name: 'Log_s'
          type: 'string'
          displayName: 'Log_s'
        }
        {
          name: 'Stream_s'
          type: 'string'
          displayName: 'Stream_s'
        }
        {
          name: 'ContainerGroupId_g'
          type: 'guid'
          displayName: 'ContainerGroupId_g'
        }
        {
          name: 'ContainerImage_s'
          type: 'string'
          displayName: 'ContainerImage_s'
        }
        {
          name: 'Category'
          type: 'string'
          displayName: 'Category'
        }
        {
          name: 'EnvironmentName_s'
          type: 'string'
          displayName: 'EnvironmentName_s'
        }
        {
          name: 'ContainerGroupName_s'
          type: 'string'
          displayName: 'ContainerGroupName_s'
        }
        {
          name: 'ContainerName_s'
          type: 'string'
          displayName: 'ContainerName_s'
        }
        {
          name: 'ContainerId_g'
          type: 'guid'
          displayName: 'ContainerId_g'
        }
        {
          name: 'RevisionName_s'
          type: 'string'
          displayName: 'RevisionName_s'
        }
      ]
    }
    retentionInDays: 30
  }
}

resource ContainerAppSystemLogs 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerAppSystemLogs'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerAppSystemLogs'
      displayName: 'ContainerAppSystemLogs'
    }
    retentionInDays: 30
  }
}

resource ContainerAppSystemLogs_CL 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerAppSystemLogs_CL'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerAppSystemLogs_CL'
      displayName: 'ContainerAppSystemLogs_CL'
      columns: [
        {
          name: 'time_t'
          type: 'datetime'
          displayName: 'time_t'
        }
        {
          name: '_timestamp_d'
          type: 'real'
          displayName: '_timestamp_d'
        }
        {
          name: 'EnvironmentName_s'
          type: 'string'
          displayName: 'EnvironmentName_s'
        }
        {
          name: 'time_s'
          type: 'string'
          displayName: 'time_s'
        }
        {
          name: 'ContainerAppName_s'
          type: 'string'
          displayName: 'ContainerAppName_s'
        }
        {
          name: 'EventSource_s'
          type: 'string'
          displayName: 'EventSource_s'
        }
        {
          name: 'Reason_s'
          type: 'string'
          displayName: 'Reason_s'
        }
        {
          name: 'TimeStamp_s'
          type: 'string'
          displayName: 'TimeStamp_s'
        }
        {
          name: 'JobName_s'
          type: 'string'
          displayName: 'JobName_s'
        }
        {
          name: 'Type_s'
          type: 'string'
          displayName: 'Type_s'
        }
        {
          name: 'Level'
          type: 'string'
          displayName: 'Level'
        }
        {
          name: 'ExecutionName_s'
          type: 'string'
          displayName: 'ExecutionName_s'
        }
        {
          name: 'RevisionName_s'
          type: 'string'
          displayName: 'RevisionName_s'
        }
        {
          name: 'ReplicaName_s'
          type: 'string'
          displayName: 'ReplicaName_s'
        }
        {
          name: 'Log_s'
          type: 'string'
          displayName: 'Log_s'
        }
        {
          name: 'Count_d'
          type: 'real'
          displayName: 'Count_d'
        }
      ]
    }
    retentionInDays: 30
  }
}

resource ContainerEvent 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerEvent'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerEvent'
      displayName: 'ContainerEvent'
    }
    retentionInDays: 30
  }
}

resource ContainerImageInventory 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerImageInventory'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerImageInventory'
      displayName: 'ContainerImageInventory'
    }
    retentionInDays: 30
  }
}

resource ContainerInstanceLog 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerInstanceLog'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerInstanceLog'
      displayName: 'ContainerInstanceLog'
    }
    retentionInDays: 30
  }
}

resource ContainerInventory 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerInventory'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerInventory'
      displayName: 'ContainerInventory'
    }
    retentionInDays: 30
  }
}

resource ContainerLog 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerLog'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerLog'
      displayName: 'ContainerLog'
    }
    retentionInDays: 30
  }
}

resource ContainerLogV2 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerLogV2'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerLogV2'
      displayName: 'ContainerLogV2'
    }
    retentionInDays: 30
  }
}

resource ContainerNodeInventory 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerNodeInventory'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerNodeInventory'
      displayName: 'ContainerNodeInventory'
    }
    retentionInDays: 30
  }
}

resource ContainerRegistryLoginEvents 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerRegistryLoginEvents'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerRegistryLoginEvents'
      displayName: 'ContainerRegistryLoginEvents'
    }
    retentionInDays: 30
  }
}

resource ContainerRegistryRepositoryEvents 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerRegistryRepositoryEvents'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerRegistryRepositoryEvents'
      displayName: 'ContainerRegistryRepositoryEvents'
    }
    retentionInDays: 30
  }
}

resource ContainerServiceLog 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'ContainerServiceLog'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'ContainerServiceLog'
      displayName: 'ContainerServiceLog'
    }
    retentionInDays: 30
  }
}

resource DatabricksAccounts 'Microsoft.OperationalInsights/workspaces/tables@2025-02-01' = {
  parent: logAnalyticsWorkspace
  name: 'DatabricksAccounts'
  properties: {
    totalRetentionInDays: 30
    plan: 'Analytics'
    schema: {
      name: 'DatabricksAccounts'
      displayName: 'DatabricksAccounts'
    }
    retentionInDays: 30
  }
}




// Outputs
output workspaceId string = logAnalyticsWorkspace.id
output workspaceName string = logAnalyticsWorkspace.name
output workspaceCustomerId string = logAnalyticsWorkspace.properties.customerId
