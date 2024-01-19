terraform {
  required_version = "~> 1.1"
}

module "template" {
  source            = "../../"
  raw_string_list   = var.raw_string_list_test
  permutation_count = var.permutation_count_test
}
