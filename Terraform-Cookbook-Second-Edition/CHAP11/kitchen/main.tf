terraform {
  required_version = "~> 1.1"
  required_providers {
    local = {
      source  = "hashicorp/local"
      version = "2.4.0"
    }
  }
}

variable "vmhosts" {
  type    = list(string)
  default = ["vmwebdemo1", "vmwebdemo2"]
}

variable "vmips" {
  type    = list(string)
  default = ["0.0.0.1", "0.0.0.2"]
}


resource "local_file" "inventory" {
  filename = "inventory"
  content = templatefile("template-inventory.tpl",
    {
      vm_ip  = var.vmips
      vm_dns = var.vmhosts
  })
}
