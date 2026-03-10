# Resim Plan

Resim is a browser-first image processing showcase built with Rust, WebAssembly, and a small React demo.

## Current target

- Keep the library scope intentionally narrow: grayscale, invert, and blur.
- Make Rust the source of truth for image transforms.
- Keep the React app as an example consumer of the wasm package, not the main product.

## Implementation priorities

1. Separate pure pixel operations from browser-specific wasm bindings.
2. Expose a small JavaScript-facing API around `ImageData` and canvas helpers.
3. Ship a demo that lets users preview, apply, and reset transforms on a sample image.
4. Document the local workflow for Rust, `wasm-pack`, and the demo app.
5. Add baseline tests for transform correctness before expanding the feature set.

## Near-term backlog

- Add more deterministic tests for edge cases and invalid dimensions.
- Add drag-and-drop or file upload support in the demo.
- Evaluate publishing once the API and build flow are stable.
