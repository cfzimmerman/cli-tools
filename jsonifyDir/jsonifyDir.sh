#!/bin/bash

script_dir="$(dirname "$(readlink -f $0)")"
file_path="$1"
exclude_regex="$2"

(find "$file_path" -type f) | \
  (python3 "$script_dir/../serializeFileTree/makeTree.py" "$exclude_regex") | \
  python3 -m json.tool 
