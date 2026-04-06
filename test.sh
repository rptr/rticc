#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <path>"
    exit 1
fi

find "$1" -type f | sort | while read -r file; do
    echo "Testing: $file"
    ./target/debug/rticc "$file"
done
