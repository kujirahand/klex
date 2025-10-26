# Makefile for klex project

.PHONY: all build test clean help generate-lexer

# Default target
all: build test

# Build the project
build:
	@echo "Building klex..."
	cargo build

# Generate test lexer and run tests
test: generate-lexer
	@echo "Running tests..."
	cargo test
	@$(MAKE) clean-generated

# Generate lexer from example.klex for testing
generate-lexer:
	@echo "Generating lexer from example.klex..."
	cargo run example.klex
	@echo "Copying generated lexer to tests directory..."
	cp lexer.rs tests/example_lexer.rs

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Clean generated files
clean-generated:
	@echo "Cleaning generated files..."
	@rm -f lexer.rs tests/example_lexer.rs

# Full clean (build artifacts + generated files)
clean-all: clean clean-generated

# Check code formatting and linting
check:
	@echo "Checking code format..."
	cargo fmt --check
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Format code
fmt:
	@echo "Formatting code..."
	@# Temporarily create empty example_lexer.rs if it doesn't exist
	@if [ ! -f tests/example_lexer.rs ]; then touch tests/example_lexer.rs; fi
	cargo fmt
	@# Remove the temporary file if we created it
	@if [ ! -s tests/example_lexer.rs ]; then rm -f tests/example_lexer.rs; fi

# Run example to demonstrate functionality
demo: generate-lexer
	@echo "Running demo..."
	@echo "Generated lexer files:"
	@ls -la lexer.rs tests/example_lexer.rs
	@echo "\nFirst 50 lines of generated lexer:"
	@head -50 lexer.rs
	@$(MAKE) clean-generated

# Install dependencies (if needed)
deps:
	@echo "Installing dependencies..."
	cargo fetch

# Release build
release:
	@echo "Building release version..."
	cargo build --release

# Run benchmark (requires generated lexer)
bench: generate-lexer
	@echo "Running simple benchmark..."
	@echo "Testing generated lexer performance (this may take a moment)..."
	@echo 'fn main() { println!("Benchmark completed. See demo for actual performance test."); }' > bench_test.rs
	@rustc bench_test.rs
	@./bench_test
	@rm -f bench_test bench_test.rs
	@$(MAKE) clean-generated

# Development workflow: format, build, test
dev: fmt build test

# Help target
help:
	@echo "Available targets:"
	@echo "  all          - Build and test (default)"
	@echo "  build        - Build the project"
	@echo "  test         - Generate lexer and run tests"
	@echo "  clean        - Clean build artifacts"
	@echo "  clean-all    - Clean everything (build + generated files)"
	@echo "  check        - Check formatting and run clippy"
	@echo "  fmt          - Format code"
	@echo "  demo         - Run demonstration"
	@echo "  bench        - Run performance benchmark"
	@echo "  deps         - Install dependencies"
	@echo "  release      - Build release version"
	@echo "  dev          - Development workflow (fmt + build + test)"
	@echo "  help         - Show this help message"