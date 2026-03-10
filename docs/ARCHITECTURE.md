# Architecture

## Split of responsibilities

- `rust/src/core.rs` owns transform validation, parameter bounds, transform composition, and pixel math.
- `rust/src/wasm.rs` converts browser `ImageData` values to Rust buffers, maps core errors to `JsValue`, and exposes the browser API with `wasm-bindgen`.
- `react/index.jsx` is only a consumer of the wasm package. It should not duplicate the authoritative transform catalog.

## Data flow

1. The browser loads the generated wasm package and awaits its default initializer.
2. The demo or consuming app reads `ImageData` from a canvas or another DOM source.
3. wasm bindings validate the input shape and delegate to the Rust core.
4. The core applies one transform or a sequence of transforms and returns a fresh RGBA buffer.
5. The browser layer wraps that buffer in `ImageData` and writes it back to the canvas if needed.

## API shape

- The primary API is `ImageData`-first.
- Canvas-bound helpers remain convenience wrappers.
- `getTransformCatalog()` is the metadata source for browser UIs.
- The Rust core keeps `apply_transform_sequence` available for validated multi-step composition.
