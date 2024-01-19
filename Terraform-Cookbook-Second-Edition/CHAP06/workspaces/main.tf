terraform {
  required_version = ">= 1.0"
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "3.29.1"
    }
  }
}

provider "azurerm" {
  features {}
}


resource "azurerm_resource_group" "rg-app" {
  name     = "RG-APP-${terraform.workspace}"
  location = "westeurope"
}