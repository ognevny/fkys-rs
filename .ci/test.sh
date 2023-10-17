#!/usr/bin/sh

output="$(./target/debug/fkysoxide main.fkys)"
if [[ $output != "Hello, world!" ]]; then
    printf "%s\n%s\n%s\n%s\n" "failed with" "~~~~~~~~" "$output" "~~~~~~~~"
    exit 1
fi

printf "\n%s\n" "$output"
