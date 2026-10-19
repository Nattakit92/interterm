#!/bin/bash

set -e

cargo build --example simple_input --quiet

WORD="$1"

output=$(echo "$WORD" | fold -w1 | while read -n1 char; do echo -n "$char"; sleep 0.05; done | cargo run --example simple_input --quiet 2>/dev/null | tr -d '\0')

expected=$(echo "$WORD" | fold -w1 | sed 's/^/You type:/')

if [[ "$output" == *"$expected"* ]]; then
    echo -e "Ok"
    exit 0
else
    echo -e "Error expected:\n$expected"
    echo "Got:\n$output"
    exit 1
fi
