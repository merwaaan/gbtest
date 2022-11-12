#!/bin/bash

cargo build && RUST_LOG=info ./target/debug/server.exe
