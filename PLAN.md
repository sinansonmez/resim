# Resim Long-Term Improvement Plan

Use this file as a living checklist for moving Resim from an early wasm showcase to a more complete browser image toolkit.

## Product direction

- [ ] Define the public positioning of Resim: browser image toolkit, demo showcase, or publishable npm package.
- [ ] Decide the minimum stable API surface for a first non-alpha release.
- [ ] Document officially supported environments and bundler expectations.
- [ ] Remove stale product messaging from docs whenever project direction changes.

## Core library

- [x] Split pure pixel-processing logic from wasm/browser bindings.
- [x] Support baseline transforms: grayscale, invert, blur, brightness, contrast, threshold.
- [ ] Add a transform composition API for applying multiple operations in sequence.
- [ ] Add more practical transforms such as sharpen, saturation, sepia, opacity, and gamma.
- [ ] Decide whether spatial operations like resize, crop, and rotate belong in scope.
- [ ] Standardize parameter handling and bounds across all transforms.
- [ ] Add explicit error handling for invalid dimensions, malformed buffers, and unsupported parameter values.
- [ ] Review performance hotspots and optimize expensive transforms where needed.

## JavaScript and wasm API

- [x] Keep the primary JS API `ImageData`-first.
- [ ] Audit exported wasm function names for consistency and long-term stability.
- [x] Add a documented transform catalog or metadata export that consumers can inspect programmatically.
- [ ] Decide whether `applyTransformToCanvas` remains a convenience helper or should be replaced by a more generic helper layer.
- [ ] Add usage examples for parameterized transforms and chained workflows.
- [ ] Define versioning and deprecation rules for public API changes.

## Demo application

- [x] Provide a demo with transform selection and preview/reset controls.
- [x] Add parameter controls for brightness, contrast, and threshold.
- [x] Add local file upload so users can test their own images.
- [x] Add side-by-side original vs processed comparison mode.
- [x] Add export/download for processed output.
- [x] Add transform history or undo/reset-to-last-step behavior.
- [x] Improve mobile layout and control ergonomics for parameter-heavy workflows.
- [x] Keep demo behavior aligned with the documented public API instead of adding one-off demo-only logic.

## Testing and quality

- [x] Add baseline Rust unit tests for core transforms.
- [ ] Add more edge-case tests for parameter limits and tiny image sizes.
- [ ] Add wasm integration tests that verify exported functions and initialization flow.
- [ ] Add demo-level smoke validation for key user flows.
- [ ] Add CI checks for Rust tests and frontend build verification.
- [ ] Add release checks so published packages match documented usage examples.

## Docs and developer experience

- [x] Add a root `Makefile` for common development commands.
- [x] Add a `CHANGELOG.md`.
- [ ] Keep `README.md` aligned with the actual repo state after each feature change.
- [ ] Add a contributor guide for local setup, wasm build flow, and release expectations.
- [ ] Add architecture notes explaining the split between core Rust logic and wasm bindings.
- [ ] Add examples showing how to consume the package in a minimal browser app.
- [ ] Add troubleshooting notes for common local issues like `wasm-pack`, Rust target setup, and bundler integration.

## Release and maintenance

- [x] Modernize the npm publish workflow.
- [ ] Add a license field to the Rust package metadata.
- [ ] Decide the release process for alpha, beta, and stable versions.
- [ ] Define semantic versioning rules across the Rust crate and npm package surface.
- [ ] Add automated checks that prevent broken publish artifacts.
- [ ] Periodically review outdated dependencies in Rust and React tooling.
