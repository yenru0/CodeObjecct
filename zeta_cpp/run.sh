#!/bin/bash
if [ -z "$*" ]; then echo "No args"; exit 0; fi
g++ -std=c++17 $1
./a.out < stdin.txt
