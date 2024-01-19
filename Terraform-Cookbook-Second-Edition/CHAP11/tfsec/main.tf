terraform {
  required_version = "~> 1.1"
  required_providers {
    azurerm = {
      version = "~> 3.35"
    }
  }
}

provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "rg" {
  name     = "rg-demo-opa"
  location = "westeurope"
}

resource "azurerm_storage_account" "storage" {
  name                      = "sademotesttfsec123"
  resource_group_name       = azurerm_resource_group.rg.name
  location                  = azurerm_resource_group.rg.location
  account_tier              = "Standard"
  account_replication_type  = "GRS"
  enable_https_traffic_only = false #incorrect
  #enable_https_traffic_only = false #correct
  #min_tls_version           = "TLS1_2" #to add to be secure
}