.PHONY: help check fmt fmt-check clippy test build run clean

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

check: fmt-check clippy test ## Everything CI runs

fmt: ## Format the whole workspace
	cargo fmt --all

fmt-check: ## Verify formatting without changing files
	cargo fmt --all -- --check

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
