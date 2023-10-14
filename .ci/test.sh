#!/usr/bin/sh

output="$(./target/debug/fkysoxide main.fkys)"
if [ $output != "Hello, world!" ]; then
    echo "failed with $output"
    exit 1
fi
