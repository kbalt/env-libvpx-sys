#!/bin/sh
touch build.rs
cargo build --no-default-features --features=generate
echo "Generated `out/ffi.fs`. You may want to copy this into `generated/`."
