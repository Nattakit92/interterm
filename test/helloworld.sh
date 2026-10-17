#!/bin/bash

set -e

cargo build --example simple_input --quiet

output=$(echo "helloworld" | fold -w1 | while read -n1 char; do echo -n "$char"; sleep 0.05; done | cargo run --example simple_input --quiet 2>/dev/null | tr -d '\0')

expected="You type:h
You type:e
You type:l
You type:l
You type:o
You type:w
You type:o
You type:r
You type:l
You type:d"

if [[ "$output" == *"$expected"* ]]; then
    echo -e "Ok"
    exit 0
else
    echo -e "Error expected:\n $expected"
    echo "Got:\n $output"
    exit 1
fi
