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

module "resourcegroup" {
  source              = "git::https://github.com/mikaelkrief/terraform-azurerm-resource-group.git?ref=0.0.1"
  resource_group_name = "RG_MyAPP_Demo2-${random_string.random.result}"
  location            = "West Europe"
}

module "webapp" {
  source               = "git::https://github.com/mikaelkrief/terraform-azurerm-webapp.git?ref=v1.0.0"
  service_plan_name    = "spmyapp2-${random_string.random.result}"
  app_name             = "myappdemobook2-${random_string.random.result}"
  location             = "West Europe"
  resource_groupe_name = module.resourcegroup.resource_group_name
}


module "network" {
  source                  = "git::https://github.com/Azure/terraform-azurerm-network.git?ref=5.3.0"
  resource_group_name     = module.resourcegroup.resource_group_name
  resource_group_location = "West Europe"
  address_space           = "10.0.0.0/16"
  subnet_prefixes         = ["10.0.1.0/24", "10.0.2.0/24", "10.0.3.0/24"]
  subnet_names            = ["subnet1", "subnet2", "subnet3"]
  use_for_each            = true
}