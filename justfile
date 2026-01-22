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

# Run a benchmark: `just bench <name> [args]`
[working-directory('benchmarks')]
bench name *args:
    @if [ -n "{{ args }}" ]; then \
        echo "Running benchmark \"{{ name }}\" with: {{ args }}"; \
        cargo bench --bench {{ name }} -- {{ args }}; \
    else \
        echo "Running benchmark \"{{ name }}\""; \
        cargo bench --bench {{ name }} -- --quiet; \
    fi

# Run all benchmarks: `just benches [args]`
[working-directory('benchmarks')]
benches *args:
    @if [ -n "{{ args }}" ]; then \
        echo "Running all benchmarks with: {{ args }}"; \
        cargo bench -- {{ args }}; \
    else \
        echo "Running all benchmarks"; \
        cargo bench -- --quiet; \
    fi

# Run fuzz: `just fuzz <target> [runs]`
[positional-arguments]
[working-directory('fuzz')]
fuzz target runs='1000':
    #!/usr/bin/env bash
    set -eo pipefail
    echo "Running fuzzer on target \"{{ target }}\" with {{ runs }} runs"
    cargo +nightly fuzz run {{ target }} -- -runs={{ runs }}

# Cleanup fuzz artifacts
[working-directory('fuzz')]
fuzz-clean:
    #!/usr/bin/env bash
    set -eo pipefail
    rm -rf artifacts corpus
