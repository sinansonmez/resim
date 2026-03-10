# React demo

This app is the showcase consumer for the generated Resim wasm package.
It demonstrates the current transform catalog, including parameterized controls for brightness,
contrast, and threshold. The transform list is loaded from the wasm package's
`getTransformCatalog()` export so the demo reflects the public browser API more directly.

## Local workflow

```bash
npm install
npm run build
npm start
```

`npm run build` and `npm start` both generate the wasm package first by calling `wasm-pack build ../rust --target web --out-dir pkg --release`.
