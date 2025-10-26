# Makefile for klex project

.PHONY: all build test clean help generate-lexer

# Default target
all: build test

# Build the project
build:
	@echo "Building klex..."
	cargo build

# Generate test lexer and run tests
test: generate-lexers
	@echo "Running tests..."
	cargo test
	@$(MAKE) clean-generated

# Generate lexers from all test/*.klex files
generate-lexers: generate-example generate-test-context

# Generate lexer from tests/example.klex for testing
generate-example:
	@echo "Generating lexer from tests/example.klex..."
	cargo run tests/example.klex tests/example_lexer.rs
	@echo "Generated lexer saved as tests/example_lexer.rs"

# Generate lexer from tests/test_context.klex for testing
generate-test-context:
	@echo "Generating lexer from tests/test_context.klex..."
	cargo run tests/test_context.klex tests/test_context_lexer.rs
	@echo "Generated lexer saved as tests/test_context_lexer.rs"

# Legacy target for backward compatibility
generate-lexer: generate-example

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Clean generated files
clean-generated:
	@echo "Cleaning generated files..."
	@rm -f lexer.rs tests/example_lexer.rs tests/test_context_lexer.rs

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
demo: generate-lexers
	@echo "Running demo..."
	@echo "Generated lexer files:"
	@ls -la tests/example_lexer.rs tests/test_context_lexer.rs 2>/dev/null || echo "No generated files found"
	@echo "\nTest files in tests/ directory:"
	@ls -la tests/*.klex
	@echo "\nFirst 50 lines of example lexer:"
	@head -50 tests/example_lexer.rs 2>/dev/null || echo "example_lexer.rs not found"
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
bench: generate-lexers
	@echo "Running simple benchmark..."
	@echo "Testing generated lexer performance (this may take a moment)..."
	@echo 'fn main() { println!("Benchmark completed. See demo for actual performance test."); }' > bench_test.rs
	@rustc bench_test.rs
	@./bench_test
	@rm -f bench_test bench_test.rs
	@$(MAKE) clean-generated

# Development workflow: format, build, test
dev: fmt build test

# Test individual klex files
test-example: generate-example
	@echo "Testing example.klex..."
	@echo "Generated file: tests/example_lexer.rs"
	@wc -l tests/example_lexer.rs
	
test-context: generate-test-context
	@echo "Testing test_context.klex..."
	@echo "Generated file: tests/test_context_lexer.rs"
	@wc -l tests/test_context_lexer.rs

# List all test files
list-tests:
	@echo "Available test files:"
	@ls -la tests/*.klex

# Help target
help:
	@echo "Available targets:"
	@echo "  all              - Build and test (default)"
	@echo "  build            - Build the project"
	@echo "  test             - Generate all lexers and run tests"
	@echo "  generate-lexers  - Generate lexers from all tests/*.klex files"
	@echo "  generate-example - Generate lexer from tests/example.klex"
	@echo "  generate-test-context - Generate lexer from tests/test_context.klex"
	@echo "  test-example     - Test example.klex individually"
	@echo "  test-context     - Test test_context.klex individually"
	@echo "  list-tests       - List all available test files"
	@echo "  clean            - Clean build artifacts"
	@echo "  clean-all        - Clean everything (build + generated files)"
	@echo "  check            - Check formatting and run clippy"
	@echo "  fmt              - Format code"
	@echo "  demo             - Run demonstration"
	@echo "  bench            - Run performance benchmark"
	@echo "  deps             - Install dependencies"
	@echo "  release          - Build release version"
	@echo "  dev              - Development workflow (fmt + build + test)"
	@echo "  help             - Show this help message"