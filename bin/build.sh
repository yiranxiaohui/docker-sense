#!/bin/bash
cargo build --release --target x86_64-unknown-linux-musl
cp -f ./target/x86_64-unknown-linux-musl/release/DockSense ./bin/DockSense