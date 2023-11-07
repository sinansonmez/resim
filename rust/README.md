# Resim - Image Manipulation written in Rust for Javascript worl

Resim is an npm package that allows you to perform various image manipulations on your images using Rust, a fast and efficient programming language. Currently, Resim supports converting png images to grayscale, and it aims to expand its feature set in the future.

Disclaimer: Resim is still at a very early phase of development, so don't use it for production.

P.S.: Resim means image in Turkish

## Features

- Convert images to grayscale.

## Installation

You can install Resim using npm:

```bash
npm install resim

## Usage
```JavaScript
import * as resim from "resim";

resim.grayscale([BASE64 OF AN IMAGE]) // it will return an [ImageData](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)
