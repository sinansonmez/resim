# Changelog

All notable changes to this project will be documented in this file.

## [0.0.8-alpha] - 2026-03-10

### Added

- Added a pure Rust image-processing core for grayscale, invert, and blur transforms.
- Added wasm-specific bindings that expose `ImageData` and canvas helper APIs for browser consumers.
- Added baseline Rust tests for transform behavior and dispatch.
- Added a root `Makefile` with documented local development commands.
- Added `PLAN.md` to capture the current project direction and near-term priorities.

### Changed

- Reworked the React demo into a fuller showcase with transform selection, apply/reset controls, status messaging, and responsive layout behavior.
- Switched the local wasm generation flow to `wasm-pack --target web` and updated the frontend integration to initialize the generated package explicitly.
- Updated Rust wasm dependencies to current compatible versions.
- Updated the npm publish workflow to use current GitHub Actions versions and a modern Rust setup path.
- Refreshed top-level, Rust, and React documentation to match the current architecture and development flow.
- Bumped the Rust package version from `0.0.7-alpha` to `0.0.8-alpha`.
- Bumped the React demo version from `0.1.0` to `0.1.1`.

### Removed

- Removed the old browser API shape centered on direct canvas-bound functions like `convertToGrayscale` and `placeImage` as the primary documented interface.
