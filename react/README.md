# React demo

This app is the showcase consumer for the generated Resim wasm package.

## What it covers

- transform catalog loading through `getTransformCatalog()`
- parameterized controls for brightness, contrast, saturation, opacity, gamma, and threshold
- local upload, reset, undo, comparison, and PNG export flows

## Local workflow

```bash
npm install
npm run build
npm run smoke
npm start
```

`npm run build` and `npm start` both generate the wasm package first by calling `wasm-pack build ../rust --target web --out-dir pkg --release`.
