# Oxidize - Justfile
# Main commands for the Rust task manager project

# Set shell for Windows compatibility
set shell := ["powershell.exe", "-c"]

# Default recipe: run tests and format code
default: ci build clippy

# Run all tests
test:
    cargo test

# Format code with rustfmt
fmt:
    cargo fmt

# Check code without building
check:
    cargo check

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run the application
run:
    cargo run

# Run in release mode
run-release:
    cargo run --release

# Run clippy for linting
clippy:
    cargo clippy -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Build and serve the web version
web-serve:
    trunk serve

# Build the web version
web-build:
    trunk build

# Run tests with coverage (requires cargo-tarpaulin)
test-coverage:
    cargo tarpaulin --out html

# Full CI check: format, clippy, test
ci: fmt clippy test

# Update dependencies
update:
    cargo update

# Install required tools
install-tools:
    cargo install trunk
    cargo install cargo-tarpaulin

# Watch for changes and run tests
watch-test:
    cargo watch -x test

# Watch for changes and run the app
watch-run:
    cargo watch -x run

# Generate documentation
docs:
    cargo doc --open