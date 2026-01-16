.PHONY: dev build run clean

# Development server with hot reload
dev:
	cargo run -p nucleus-cli -- dev

# Build the application
build:
	cargo run -p nucleus-cli -- build

# Run the built application
run:
	cargo run --bin site

# Clean build artifacts
clean:
	cargo clean
	rm -rf static/css/style-*.css
