mod utils;

use base64::{engine::general_purpose, Engine};
use image::DynamicImage;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, ImageData};

pub fn to_image_data(mut raw_pixels: Vec<u8>, width: u32) -> ImageData {
    if width <= 0 {
        console::error_1(&JsValue::from_str("Invalid image dimensions"));
        return ImageData::new_with_sw(1, 1).unwrap();
    }

    let image_data = ImageData::new_with_u8_clamped_array(Clamped(&mut raw_pixels), width);
    match image_data {
        Ok(image_data_success) => image_data_success,
        Err(image_data_err) => {
            console::error_1(&JsValue::from_str(
                format!("image_data_err {:?}", image_data_err).as_str(),
            ));
            ImageData::new_with_sw(width, width).unwrap()
        }
    }
}

pub fn base64_to_vec(base64: &str) -> Vec<u8> {
    let base = general_purpose::STANDARD.decode(base64);

    match base {
        Ok(base_image) => base_image,
        Err(base_err) => {
            console::error_1(&JsValue::from_str(
                format!("error when converting base64 to Vector {:?}", base_err).as_str(),
            ));
            Vec::new()
        }
    }
}

fn vec_to_image(vec: Vec<u8>) -> DynamicImage {
    let slice = vec.as_slice();

    let img = image::load_from_memory(slice).unwrap();
    let grayscale_img = img.grayscale();
    DynamicImage::ImageRgb8(grayscale_img.to_rgb8())
}

#[wasm_bindgen]
pub fn grayscale(base64: &str) -> ImageData {
    let base64_to_vec: Vec<u8> = base64_to_vec(base64);
    let img = vec_to_image(base64_to_vec);
    // todo why width of img is not correct
    to_image_data(img.as_bytes().to_vec(), 600)
}
