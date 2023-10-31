mod utils;

use wasm_bindgen::prelude::*;
use image::{open, ImageFormat};
use web_sys::console;
use js_sys::Error;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(path: &str) -> Result<(), JsValue> {
        // Open the image
        let img = match open(path) {
            Ok(image) => image,
            Err(err) => {
                console::error_1(&format!("Error opening image: {}", err).into());
                return Err(Error::new(&format!("Error opening image: {}", err)).into());
            }
        };
    
        // Convert to grayscale
        let grayscale_img = img.grayscale();
    
        // Save the grayscale image
        let output_path = "output.png"; // Change the output path as needed
        match grayscale_img.save_with_format(output_path, ImageFormat::Png) {
            Ok(_) => Ok(()),
            Err(err) => {
                console::error_1(&format!("Error saving image: {}", err.to_string()).into());
                Err(Error::new(&format!("Error saving image: {}", err.to_string())).into())
            }
        }
    // alert(&format!("Hello, {}!", path));
}