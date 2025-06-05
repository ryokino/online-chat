.PHONY: build test lint format clean run-server run-client help

# Default target
all: build test lint

# Build commands
build:
	cargo build --workspace

build-release:
	cargo build --workspace --release

# Test commands
test:
	cargo test --workspace --verbose

test-server:
	cargo test -p server --verbose

test-client:
	cargo test -p client --verbose

test-protocol:
	cargo test -p protocol --verbose

# Lint and format commands
lint:
	cargo clippy -- -D warnings

format:
	cargo fmt --all

format-check:
	cargo fmt --all --check

# Run commands
run-server:
	cargo run -p server

run-client:
	cargo run -p client

# Clean command
clean:
	cargo clean

# CI commands (mimics GitHub Actions)
ci: test lint format-check

# Help command
help:
	@echo "Available targets:"
	@echo "  build         - Build entire workspace"
	@echo "  build-release - Build release version"
	@echo "  test          - Run all tests"
	@echo "  test-server   - Run server tests only"
	@echo "  test-client   - Run client tests only"
	@echo "  test-protocol - Run protocol tests only"
	@echo "  lint          - Run clippy linter"
	@echo "  format        - Format all code"
	@echo "  format-check  - Check if code is formatted"
	@echo "  run-server    - Run server (port 9001)"
	@echo "  run-client    - Run client (port 9050)"
	@echo "  clean         - Clean build artifacts"
	@echo "  ci            - Run CI checks (test + lint + format-check)"
	@echo "  help          - Show this help message"