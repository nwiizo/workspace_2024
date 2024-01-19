
terraform {
  required_version = "~> 1.1"
  required_providers {
    azurerm = {
      version = "~> 3.35"
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

resource "azurerm_resource_group" "rg-app" {
  name     = "RG-DEMO-ARM"
  location = "westeurope"
}

resource "azurerm_service_plan" "plan-app" {
  name                = "SPDemo"
  location            = "westeurope"
  resource_group_name = azurerm_resource_group.rg-app.name

  os_type  = "Windows"
  sku_name = "S1"
}

resource "azurerm_linux_web_app" "app" {
  name                = "webapparm${random_string.random.result}"
  location            = azurerm_resource_group.rg-app.location
  resource_group_name = azurerm_resource_group.rg-app.name
  service_plan_id     = azurerm_service_plan.plan-app.id

  site_config {}
}


resource "azurerm_resource_group_template_deployment" "extension" {
  name                = "extension"
  resource_group_name = azurerm_resource_group.rg-app.name
  template_content    = file("ARM_siteExtension.json")

  parameters_content = jsonencode({
    "appserviceName" = {
      value = azurerm_linux_web_app.app.name
    },
    "extensionName" = {
      value = "AspNetCoreRuntime.2.2.x64"
    },
    "extensionVersion" = {
      value = "2.2.0-preview3-35497"
    }
  })

  deployment_mode = "Incremental"
}