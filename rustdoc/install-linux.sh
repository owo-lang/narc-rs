#!/bin/sh
cargo update
cargo install --path . --bin narc --force
cargo clippy -- --allow pedantic --allow nursery --allow cargo
