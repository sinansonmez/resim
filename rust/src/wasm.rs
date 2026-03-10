use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

use crate::core::{
    apply_preset, apply_transform, apply_transform_sequence, resize_image, validate_image,
    BRIGHTNESS_MAX, BRIGHTNESS_MIN, CONTRAST_MAX, CONTRAST_MIN, GAMMA_MAX, GAMMA_MIN,
    OPACITY_MAX, OPACITY_MIN, PresetKind, SATURATION_MAX, SATURATION_MIN, Transform,
};
use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub enum TransformKind {
    Grayscale,
    Invert,
    Blur,
    Brightness,
    Contrast,
    Threshold,
    Sharpen,
    Saturation,
    Sepia,
    Opacity,
    Gamma,
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
            TransformKind::Sharpen => Transform::Sharpen,
            TransformKind::Saturation => Transform::Saturation(0.0),
            TransformKind::Sepia => Transform::Sepia,
            TransformKind::Opacity => Transform::Opacity(100.0),
            TransformKind::Gamma => Transform::Gamma(1.0),
        }
    }
}

fn js_error(message: impl Into<String>) -> JsValue {
    JsValue::from_str(&message.into())
}

fn preset_kind_from_id(preset_id: &str) -> Result<PresetKind, JsValue> {
    match preset_id {
        "clean-bright-everyday" => Ok(PresetKind::CleanBrightEveryday),
        "warm-soft-lifestyle" => Ok(PresetKind::WarmSoftLifestyle),
        "moody-street" => Ok(PresetKind::MoodyStreet),
        "crisp-color-pop" => Ok(PresetKind::CrispColorPop),
        "film-portrait" => Ok(PresetKind::FilmPortrait),
        "vintage-faded-feed" => Ok(PresetKind::VintageFadedFeed),
        "minimal-black-and-white" => Ok(PresetKind::MinimalBlackAndWhite),
        "soft-cool-editorial" => Ok(PresetKind::SoftCoolEditorial),
        "selfie-portrait" => Ok(PresetKind::SelfiePortrait),
        "beach-travel" => Ok(PresetKind::BeachTravel),
        "night-out" => Ok(PresetKind::NightOut),
        _ => Err(js_error(format!("unsupported preset id: {}", preset_id))),
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

fn read_image_data_pixels(image_data: &ImageData) -> Result<(Vec<u8>, u32, u32), JsValue> {
    let width = image_data.width();
    let height = image_data.height();
    let pixels = image_data.data().to_vec();
    validate_image(&pixels, width, height).map_err(|error| js_error(error.to_string()))?;
    Ok((pixels, width, height))
}

fn transform_image_data(image_data: ImageData, transform: Transform) -> Result<ImageData, JsValue> {
    let (pixels, width, height) = read_image_data_pixels(&image_data)?;
    let next_pixels =
        apply_transform(&pixels, width, height, transform).map_err(|error| js_error(error.to_string()))?;
    image_data_from_pixels(next_pixels, width, height)
}

#[wasm_bindgen(js_name = getTransformCatalog)]
#[allow(non_snake_case)]
pub fn getTransformCatalog() -> Result<Array, JsValue> {
    set_panic_hook();

    let catalog = Array::new();
    catalog.push(
        &transform_metadata_entry(
            "grayscale",
            "Grayscale",
            "Average RGB channels for a classic monochrome output.",
            "grayscaleImageData",
            None,
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "invert",
            "Invert",
            "Flip every color channel while preserving transparency.",
            "invertImageData",
            None,
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "blur",
            "Blur",
            "Apply a simple 3x3 box blur to soften the image.",
            "blurImageData",
            None,
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "brightness",
            "Brightness",
            "Lift or darken every RGB channel with one intensity control.",
            "brightnessImageData",
            Some(control_metadata(
                "amount",
                "Amount",
                BRIGHTNESS_MIN as f64,
                BRIGHTNESS_MAX as f64,
                1.0,
                30.0,
                "signed",
            )?),
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "contrast",
            "Contrast",
            "Push shadows and highlights apart while preserving the midpoint.",
            "contrastImageData",
            Some(control_metadata(
                "amount",
                "Contrast",
                CONTRAST_MIN as f64,
                CONTRAST_MAX as f64,
                1.0,
                25.0,
                "signedPercent",
            )?),
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "threshold",
            "Threshold",
            "Convert the image to pure black and white at a selected cutoff.",
            "thresholdImageData",
            Some(control_metadata("cutoff", "Cutoff", 0.0, 255.0, 1.0, 128.0, "plain")?),
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "sharpen",
            "Sharpen",
            "Increase edge contrast with a simple 3x3 sharpening kernel.",
            "sharpenImageData",
            None,
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "saturation",
            "Saturation",
            "Reduce or boost color intensity while preserving transparency.",
            "saturationImageData",
            Some(control_metadata(
                "amount",
                "Saturation",
                SATURATION_MIN as f64,
                SATURATION_MAX as f64,
                1.0,
                15.0,
                "signedPercent",
            )?),
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "sepia",
            "Sepia",
            "Apply a warm vintage toning effect to the RGB channels.",
            "sepiaImageData",
            None,
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "opacity",
            "Opacity",
            "Scale the alpha channel without changing RGB values.",
            "opacityImageData",
            Some(control_metadata(
                "amount",
                "Opacity",
                OPACITY_MIN as f64,
                OPACITY_MAX as f64,
                1.0,
                100.0,
                "percent",
            )?),
        )?
        .into(),
    );
    catalog.push(
        &transform_metadata_entry(
            "gamma",
            "Gamma",
            "Remap the brightness curve with gamma correction.",
            "gammaImageData",
            Some(control_metadata(
                "amount",
                "Gamma",
                GAMMA_MIN as f64,
                GAMMA_MAX as f64,
                0.1,
                1.2,
                "float",
            )?),
        )?
        .into(),
    );

    Ok(catalog)
}

#[wasm_bindgen(js_name = readCanvasImageData)]
#[allow(non_snake_case)]
pub fn readCanvasImageData(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
) -> Result<ImageData, JsValue> {
    readImageDataFromCanvas(canvas, ctx)
}

#[wasm_bindgen(js_name = readImageDataFromCanvas)]
#[allow(non_snake_case)]
pub fn readImageDataFromCanvas(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
) -> Result<ImageData, JsValue> {
    set_panic_hook();
    let image_data = ctx.get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)?;
    let _ = read_image_data_pixels(&image_data)?;
    Ok(image_data)
}

#[wasm_bindgen(js_name = writeCanvasImageData)]
#[allow(non_snake_case)]
pub fn writeCanvasImageData(
    ctx: &CanvasRenderingContext2d,
    image_data: &ImageData,
) -> Result<(), JsValue> {
    writeImageDataToCanvas(ctx, image_data)
}

#[wasm_bindgen(js_name = writeImageDataToCanvas)]
#[allow(non_snake_case)]
pub fn writeImageDataToCanvas(
    ctx: &CanvasRenderingContext2d,
    image_data: &ImageData,
) -> Result<(), JsValue> {
    set_panic_hook();
    let _ = read_image_data_pixels(image_data)?;
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

#[wasm_bindgen(js_name = sharpenImageData)]
#[allow(non_snake_case)]
pub fn sharpenImageData(image_data: ImageData) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Sharpen)
}

