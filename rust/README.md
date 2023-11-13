# Resim - Image Manipulation written in Rust for Javascript world

Resim is an npm package that allows you to perform various image manipulations on your images using Rust, a fast and efficient programming language. Currently, Resim supports converting png images to grayscale, and it aims to expand its feature set in the future.

Disclaimer: Resim is still at a very early phase of development, so don't use it for production.

P.S.: Resim means 'image' in Turkish

## Features

- Convert images to grayscale (function name: `convertToGrayscale`)
- Convert images to have inverted colors (function name: `invertColors`)

## Installation

You can install Resim using npm:

```javascript
npm install resim
```

## Usage
```javascript
import * as resim from "resim";

var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");

ctx.drawImage(document.getElementById('image'), 0, 0);
const imageResim = resim.convertToGrayscale(canvas, ctx)
resim.placeImage(canvas, ctx, imageResim);
```

## Example
Take a look at sample implementation [here](https://github.com/sinansonmez/resim/blob/main/react/index.jsx)