#!/bin/sh

## Classic run
APP_ENVIRONMENT=local cargo run

## Run using the debug level.
## APP_ENVIRONMENT=local RUST_LOG=trace cargo run
