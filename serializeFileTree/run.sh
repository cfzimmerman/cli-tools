#!/bin/bash 

(cat paths.txt) | \
  (python3 makeTree.py '^\.git$|^target$|^node_modules$') | \
  python3 -m json.tool

