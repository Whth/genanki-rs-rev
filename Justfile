#!/usr/bin/env just --justfile

lint:
    cargo clippy

publish:
    cargo publish --registry crates-io

test:
    cargo test
