
terraform {
  required_version = "~> 1.1"
  required_providers {
    azurerm = {
      version = "~> 3.23"
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

data "azurerm_key_vault" "keyvault" {
  name                = "keyvdemobook"
  resource_group_name = "rg_keyvault"
}

data "azurerm_key_vault_secret" "app-connectionstring" {
  name         = "ConnectionStringApp"
  key_vault_id = data.azurerm_key_vault.keyvault.id
}

resource "azurerm_resource_group" "rg-app" {
  name     = "RG-DEMOVAULT"
  location = "West Europe"
}

resource "azurerm_service_plan" "plan-app" {
  name                = "SP-demovault"
  location            = azurerm_resource_group.rg-app.location
  resource_group_name = azurerm_resource_group.rg-app.name

  sku_name = "S1"
  os_type  = "Windows"
}

resource "azurerm_linux_web_app" "app" {
  name                = "demovaultbook-${random_string.random.result}"
  location            = azurerm_resource_group.rg-app.location
  resource_group_name = azurerm_resource_group.rg-app.name
  service_plan_id     = azurerm_service_plan.plan-app.id

  connection_string {
    name  = "Database"
    type  = "SQLServer"
    value = data.azurerm_key_vault_secret.app-connectionstring.value
  }

  site_config {}
}
