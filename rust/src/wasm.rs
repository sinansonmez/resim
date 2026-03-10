use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::core::{apply_transform, Transform};
use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub enum TransformKind {
    Grayscale,
    Invert,
    Blur,
}

impl From<TransformKind> for Transform {
    fn from(value: TransformKind) -> Self {
        match value {
            TransformKind::Grayscale => Transform::Grayscale,
            TransformKind::Invert => Transform::Invert,
            TransformKind::Blur => Transform::Blur,
        }
    }
}

fn image_data_from_pixels(mut pixels: Vec<u8>, width: u32, height: u32) -> Result<ImageData, JsValue> {
    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut pixels), width, height)
}

fn transform_image_data(image_data: ImageData, transform: Transform) -> Result<ImageData, JsValue> {
    let width = image_data.width();
    let height = image_data.height();
    let pixels = image_data.data().to_vec();
    let next_pixels = apply_transform(&pixels, width, height, transform);
    image_data_from_pixels(next_pixels, width, height)
}

#[wasm_bindgen(js_name = readCanvasImageData)]
#[allow(non_snake_case)]
pub fn readCanvasImageData(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
) -> Result<ImageData, JsValue> {
    set_panic_hook();
    ctx.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
}

#[wasm_bindgen(js_name = writeCanvasImageData)]
#[allow(non_snake_case)]
pub fn writeCanvasImageData(
    ctx: &CanvasRenderingContext2d,
    image_data: &ImageData,
) -> Result<(), JsValue> {
    set_panic_hook();
    ctx.put_image_data(image_data, 0.0, 0.0)
}

#[wasm_bindgen(js_name = grayscaleImageData)]
#[allow(non_snake_case)]
pub fn grayscaleImageData(image_data: ImageData) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Grayscale)
}

#[wasm_bindgen(js_name = invertImageData)]
#[allow(non_snake_case)]
pub fn invertImageData(image_data: ImageData) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Invert)
}

#[wasm_bindgen(js_name = blurImageData)]
#[allow(non_snake_case)]
pub fn blurImageData(image_data: ImageData) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Blur)
}

#[wasm_bindgen(js_name = applyTransformToCanvas)]
#[allow(non_snake_case)]
pub fn applyTransformToCanvas(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
    transform: TransformKind,
) -> Result<ImageData, JsValue> {
    set_panic_hook();
    let image_data = readCanvasImageData(canvas, ctx)?;
    let next_image = transform_image_data(image_data, transform.into())?;
    writeCanvasImageData(ctx, &next_image)?;
    Ok(next_image)
}
