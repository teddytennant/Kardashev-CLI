set working-directory := "kardashev-rs"
set positional-arguments

# Display help
help:
    just -l

# `kardashev`
alias c := kardashev
kardashev *args:
    cargo run --bin kardashev -- "$@"

# `kardashev exec`
exec *args:
    cargo run --bin kardashev -- exec "$@"

# `kardashev tui`
tui *args:
    cargo run --bin kardashev -- tui "$@"

# Run the CLI version of the file-search crate.
file-search *args:
    cargo run --bin kardashev-file-search -- "$@"

# Build the CLI and run the app-server test client
app-server-test-client *args:
    cargo build -p kardashev-cli
    cargo run -p kardashev-app-server-test-client -- --kardashev-bin ./target/debug/kardashev "$@"

# format code
fmt:
    cargo fmt -- --config imports_granularity=Item

fix *args:
    cargo clippy --fix --all-features --tests --allow-dirty "$@"

clippy:
    cargo clippy --all-features --tests "$@"

install:
    rustup show active-toolchain
    cargo fetch

# Run `cargo nextest` since it's faster than `cargo test`, though including
# --no-fail-fast is important to ensure all tests are run.
#
# Run `cargo install cargo-nextest` if you don't have it installed.
test:
    cargo nextest run --no-fail-fast

# Run the MCP server
mcp-server-run *args:
    cargo run -p kardashev-mcp-server -- "$@"
