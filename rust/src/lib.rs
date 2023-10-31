mod utils;

use std::borrow::Borrow;

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
pub fn greet(bytes: Vec<u8>) -> Vec<u8> {
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str(format!("converting from bytes: {:?}", bytes).as_str()));
  
    let img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png);

    match img {
        Ok(loaded_img) => {
            console::log_1(&JsValue::from_str(format!("loaded image: {:?}", loaded_img).as_str()));
            loaded_img.to_luma8().into_vec()
        },
        Err(err) => {
            console::error_1(&JsValue::from_str(format!("converting from bytes: {:?}", err).as_str()));
            Vec::new()
        }
    }
    // alert(&format!("Hello, {}!", path));
}