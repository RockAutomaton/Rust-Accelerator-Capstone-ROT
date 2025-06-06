@description('The name of the Cosmos DB account')
param cosmosDbAccountName string

@description('The location for the Cosmos DB account')
param location string = 'UK South'

@description('The database name')
param databaseName string = 'device-data'

@description('The container name for telemetry data')
param telemetryContainerName string = 'telemetry'

@description('The partition key path for the telemetry container')
param telemetryPartitionKeyPath string = '/device_id'

@description('The total throughput limit for the Cosmos DB account')
param totalThroughputLimit int = 4000

@description('The backup interval in minutes')
param backupIntervalInMinutes int = 1440

@description('The backup retention interval in hours')
param backupRetentionIntervalInHours int = 48


resource cosmosDbAccount 'Microsoft.DocumentDB/databaseAccounts@2024-12-01-preview' = {
  name: cosmosDbAccountName
  location: location
  tags: {
    defaultExperience: 'Core (SQL)'
    'hidden-workload-type': 'Learning'
    'hidden-cosmos-mmspecial': ''
  }
  kind: 'GlobalDocumentDB'
  identity: {
    type: 'None'
  }
  properties: {
    publicNetworkAccess: 'Enabled'
    enableAutomaticFailover: true
    enableMultipleWriteLocations: false
    isVirtualNetworkFilterEnabled: false
    virtualNetworkRules: []
    disableKeyBasedMetadataWriteAccess: false
    enableFreeTier: false
    enableAnalyticalStorage: false
    analyticalStorageConfiguration: {
      schemaType: 'WellDefined'
    }
    databaseAccountOfferType: 'Standard'
    enableMaterializedViews: false
    capacityMode: 'Serverless'
    defaultIdentity: 'FirstPartyIdentity'
    networkAclBypass: 'None'
    disableLocalAuth: false
    enablePartitionMerge: false
    enablePerRegionPerPartitionAutoscale: false
    enableBurstCapacity: false

    minimalTlsVersion: 'Tls12'
    consistencyPolicy: {
      defaultConsistencyLevel: 'Session'
      maxIntervalInSeconds: 5
      maxStalenessPrefix: 100
    }
    locations: [
      {
        locationName: location
        isZoneRedundant: false
      }
    ]
    cors: []
    capabilities: []
    ipRules: []
    backupPolicy: {
      type: 'Periodic'
      periodicModeProperties: {
        backupIntervalInMinutes: backupIntervalInMinutes
        backupRetentionIntervalInHours: backupRetentionIntervalInHours
        backupStorageRedundancy: 'Local'
      }
    }
    networkAclBypassResourceIds: []
    diagnosticLogSettings: {
      enableFullTextQuery: 'None'
    }
    capacity: {
      totalThroughputLimit: totalThroughputLimit
    }
  }
}

resource cosmosDbDatabase 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases@2024-12-01-preview' = {
  parent: cosmosDbAccount
  name: databaseName
  properties: {
    resource: {
      id: databaseName
    }
  }
}

resource telemetryContainer 'Microsoft.DocumentDB/databaseAccounts/sqlDatabases/containers@2024-12-01-preview' = {
  parent: cosmosDbDatabase
  name: telemetryContainerName
  properties: {
    resource: {
      id: telemetryContainerName
      indexingPolicy: {
        indexingMode: 'consistent'
        automatic: true
        includedPaths: [
          {
            path: '/*'
          }
        ]
        excludedPaths: [
          {
            path: '/"_etag"/?'
          }
        ]
      }
      partitionKey: {
        paths: [
          telemetryPartitionKeyPath
        ]
        kind: 'Hash'
        version: 2
      }
      uniqueKeyPolicy: {
        uniqueKeys: []
      }
      conflictResolutionPolicy: {
        mode: 'LastWriterWins'
        conflictResolutionPath: '/_ts'
      }
      computedProperties: []
    }
  }
}

// Outputs
output cosmosDbAccountId string = cosmosDbAccount.id
output cosmosDbAccountName string = cosmosDbAccount.name
output databaseId string = cosmosDbDatabase.id
output telemetryContainerId string = telemetryContainer.id
output connectionString string = cosmosDbAccount.properties.documentEndpoint
