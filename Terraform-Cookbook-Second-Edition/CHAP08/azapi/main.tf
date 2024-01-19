
terraform {
  required_version = "~> 1.1"
  required_providers {
    azurerm = {
      version = "~> 3.35"
    }
    azapi = {
      source  = "Azure/azapi"
      version = "1.1.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "3.5.1"
    }
  }
}

provider "azurerm" {
  features {}
}

resource "random_string" "random" {
  length  = 4
  special = false
  upper   = false
}

resource "azurerm_resource_group" "rg" {
  name     = "rg-demo-azapi"
  location = "westeurope"
}

## Create Storage Account
resource "azurerm_storage_account" "storage" {
  name                     = "accountsftpdemo${random_string.random.result}"
  location                 = azurerm_resource_group.rg.location
  resource_group_name      = azurerm_resource_group.rg.name
  account_tier             = "Standard"
  account_replication_type = "LRS"
  min_tls_version          = "TLS1_2"
  is_hns_enabled           = true
}


resource "azapi_update_resource" "sftp_azpi_sftp" {
  type        = "Microsoft.Storage/storageAccounts@2021-09-01"
  resource_id = azurerm_storage_account.storage.id

  body = jsonencode({
    properties = {
      isSftpEnabled = true
    }
  })
  response_export_values = ["*"]
}