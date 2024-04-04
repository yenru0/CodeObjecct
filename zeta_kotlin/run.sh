#!/bin/bash
if [ -z "$*" ]; then echo "No args"; exit 0; fi
kotlinc-jvm $1 -include-runtime a.jar
java -jar a.jar -Dfile.encoding=UTF-8 < stdin.txt
