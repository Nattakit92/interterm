#!/bin/bash

set -e

cargo build --example simple_input --quiet

output=$(echo "helloworld" | fold -w1 | while read -n1 char; do echo -n "$char"; sleep 0.05; done | cargo run --example simple_input --quiet 2>/dev/null | tr -d '\0')

expected="You type:0xd00000000000000000000000
You type:0xca0000000000000000000000
You type:0xd80000000000000000000000
You type:0xd80000000000000000000000
You type:0xde0000000000000000000000
You type:0xee0000000000000000000000
You type:0xde0000000000000000000000
You type:0xe40000000000000000000000
You type:0xd80000000000000000000000
You type:0xc80000000000000000000000"

if [[ "$output" == *"$expected"* ]]; then
    echo -e "Ok"
    exit 0
else
    echo -e "Error expected:\n $expected"
    echo "Got:\n $output"
    exit 1
fi
