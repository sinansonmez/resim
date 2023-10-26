mod utils;

use wasm_bindgen::prelude::*;
use image::io::Reader as ImageReader;

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
pub fn greet(path: &str) {
    let img = ImageReader::open(path)?.decode()?;
    alert(&format!("Hello, {}!", path));
}