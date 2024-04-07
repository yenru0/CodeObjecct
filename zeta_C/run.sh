#!/bin/bash
if [ -z "$*" ]; then echo "No args"; exit 0; fi
clang-18 -std=c11 $1
./a.out < stdin.txt