mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

pub fn get_image_data_from_pixels(mut raw_pixels: Vec<u8>, width: u32, height: u32) -> ImageData {
    if width <= 0 {
        console::error_1(&JsValue::from_str("Invalid image width"));
        return ImageData::new_with_sw(1, 1).unwrap();
    }

    let image_data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height);
    
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

pub fn get_image_data_from_canvas(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> ImageData {
    let width = canvas.width();
    let height = canvas.height();

    let data = ctx
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap();
    data
}

pub fn to_raw_pixels(imgdata: ImageData) -> Vec<u8> {
    imgdata.data().to_vec()
}

pub fn grayscale(mut img: Vec<u8>) -> Vec<u8> {
    let end = img.len();

    for i in (0..end).step_by(4) {
        let r_val = img[i] as u32;
        let g_val = img[i + 1] as u32;
        let b_val = img[i + 2] as u32;
        let mut avg: u32 = (r_val + g_val + b_val) / 3;
        if avg >= 255 {
            avg = 255
        }

        img[i] = avg as u8;
        img[i + 1] = avg as u8;
        img[i + 2] = avg as u8;
    }
    img
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn convertToGrayscale(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> ImageData {
    let imgdata = get_image_data_from_canvas(&canvas, &ctx);
    let raw_pixels = to_raw_pixels(imgdata);
    let gray_raw_pixels = grayscale(raw_pixels);
    get_image_data_from_pixels(gray_raw_pixels, canvas.width(), canvas.height())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn placeImage(
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    new_image: ImageData,
) {
    let mut raw_pixels = to_raw_pixels(new_image);
    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut raw_pixels),
        canvas.width(),
        canvas.height(),
    );

    ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0)
        .expect("place image on canvas");
}
