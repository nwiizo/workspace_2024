
terraform {
  required_version = "~> 1.1"
  backend "azurerm" {
    resource_group_name  = "RG-TFBACKEND"
    storage_account_name = "storagetfbackend"
    container_name       = "tfstate"
    key                  = "myapp.tfstate"
  }
  required_providers {
    azurerm = {
      version = "~> 3.23"
    }
  }
}

provider "azurerm" {
  features {
  }
}


resource "azurerm_resource_group" "rg-app" {
  name     = "${var.resource_group_name}-${var.environment}"
  location = var.location
  tags = {
    ENV = var.environment
  }
}


data "terraform_remote_state" "service_plan_tfstate" {
  backend = "azurerm"
  config = {
    resource_group_name  = "rg_tfstate"
    storage_account_name = "storstate"
    container_name       = "tfbackends"
    key                  = "serviceplan.tfstate"
  }
}

resource "azurerm_windows_web_app" "app" {
  name                = "${var.app_name}-${var.environment}"
  location            = azurerm_resource_group.rg-app.location
  resource_group_name = azurerm_resource_group.rg-app.name
  service_plan_id     = data.terraform_remote_state.service_plan_tfstate.outputs.service_plan_id
  site_config {}
}
