#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

if [ -z "$1" ]; then
    echo "Usage: $0 <path>"
    exit 1
fi

find "$1" -type f | sort | while read -r file; do
    if RUST_BACKTRACE=1 ./target/debug/rticc "$file"; then
        echo -e "${GREEN}PASS${NC}: $file"
    else
        echo -e "${RED}FAIL${NC}: $file"
    fi
done
