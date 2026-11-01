#!/bin/bash

set -e

cargo build --example simple_input --quiet

WORD="$1"

output=$(echo "$WORD" | cargo run --example simple_input --quiet 2>/dev/null | tr -d '\0')

expected=$(awk -v word="$WORD" 'BEGIN {
    for (i = 1; i <= length(word); i++) {
        c = substr(word, i, 1)
        if (c == "\n") print "You type:Enter"
        else if (c == " ") print "You type:Space"
        else print "You type:"c
    }
}')

if [[ "$output" == *"$expected"* ]]; then
    echo -e "Ok"
    exit 0
else
    echo -e "Error expected:\n$expected"
    echo "Got:\n$output"
    exit 1
fi
