.PHONY: dev fmt-check fmt clippy clippy-release fuzz coverage manual-coverage clean

dev: fmt clippy

fmt-check:
	taplo fmt --check
	cargo fmt --all -- --check

fmt:
	taplo fmt
	cargo fmt --all

clippy:
	cargo clippy --all --all-targets -- -D warnings

# Example: `make fuzz target=from_phrase`
RUN_NUMBER=1000
fuzz:
	cargo +nightly fuzz run $(target) -- -runs=$(RUN_NUMBER)

# Example `make coverage target=from_phrase`
HOST=$(shell rustc -vV | grep "host" | awk '{print $$2}')
coverage: fuzz
	cargo +nightly fuzz coverage $(target)
	llvm-cov show target/$(HOST)/release/$(target) -instr-profile=coverage/$(target)/coverage.profdata --format=html > $(target).html

# If you hit an error like Unsupported instrumentation profile format version or Unsupported profiling format version
# or Failed to load coverage: Unsupported coverage format version running llvm-profdata or llvm-cov that means
# you have a mismatch in the versions between the clang++ compiler and the llvm-cov or llvm-profdata tools.
manual-coverage: fuzz
	cargo +nightly fuzz coverage $(target)
	llvm-profdata merge -sparse coverage/$(target)/raw/ -o coverage/$(target)/coverage.profdata
	llvm-cov show target/$(HOST)/release/$(target) -instr-profile=coverage/$(target)/coverage.profdata --format=html > $(target).html

clean:
	cargo clean
	rm -rf artifacts corpus coverage
