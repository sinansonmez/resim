use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use js_sys::{Array, Object, Reflect};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::core::{apply_transform, Transform};
use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub enum TransformKind {
    Grayscale,
    Invert,
    Blur,
    Brightness,
    Contrast,
    Threshold,
}

impl From<TransformKind> for Transform {
    fn from(value: TransformKind) -> Self {
        match value {
            TransformKind::Grayscale => Transform::Grayscale,
            TransformKind::Invert => Transform::Invert,
            TransformKind::Blur => Transform::Blur,
            TransformKind::Brightness => Transform::Brightness(0),
            TransformKind::Contrast => Transform::Contrast(0.0),
            TransformKind::Threshold => Transform::Threshold(128),
        }
    }
}

fn set_object_property(object: &Object, key: &str, value: JsValue) -> Result<(), JsValue> {
    Reflect::set(object, &JsValue::from_str(key), &value).map(|_| ())
}

fn transform_metadata_entry(
    id: &str,
    label: &str,
    description: &str,
    method: &str,
    control: Option<Object>,
) -> Result<Object, JsValue> {
    let entry = Object::new();
    set_object_property(&entry, "id", JsValue::from_str(id))?;
    set_object_property(&entry, "label", JsValue::from_str(label))?;
    set_object_property(&entry, "description", JsValue::from_str(description))?;
    set_object_property(&entry, "method", JsValue::from_str(method))?;

    if let Some(control) = control {
        set_object_property(&entry, "control", control.into())?;
    }

    Ok(entry)
}

fn control_metadata(
    name: &str,
    label: &str,
    min: f64,
    max: f64,
    step: f64,
    default_value: f64,
    display: &str,
) -> Result<Object, JsValue> {
    let control = Object::new();
    set_object_property(&control, "name", JsValue::from_str(name))?;
    set_object_property(&control, "label", JsValue::from_str(label))?;
    set_object_property(&control, "min", JsValue::from_f64(min))?;
    set_object_property(&control, "max", JsValue::from_f64(max))?;
    set_object_property(&control, "step", JsValue::from_f64(step))?;
    set_object_property(&control, "defaultValue", JsValue::from_f64(default_value))?;
    set_object_property(&control, "display", JsValue::from_str(display))?;
    Ok(control)
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

#[wasm_bindgen(js_name = getTransformCatalog)]
#[allow(non_snake_case)]
pub fn getTransformCatalog() -> Result<Array, JsValue> {
    set_panic_hook();

    let catalog = Array::new();
    catalog.push(&transform_metadata_entry(
        "grayscale",
        "Grayscale",
        "Average RGB channels for a classic monochrome output.",
        "grayscaleImageData",
        None,
    )?.into());
    catalog.push(&transform_metadata_entry(
        "invert",
        "Invert",
        "Flip every color channel while preserving transparency.",
        "invertImageData",
        None,
    )?.into());
    catalog.push(&transform_metadata_entry(
        "blur",
        "Blur",
        "Apply a simple 3x3 box blur to soften the image.",
        "blurImageData",
        None,
    )?.into());
    catalog.push(&transform_metadata_entry(
        "brightness",
        "Brightness",
        "Lift or darken every RGB channel with one intensity control.",
        "brightnessImageData",
        Some(control_metadata("amount", "Amount", -255.0, 255.0, 1.0, 30.0, "signed")?),
    )?.into());
    catalog.push(&transform_metadata_entry(
        "contrast",
        "Contrast",
        "Push shadows and highlights apart while preserving the midpoint.",
        "contrastImageData",
        Some(control_metadata("amount", "Contrast", -100.0, 100.0, 1.0, 25.0, "signedPercent")?),
    )?.into());
    catalog.push(&transform_metadata_entry(
        "threshold",
        "Threshold",
        "Convert the image to pure black and white at a selected cutoff.",
        "thresholdImageData",
        Some(control_metadata("cutoff", "Cutoff", 0.0, 255.0, 1.0, 128.0, "plain")?),
    )?.into());

    Ok(catalog)
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

#[wasm_bindgen(js_name = brightnessImageData)]
#[allow(non_snake_case)]
pub fn brightnessImageData(image_data: ImageData, amount: i16) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Brightness(amount))
}

#[wasm_bindgen(js_name = contrastImageData)]
#[allow(non_snake_case)]
pub fn contrastImageData(image_data: ImageData, amount: f32) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Contrast(amount))
}

#[wasm_bindgen(js_name = thresholdImageData)]
#[allow(non_snake_case)]
pub fn thresholdImageData(image_data: ImageData, cutoff: u8) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Threshold(cutoff))
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
