mod utils;

use base64::{engine::general_purpose, Engine};
use image::{io::Reader, DynamicImage, ImageFormat};
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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

pub fn dyn_image_from_raw(image: Vec<u8>) -> DynamicImage {
    // convert a vec of raw pixels (as u8s) to a DynamicImage type
    let img_buffer = image::ImageBuffer::from_vec(600, 800, image).unwrap();
    DynamicImage::ImageRgba8(img_buffer)
}

fn create_image_from_bytes(data: Vec<u8>) -> DynamicImage {
    // Create a Cursor from the Vec<u8>
    let cursor = Cursor::new(data);

    // Attempt to read the image from the cursor
    let dynamic_image = Reader::with_format(cursor, ImageFormat::Png).decode();

    match dynamic_image {
        Ok(dynamic_image_success) => {
            console::error_1(&JsValue::from_str(
                format!("dynamic_image_success: {:?}", dynamic_image_success).as_str(),
            ));
            dynamic_image_success
        }
        Err(dynamic_image_err) => {
            console::error_1(&JsValue::from_str(
                format!("dynamic_image_err: {:?}", dynamic_image_err).as_str(),
            ));
            DynamicImage::new_luma8(32, 32)
        }
    }
}

fn image_to_base64(image: DynamicImage, format: ImageFormat) -> String {
    // Create a temporary file
    let mut temp_file = File::create("temp_image.png").unwrap();

    // Write the image data to the temporary file
    image.write_to(&mut temp_file, format);

    // Read the image data from the temporary file
    let mut temp_file = File::open("temp_image.png").unwrap();
    let mut buffer = Vec::new();
    temp_file.read_to_end(&mut buffer);

    // Encode the image data to base64
    let base64_encoded = base64::encode(&buffer);

    base64_encoded
}

/// Convert the PhotonImage to base64.
pub fn get_base64(image: Vec<u8>) -> String {
    let img = create_image_from_bytes(image);

    let base64 = image_to_base64(img, image::ImageFormat::Png);
    let res_base64 = format!("data:image/png;base64,{}", base64.replace("\r\n", ""));
    res_base64
}

#[wasm_bindgen]
pub fn greet(base64: &str) -> String {
    let base64_to_vec: Vec<u8> = base64_to_vec(base64);

    let slice = base64_to_vec.as_slice();

    let mut img = image::load_from_memory(slice).unwrap();
    img = DynamicImage::ImageLuma8(img.to_luma8());
    get_base64(img.into_bytes())
}
