# Troubleshooting

## `wasm-pack` is missing

Run:

```bash
make install-wasm-pack
```

## The demo build fails before webpack starts

The React build invokes `wasm-pack` first. Confirm:

- `rustup target add wasm32-unknown-unknown`
- `wasm-pack --version`
- `cargo test` succeeds inside `rust/`

## Browser wasm tests fail locally

`make wasm-test` uses headless Chrome. Install a Chromium-compatible browser or run the same check in CI where the browser is provisioned.

## The bundler cannot find the generated wasm asset

Make sure the consuming app:

- awaits the package default initializer
- preserves the emitted `.wasm` asset from the generated `pkg/` directory
- uses a bundler/runtime that supports wasm asset loading

## The canvas helpers throw dimension or buffer errors

Resim now validates image dimensions and buffer length explicitly. A valid image must have:

- non-zero width and height
- `width * height * 4` bytes in the RGBA buffer
