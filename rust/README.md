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
- brightness
- contrast
- threshold

## Local workflow

Run tests from this directory once Rust is installed:

```bash
cargo test
```

Generate the wasm package for the demo app:

```bash
wasm-pack build . --target web --out-dir pkg --release
```

## Intended API style

The public browser-facing API stays `ImageData`-first:

```javascript
import init, { contrastImageData } from "@sinansonmez/resim";

await init();
const next = contrastImageData(imageData, 25);
```

The wasm package also exposes `getTransformCatalog()` so browser consumers can inspect
the current transform metadata and build UI around the supported operations.
