.PHONY: help check fmt fmt-check toolchain-check clippy test build run clean

# The exact toolchain (rust-toolchain.toml) and the declared MSRV (Cargo.toml)
# must agree. Extract both here so `toolchain-check` can compare them.
TOOLCHAIN_CHANNEL := $(shell sed -n 's/^channel = "\(.*\)"/\1/p' rust-toolchain.toml)
CARGO_MSRV        := $(shell sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml)

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}'

check: fmt-check toolchain-check clippy test ## Everything CI runs

fmt: ## Format the whole workspace
	cargo fmt --all

fmt-check: ## Verify formatting without changing files
	cargo fmt --all -- --check

toolchain-check: ## Verify rust-toolchain.toml and Cargo.toml agree on the version
	@set -eu; \
	channel='$(TOOLCHAIN_CHANNEL)'; \
	msrv='$(CARGO_MSRV)'; \
	case "$$channel" in \
		[0-9]*.[0-9]*) ;; \
		*) echo "ERROR: rust-toolchain.toml channel must be a concrete version, got '$$channel'"; exit 1 ;; \
	esac; \
	norm() { printf '%s\n' "$$1" | awk -F. '{printf "%s.%s.%s\n", $$1, $$2, ($$3 == "" ? "0" : $$3)}'; }; \
	nc=$$(norm "$$channel"); \
	nm=$$(norm "$$msrv"); \
	if [ "$$nc" != "$$nm" ]; then \
		echo "ERROR: version mismatch — rust-toolchain.toml channel '$$channel' vs Cargo.toml rust-version '$$msrv'"; \
		exit 1; \
	fi; \
	echo "toolchain version OK: $$nc"

clippy: ## Lint everything, warnings are errors
	cargo clippy --workspace --all-targets -- -D warnings

test: ## Run all tests
	cargo test --workspace

build: ## Build an optimised binary
	cargo build --workspace --release

run: ## Run the CLI (composition root)
	cargo run -p app-cli

clean: ## Remove build artifacts
	cargo clean
