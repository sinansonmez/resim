# React demo

This app is the showcase consumer for the generated Resim wasm package.

## Local workflow

```bash
npm install
npm run build
npm start
```

`npm run build` and `npm start` both generate the wasm package first by calling `wasm-pack build ../rust --target web --out-dir pkg --release`.
