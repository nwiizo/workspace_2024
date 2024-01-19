#!/bin/bash
echo "creating sample product"

IN=$(cat)
echo "stdin: ${IN}" #the old state, not useful for create step since the old state was empty

/bin/cat <<END >book.json
  {"id": "1", "title": "Terraform Cookbook", "Author": "MK", "tags": "terraform-Azure"}
END
cat book.json