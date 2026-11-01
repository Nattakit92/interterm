#!/bin/bash

set -e

cargo build --example simple_input --quiet

WORD="$1"

output=$(echo "$WORD" | cargo run --example simple_input --quiet 2>/dev/null | tr -d '\0')

expected=""
while IFS= read -r -n1 c; do
    if [[ -z "$c" ]]; then
        expected+=$'You type:Enter\n'
    elif [[ "$c" == " " ]]; then
        expected+="You type:Space"$'\n'
    else
        expected+="You type:$c"$'\n'
    fi
done <<< "$WORD"

if [[ "$output" == *"$expected"* ]]; then
    echo -e "Ok"
    exit 0
else
    echo -e "Error expected:\n$expected"
    echo "Got:\n$output"
    exit 1
fi
