terraform {
  required_version = "~> 1.1"
  required_providers {
    azurerm = {
      version = "~> 3.23"
    }
  }
}

provider "azurerm" {
  features {}
}

data "azurerm_resources" "nsg" {
  type                = "Microsoft.Network/networkSecurityGroups"
  resource_group_name = "RG-DEMO"
  required_tags = {
    DEFAULTRULES = "TRUE"
  }
}


resource "azurerm_network_security_rule" "default-rules" {
  for_each                    = { for n in data.azurerm_resources.nsg.resources : n.name => n }
  name                        = "${each.key}-SSH"
  priority                    = 100
  direction                   = "Inbound"
  access                      = "Allow"
  protocol                    = "Tcp"
  source_port_range           = "*"
  destination_port_range      = "22"
  source_address_prefix       = "*"
  destination_address_prefix  = "*"
  resource_group_name         = "RG-DEMO"
  network_security_group_name = each.key
}

output "nsg" {
  value = { for n in data.azurerm_resources.nsg.resources : n.name => n }
}