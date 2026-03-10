# Rust package

This crate contains the validated pixel-processing core and the wasm bindings exposed to browser consumers.

## Current transforms

- grayscale
- invert
- blur
- brightness
- contrast
- threshold
- sharpen
- saturation
- sepia
- opacity
- gamma
- resize

## Core API

Important Rust-side entry points:

- `apply_transform`
- `apply_transform_sequence`
- `validate_image`
- `Transform`
- `ResimError`

The core now rejects malformed buffers, zero dimensions, and out-of-range parameter values instead of silently clamping every invalid call.

## Browser API

The wasm package exposes:

- `getTransformCatalog()`
- `readImageDataFromCanvas()` and `writeImageDataToCanvas()`
- backward-compatible aliases `readCanvasImageData()` and `writeCanvasImageData()`
- individual `*ImageData` transform functions
- `resizeImageData()`
- `applyCanvasTransform()` and the older alias `applyTransformToCanvas()`

## Local workflow

```bash
cargo test
wasm-pack build . --target web --out-dir pkg --release
wasm-pack test --headless --chrome
```

## Example

```javascript
import init, {
  contrastImageData,
  readImageDataFromCanvas,
  writeImageDataToCanvas,
} from "@sinansonmez/resim";

await init();
const imageData = readImageDataFromCanvas(canvas, ctx);
const next = contrastImageData(imageData, 25);
writeImageDataToCanvas(ctx, next);
```
