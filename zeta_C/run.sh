#!/bin/bash
if [ -z "$*" ]; then echo "No args"; exit 0; fi
clang-18 -std=c17 $1
./a.out < stdin.txt