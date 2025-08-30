.PHONY: fmt-check fmt
# Check the code format
fmt-check:
	taplo fmt --check
	cargo fmt --all -- --check
# Format the code
fmt:
	taplo fmt
	cargo fmt --all

.PHONY: clippy clippy-release
# Run rust clippy with debug profile
clippy:
	cargo clippy --all-targets -- -D warnings
# Run rust clippy with release profile
clippy-release:
	cargo clippy --all-targets --release -- -D warnings

RUN_NUMBER=1000
.PHONY: fuzz
# Run fuzz, example: `make fuzz target=from_phrase`
fuzz:
	cargo +nightly fuzz run $(target) -- -runs=$(RUN_NUMBER)

HOST=$(shell rustc -vV | grep "host" | awk '{print $$2}')
.PHONY: coverage manual-coverage
# Run coverage, example: `make coverage target=from_phrase`
coverage: fuzz
	cargo +nightly fuzz coverage $(target)
	llvm-cov show target/$(HOST)/release/$(target) -instr-profile=coverage/$(target)/coverage.profdata --format=html > $(target).html
# If you hit an error like Unsupported instrumentation profile format version or Unsupported profiling format version
# or Failed to load coverage: Unsupported coverage format version running llvm-profdata or llvm-cov that means
# you have a mismatch in the versions between the clang++ compiler and the llvm-cov or llvm-profdata tools.
# Run coverage manually
manual-coverage: fuzz
	cargo +nightly fuzz coverage $(target)
	llvm-profdata merge -sparse coverage/$(target)/raw/ -o coverage/$(target)/coverage.profdata
	llvm-cov show target/$(HOST)/release/$(target) -instr-profile=coverage/$(target)/coverage.profdata --format=html > $(target).html

.PHONY: clean
# Cleanup compilation outputs
clean:
	cargo clean
	rm -rf artifacts corpus coverage

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
