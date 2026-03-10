mod core;
mod utils;
#[cfg(feature = "enable_wasm")]
mod wasm;

pub use core::{
    adjust_brightness, adjust_contrast, apply_transform, blur, grayscale, invert_colors,
    threshold, Transform,
};
#[cfg(feature = "enable_wasm")]
pub use wasm::{
    applyTransformToCanvas, blurImageData, brightnessImageData, contrastImageData,
    getTransformCatalog, grayscaleImageData, invertImageData, readCanvasImageData,
    thresholdImageData, writeCanvasImageData, TransformKind,
};
