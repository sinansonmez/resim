# Rust package

This crate contains both the pure image transform logic and the wasm bindings exposed to browser consumers.

## Structure

- `src/core.rs` implements raw RGBA pixel transforms.
- `src/wasm.rs` exposes the browser-facing API with `wasm-bindgen`.
- `src/lib.rs` re-exports the core functions and wasm entrypoints.

## Current transforms

- grayscale
- invert
- blur

## Local workflow

Run tests from this directory once Rust is installed:

```bash
cargo test
```

Generate the wasm package for the demo app:

```bash
wasm-pack build . --target web --out-dir pkg --release
```
