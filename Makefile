.PHONY: setup
# Setup development environment
setup:
	bash ./scripts/setup-dev.sh

.PHONY: clean
# Cleanup compilation outputs
clean:
	cargo clean

.PHONY: fmt-check fmt
# Check the code format
fmt-check:
	taplo fmt --check
	cargo fmt --all -- --check
# Format the code
fmt:
	taplo fmt
	cargo +nightly fmt --all

.PHONY: clippy clippy-release
# Run rust clippy with debug profile
clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings
# Run rust clippy with release profile
clippy-release:
	cargo clippy --workspace --all-targets --all-features --release -- -D warnings

.PHONY: check check-release
# Check code with debug profile
check:
	cargo check --workspace --all-targets --all-features
# Check code with release profile
check-release:
	cargo check --workspace --all-targets --all-features --release

.PHONY: build build-release
# Build with debug profile
build:
	cargo build --workspace --all-targets --all-features
# Build with release profile
build-release:
	cargo build --workspace --all-targets --all-features --release

.PHONY: test test-release
# Run all tests with debug profile
test:
	cargo test --workspace --all-features
# Run all tests with release profile
test-release:
	cargo test --workspace --all-features --release

.PHONY: doc
# Generate docs
gen-doc:
	cargo doc --no-deps --workspace --lib --all-features

.PHONY: help
# Show help
help:
	@echo ''
	@echo 'Usage:'
	@echo ' make [target]'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
	helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf "\033[36m%-30s\033[0m %s\n", helpCommand,helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help
