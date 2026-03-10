# Resim

Resim is a browser-first image processing showcase built with Rust and WebAssembly.
The repository contains a small Rust library for pixel transforms and a React demo that
consumes the generated wasm package.

Resim is still early-stage. The current focus is a clean core API, accurate docs, and a
working local demo rather than wide feature coverage.

## Current features

- `grayscaleImageData`
- `invertImageData`
- `blurImageData`
- `brightnessImageData`
- `contrastImageData`
- `thresholdImageData`
- `readCanvasImageData`
- `writeCanvasImageData`

The React demo currently supports:

- selecting built-in transforms and adjusting parameters
- uploading a local image and restoring the bundled sample
- side-by-side comparison of original vs processed output
- undoing the last applied transform step
- downloading the processed canvas as a PNG
- driving its transform controls from the wasm-exported catalog metadata

The Rust core keeps transform logic separate from browser bindings so the pixel-processing
functions can be tested without DOM types.

## Repo layout

- `rust/` contains the library source and wasm bindings.
- `react/` contains the showcase app that imports the generated package.
- `CHANGELOG.md` tracks shipped changes.

## Local development

Prerequisites:

- Rust toolchain with `cargo`
- `wasm-pack`
- Node.js with npm

Build the wasm package and demo bundle:

```bash
cd react
npm install
npm run build
```

Start the demo locally:

```bash
cd react
npm start
```

The React scripts call `wasm-pack` first, which generates `rust/pkg` with the web target for the demo to import and initialize.

## Intended API shape

```javascript
import {
  default as init,
  readCanvasImageData,
  brightnessImageData,
  writeCanvasImageData,
} from "@sinansonmez/resim";

await init();
const imageData = readCanvasImageData(canvas, ctx);
const transformed = brightnessImageData(imageData, 30);
writeCanvasImageData(ctx, transformed);
```
