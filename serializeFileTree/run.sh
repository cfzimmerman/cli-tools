#!/bin/bash 

(cat paths.txt) | python3 makeTree.py | python3 -m json.tool

