using '../main.bicep'

param secretKey = string(readEnvironmentVariable('SECRET_KEY'))
param azureClientSecret = string(readEnvironmentVariable('AZURE_CLIENT_SECRET'))
param azureTenantId = string(readEnvironmentVariable('AZURE_TENANT_ID'))
param azureClientId = string(readEnvironmentVariable('AZURE_CLIENT_ID'))
param contributorPrincipalIds = array(readEnvironmentVariable('CONTRIBUTOR_PRINCIPAL_IDS'))


