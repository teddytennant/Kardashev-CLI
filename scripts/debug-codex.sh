#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/kardashev/scripts/debug-kardashev.sh" in VSCode settings to always get the 
# latest kardashev-rs binary when debugging Codex Extension.


set -euo pipefail

CODEX_RS_DIR=$(realpath "$(dirname "$0")/../kardashev-rs")
(cd "$CODEX_RS_DIR" && cargo run --quiet --bin kardashev -- "$@")