#[wasm_bindgen(js_name = saturationImageData)]
#[allow(non_snake_case)]
pub fn saturationImageData(image_data: ImageData, amount: f32) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Saturation(amount))
}

#[wasm_bindgen(js_name = sepiaImageData)]
#[allow(non_snake_case)]
pub fn sepiaImageData(image_data: ImageData) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Sepia)
}

#[wasm_bindgen(js_name = opacityImageData)]
#[allow(non_snake_case)]
pub fn opacityImageData(image_data: ImageData, amount: f32) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Opacity(amount))
}

#[wasm_bindgen(js_name = gammaImageData)]
#[allow(non_snake_case)]
pub fn gammaImageData(image_data: ImageData, amount: f32) -> Result<ImageData, JsValue> {
    set_panic_hook();
    transform_image_data(image_data, Transform::Gamma(amount))
}

#[wasm_bindgen(js_name = resizeImageData)]
#[allow(non_snake_case)]
pub fn resizeImageData(
    image_data: ImageData,
    width: u32,
    height: u32,
) -> Result<ImageData, JsValue> {
    set_panic_hook();
    let (pixels, source_width, source_height) = read_image_data_pixels(&image_data)?;
    let next_pixels = resize_image(&pixels, source_width, source_height, width, height)
        .map_err(|error| js_error(error.to_string()))?;
    image_data_from_pixels(next_pixels, width, height)
}

#[wasm_bindgen(js_name = applyPresetImageData)]
#[allow(non_snake_case)]
pub fn applyPresetImageData(image_data: ImageData, preset_id: String) -> Result<ImageData, JsValue> {
    set_panic_hook();
    let preset = preset_kind_from_id(&preset_id)?;
    let (pixels, width, height) = read_image_data_pixels(&image_data)?;
    let next_pixels =
        apply_preset(&pixels, width, height, preset).map_err(|error| js_error(error.to_string()))?;
    image_data_from_pixels(next_pixels, width, height)
}

#[wasm_bindgen(js_name = composeImageDataTransforms)]
#[allow(non_snake_case)]
pub fn composeImageDataTransforms(
    image_data: ImageData,
    transforms: Vec<TransformKind>,
) -> Result<ImageData, JsValue> {
    set_panic_hook();
    let (pixels, width, height) = read_image_data_pixels(&image_data)?;
    let transform_values: Vec<Transform> = transforms.into_iter().map(Transform::from).collect();
    let next_pixels = apply_transform_sequence(&pixels, width, height, &transform_values)
        .map_err(|error| js_error(error.to_string()))?;
    image_data_from_pixels(next_pixels, width, height)
}

#[wasm_bindgen(js_name = applyTransformToCanvas)]
#[allow(non_snake_case)]
pub fn applyTransformToCanvas(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
    transform: TransformKind,
) -> Result<ImageData, JsValue> {
    applyCanvasTransform(canvas, ctx, transform)
}

#[wasm_bindgen(js_name = applyCanvasTransform)]
#[allow(non_snake_case)]
pub fn applyCanvasTransform(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
    transform: TransformKind,
) -> Result<ImageData, JsValue> {
    set_panic_hook();
    let image_data = readImageDataFromCanvas(canvas, ctx)?;
    let next_image = transform_image_data(image_data, transform.into())?;
    writeImageDataToCanvas(ctx, &next_image)?;
    Ok(next_image)
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::{brightnessImageData, getTransformCatalog};
    use js_sys::Reflect;
    use wasm_bindgen::Clamped;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    use web_sys::ImageData;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn catalog_includes_new_transforms() {
        let catalog = getTransformCatalog().unwrap();
        assert!(catalog.length() >= 11);

        let first = catalog.get(0);
        assert_eq!(
            Reflect::get(&first, &JsValue::from_str("id")).unwrap().as_string(),
            Some("grayscale".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn brightness_export_transforms_image_data() {
        let mut pixels = vec![10, 20, 30, 255];
        let image_data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut pixels), 1, 1).unwrap();
        let next = brightnessImageData(image_data, 10).unwrap();
        let data = next.data().to_vec();

        assert_eq!(data, vec![20, 30, 40, 255]);
    }
}
