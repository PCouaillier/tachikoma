#!/usr/bin/env bash

cargo rustc --release -q -- -Awarnings
./halite -d "240 160" "target/release/tachikoma" "target/release/tachikoma"
