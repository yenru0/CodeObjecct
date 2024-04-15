#!/bin/bash
if [ -z "$*" ]; then echo "No args"; exit 0; fi
gcc -std=c11 $1
./a.out < stdin.txt