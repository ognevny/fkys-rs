#!/usr/bin/sh

./target/debug/fkysoxide main.fkys > output.txt
if [ "$(cat output.txt)" = "Hello, world!\n" ]; then
    echo "failed with $(cat output.txt)"
    rm -f output.txt
    exit 1
fi
rm -f output.txt
