# sysaudit Makefile
# Usage: make <target>

.PHONY: all build release test lint fmt clean docs install publish help

# Default target
all: fmt lint test build

#───────────────────────────────────────────────────────────────
# BUILD TARGETS
#───────────────────────────────────────────────────────────────

build:                      ## Build debug binaries
	cargo build --workspace

release:                    ## Build optimized release binaries
	cargo build --workspace --release

install:                    ## Install CLI to ~/.cargo/bin
	cargo install --path sysaudit-cli

#───────────────────────────────────────────────────────────────
# QUALITY TARGETS
#───────────────────────────────────────────────────────────────

test:                       ## Run all tests
	cargo test --workspace --all-features

test-verbose:               ## Run tests with output
	cargo test --workspace --all-features -- --nocapture

lint:                       ## Run clippy linter
	cargo clippy --workspace --all-targets -- -D warnings

fmt:                        ## Format code
	cargo fmt --all

fmt-check:                  ## Check formatting without changes
	cargo fmt --all -- --check

#───────────────────────────────────────────────────────────────
# DOCUMENTATION
#───────────────────────────────────────────────────────────────

docs:                       ## Generate and open documentation
	cargo doc --workspace --no-deps --open

docs-build:                 ## Generate documentation only
	cargo doc --workspace --no-deps

#───────────────────────────────────────────────────────────────
# RELEASE & PUBLISH
#───────────────────────────────────────────────────────────────

publish-dry:                ## Dry run publish to crates.io
	cargo publish -p sysaudit --dry-run
	cargo publish -p sysaudit-cli --dry-run

publish:                    ## Publish to crates.io (requires CARGO_REGISTRY_TOKEN)
	cargo publish -p sysaudit
	cargo publish -p sysaudit-cli

version-patch:              ## Bump patch version (0.1.0 -> 0.1.1)
	cargo set-version --bump patch

version-minor:              ## Bump minor version (0.1.0 -> 0.2.0)
	cargo set-version --bump minor

version-major:              ## Bump major version (0.1.0 -> 1.0.0)
	cargo set-version --bump major

#───────────────────────────────────────────────────────────────
# CLEAN & UTILITIES
#───────────────────────────────────────────────────────────────

clean:                      ## Remove build artifacts
	cargo clean

run:                        ## Run full audit with debug build
	cargo run -p sysaudit-cli -- all

run-release:                ## Run CLI with release build
	cargo run -p sysaudit-cli --release -- $(ARGS)

check:                      ## Run full test suite (test + lint + fmt)
	sh scripts/test_all.sh

verify:                     ## Run verification script (WMI compare)
	pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify_output.ps1

#───────────────────────────────────────────────────────────────
# CI SIMULATION
#───────────────────────────────────────────────────────────────

ci:                         ## Simulate full CI pipeline locally
	$(MAKE) fmt-check
	$(MAKE) lint
	$(MAKE) test
	$(MAKE) build
	$(MAKE) docs-build
	@echo "CI simulation passed"

#───────────────────────────────────────────────────────────────
# HELP
#───────────────────────────────────────────────────────────────

help:                       ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-18s %s\n", $$1, $$2}'
