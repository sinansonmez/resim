# Resim developer commands.
# Run `make help` to see the available workflow targets.

.PHONY: help install install-wasm-pack check-wasm-pack build start test smoke wasm-test release-check ci

help:
	@echo "Available targets:"
	@echo "  make install  - Install React demo dependencies"
	@echo "  make install-wasm-pack - Install wasm-pack with cargo"
	@echo "  make check-wasm-pack   - Verify wasm-pack is available"
	@echo "  make build    - Build the wasm package and React demo bundle"
	@echo "  make start    - Start the local demo application"
	@echo "  make test     - Run Rust library tests"
	@echo "  make smoke    - Run the React demo smoke checks"
	@echo "  make wasm-test - Run wasm browser tests with wasm-pack"
	@echo "  make release-check - Verify cargo and npm publish artifacts"
	@echo "  make ci       - Run the local CI-equivalent checks"

install:
	# Install the React demo dependencies used by the local app.
	cd react && npm install

install-wasm-pack:
	# Install the wasm-pack CLI required to build the Rust package for the demo.
	cargo install wasm-pack

check-wasm-pack:
	@command -v wasm-pack >/dev/null 2>&1 || { \
		echo "wasm-pack is required but not installed."; \
		echo "Install it with: make install-wasm-pack"; \
		exit 1; \
	}

build: check-wasm-pack
	# Build the Rust wasm package and bundle the React demo.
	cd react && npm run build

start: check-wasm-pack
	# Start the local development server for the demo application.
	cd react && npm start

test:
	# Run the Rust test suite for the image-processing core.
	cd rust && cargo test

smoke:
	# Run the lightweight demo smoke checks for catalog/control behavior.
	cd react && npm run smoke

wasm-test: check-wasm-pack
	# Run the wasm-bindgen browser tests.
	cd rust && wasm-pack test --headless --chrome

release-check: check-wasm-pack
	# Verify the crate packages cleanly and the demo can produce an npm tarball.
	cd rust && cargo package --allow-dirty
	cd react && npm pack --dry-run

ci: test build smoke
	# Run the checks mirrored by CI.
