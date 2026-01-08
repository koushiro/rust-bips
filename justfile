# Show all commands
[default]
help:
    @just --list --list-heading $'Available commands:\n'

# Cleanup compilation outputs
clean:
    @cargo clean

# Check the code format
fmt-check:
    @taplo fmt --check
    @cargo fmt --all -- --check

# Format the code
fmt:
    @taplo fmt
    @cargo +nightly fmt --all

# Run rust clippy
clippy *args='':
    @cargo clippy --workspace --all-targets --all-features {{ args }} -- -D warnings

# Check code
check *args='':
    @cargo check --workspace --all-targets --all-features {{ args }}

# Build workspace
build *args='':
    @cargo build --workspace --all-targets --all-features {{ args }}

# Run all tests
test *args='':
    @cargo test --workspace --all-features {{ args }}

# Generate docs
gen-docs *args='':
    @cargo doc --no-deps --workspace --lib --all-features {{ args }}
