# Contributing

## Local setup

1. Install Rust stable and confirm `cargo` works.
2. Install `wasm-pack`.
3. Install the demo dependencies with `make install`.
4. Run `make test` and `make build` before opening a change.

## Development workflow

- Rust core work lives in `rust/src/core.rs`.
- Browser bindings live in `rust/src/wasm.rs`.
- The React demo should consume the public wasm API rather than reimplementing transform metadata.
- Prefer adding or updating tests when the public API or transform behavior changes.

## Validation expectations

Run the checks relevant to the change:

- `make test` for Rust unit coverage
- `make build` for the demo and wasm bundle
- `make smoke` for demo helper/state smoke coverage
- `make wasm-test` when browser-facing wasm exports change
- `make release-check` when package metadata or publish flow changes

## Release expectations

- crate and demo versions should stay aligned
- public API changes must be reflected in `README.md`, `rust/README.md`, and `react/README.md`
- aliases should be preferred over abrupt removals when renaming browser exports
- release-oriented changes should update `CHANGELOG.md`
