mod core;
mod utils;
#[cfg(feature = "enable_wasm")]
mod wasm;

pub use core::{apply_transform, blur, grayscale, invert_colors, Transform};
#[cfg(feature = "enable_wasm")]
pub use wasm::{
    applyTransformToCanvas, blurImageData, grayscaleImageData, invertImageData,
    readCanvasImageData, writeCanvasImageData, TransformKind,
};
