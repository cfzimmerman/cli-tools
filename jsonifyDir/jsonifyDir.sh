#!/bin/bash

script_dir="$(dirname "$(readlink -f $0)")"
file_path="$1"

(find "$file_path" -type f) | \
  python3 "$script_dir/../serializeFileTree/makeTree.py" | \
  python3 -m json.tool 
