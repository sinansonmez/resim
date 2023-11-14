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

pub fn get_image_data_from_canvas(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
) -> ImageData {
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

pub fn invert_colors(mut img: Vec<u8>) -> Vec<u8> {
    let end = img.len();

    for i in (0..end).step_by(4) {
        img[i] = 255 - img[i];
        img[i + 1] = 255 - img[i + 1];
        img[i + 2] = 255 - img[i + 2];
    }
    img
}

pub fn blur(img: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut new_img = img.clone();

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;

            // Sum the color values of the current pixel and its surrounding pixels
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let pixel_index = ((y as i32 + dy) as u32 * width + (x as i32 + dx) as u32) * 4;
                    r += img[pixel_index as usize] as u32;
                    g += img[pixel_index as usize + 1] as u32;
                    b += img[pixel_index as usize + 2] as u32;
                }
            }

            // Calculate the average color value
            let pixel_index = (y * width + x) * 4;
            new_img[pixel_index as usize] = (r / 9) as u8;
            new_img[pixel_index as usize + 1] = (g / 9) as u8;
            new_img[pixel_index as usize + 2] = (b / 9) as u8;
        }
    }

    new_img
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
pub fn placeImage(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d, new_image: ImageData) {
    let mut raw_pixels = to_raw_pixels(new_image);
    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut raw_pixels),
        canvas.width(),
        canvas.height(),
    );

    ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0)
        .expect("place image on canvas");
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn invertColors(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> ImageData {
    let imgdata = get_image_data_from_canvas(&canvas, &ctx);
    let raw_pixels = to_raw_pixels(imgdata);
    let inverted_raw_pixels = invert_colors(raw_pixels);
    get_image_data_from_pixels(inverted_raw_pixels, canvas.width(), canvas.height())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn blurImage(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> ImageData {
    let imgdata = get_image_data_from_canvas(&canvas, &ctx);
    let raw_pixels = to_raw_pixels(imgdata);
    let blurred_raw_pixels = blur(raw_pixels, canvas.width(), canvas.height());
    get_image_data_from_pixels(blurred_raw_pixels, canvas.width(), canvas.height())
}
