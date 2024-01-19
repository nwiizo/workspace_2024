terraform {
  required_version = "~> 1.1"
  required_providers {
    azurerm = {
      version = "~> 3.23"
    }
    random = {
      source  = "hashicorp/random"
      version = "2.3.0"
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

locals {
  linux_web_app   = toset([for each in var.web_apps : each.name if each.os == "Linux"])
  windows_web_app = toset([for each in var.web_apps : each.name if each.os == "Windows"])
  default_app_settings = {
    "DEFAULT_KEY1" = "DEFAULT_VAL1"
  }
}

resource "azurerm_resource_group" "rg_app" {
  name     = "${var.resource_group_name}-${var.environment}-${random_string.random.result}"
  location = var.location
  tags = {
    ENV = var.environment
  }
}

resource "azurerm_service_plan" "linux_plan_app" {
  name                = "${var.service_plan_name}-${var.environment}-linux-${random_string.random.result}"
  location            = azurerm_resource_group.rg_app.location
  resource_group_name = azurerm_resource_group.rg_app.name
  os_type             = "Linux"
  sku_name            = "S1"
}

resource "azurerm_service_plan" "windows_plan_app" {
  name                = "${var.service_plan_name}-${var.environment}-windows-${random_string.random.result}"
  location            = azurerm_resource_group.rg_app.location
  resource_group_name = azurerm_resource_group.rg_app.name
  os_type             = "Windows"
  sku_name            = "S1"
}

resource "azurerm_linux_web_app" "app" {
  for_each = local.linux_web_app

  name                = each.value
  location            = "westeurope"
  resource_group_name = azurerm_resource_group.rg_app.name
  service_plan_id     = azurerm_service_plan.linux_plan_app.id

  site_config {}
  app_settings = merge(local.default_app_settings, var.custom_app_settings)

}

resource "azurerm_windows_web_app" "app" {
  for_each = local.windows_web_app

  name                = each.value
  location            = "westeurope"
  resource_group_name = azurerm_resource_group.rg_app.name
  service_plan_id     = azurerm_service_plan.windows_plan_app.id

  site_config {}

  app_settings = merge(local.default_app_settings, var.custom_app_settings)
}