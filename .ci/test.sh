#!/usr/bin/sh

hw="$(./target/debug/fkysoxide ./test_scripts/hw.fkys ||
    ./target/release/fkysoxide ./test_scripts/hw.fkys)"

nums="$(./target/debug/fkysoxide ./test_scripts/nums.fkys ||
    ./target/release/fkysoxide ./test_scripts/nums.fkys)"

utf="$(./target/debug/fkysoxide ./test_scripts/utf.fkys ||
    ./target/release/fkysoxide ./test_scripts/utf.fkys)"

if ! [ "$hw" = "Hello, world!" ]; then
    printf "\n%s\n%s\n%s\n%s\n" \
        "TEST 'hw' FAILED WITH" \
        "'$hw'" "WHILE IT SHOULD BE" \
        "'Hello, world!'"
    exit 1
fi
if ! [ "$nums" = "46 31 44" ]; then
    printf "\n%s\n%s\n%s\n%s\n" \
        "TEST 'nums' FAILED WITH" \
        "'$nums'" "WHILE IT SHOULD BE" \
        "'46 31 44'"
    exit 1
fi
if ! [ "$utf" = "я" ]; then
    printf "\n%s\n%s\n%s\n%s\n" \
        "TEST 'utf' FAILED WITH" \
        "'$utf'" "WHILE IT SHOULD BE" \
        "'я'"
    exit 1
fi

printf "\n%s\n%s\n%s\n" "$hw" "$nums" "$utf"
