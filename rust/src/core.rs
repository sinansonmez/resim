#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transform {
    Grayscale,
    Invert,
    Blur,
    Brightness(i16),
    Contrast(f32),
    Threshold(u8),
}

pub fn apply_transform(pixels: &[u8], width: u32, height: u32, transform: Transform) -> Vec<u8> {
    match transform {
        Transform::Grayscale => grayscale(pixels),
        Transform::Invert => invert_colors(pixels),
        Transform::Blur => blur(pixels, width, height),
        Transform::Brightness(amount) => adjust_brightness(pixels, amount),
        Transform::Contrast(amount) => adjust_contrast(pixels, amount),
        Transform::Threshold(amount) => threshold(pixels, amount),
    }
}

fn clamp_channel(value: f32) -> u8 {
    value.clamp(0.0, 255.0).round() as u8
}

pub fn grayscale(pixels: &[u8]) -> Vec<u8> {
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(4) {
        let average = ((next[idx] as u32 + next[idx + 1] as u32 + next[idx + 2] as u32) / 3) as u8;
        next[idx] = average;
        next[idx + 1] = average;
        next[idx + 2] = average;
    }

    next
}

pub fn invert_colors(pixels: &[u8]) -> Vec<u8> {
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(4) {
        next[idx] = 255 - next[idx];
        next[idx + 1] = 255 - next[idx + 1];
        next[idx + 2] = 255 - next[idx + 2];
    }

    next
}

pub fn blur(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    if width < 3 || height < 3 {
        return pixels.to_vec();
    }

    let mut next = pixels.to_vec();

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let pixel_index = (((y as i32 + dy) as u32 * width) + (x as i32 + dx) as u32) * 4;
                    red += pixels[pixel_index as usize] as u32;
                    green += pixels[pixel_index as usize + 1] as u32;
                    blue += pixels[pixel_index as usize + 2] as u32;
                }
            }

            let pixel_index = ((y * width) + x) * 4;
            next[pixel_index as usize] = (red / 9) as u8;
            next[pixel_index as usize + 1] = (green / 9) as u8;
            next[pixel_index as usize + 2] = (blue / 9) as u8;
        }
    }

    next
}

pub fn adjust_brightness(pixels: &[u8], amount: i16) -> Vec<u8> {
    let amount = amount.clamp(-255, 255) as f32;
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(4) {
        next[idx] = clamp_channel(next[idx] as f32 + amount);
        next[idx + 1] = clamp_channel(next[idx + 1] as f32 + amount);
        next[idx + 2] = clamp_channel(next[idx + 2] as f32 + amount);
    }

    next
}

pub fn adjust_contrast(pixels: &[u8], amount: f32) -> Vec<u8> {
    let amount = amount.clamp(-100.0, 100.0);
    let factor = (259.0 * (amount + 255.0)) / (255.0 * (259.0 - amount));
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(4) {
        next[idx] = clamp_channel(factor * (next[idx] as f32 - 128.0) + 128.0);
        next[idx + 1] = clamp_channel(factor * (next[idx + 1] as f32 - 128.0) + 128.0);
        next[idx + 2] = clamp_channel(factor * (next[idx + 2] as f32 - 128.0) + 128.0);
    }

    next
}

pub fn threshold(pixels: &[u8], cutoff: u8) -> Vec<u8> {
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(4) {
        let average = ((next[idx] as u32 + next[idx + 1] as u32 + next[idx + 2] as u32) / 3) as u8;
        let value = if average >= cutoff { 255 } else { 0 };
        next[idx] = value;
        next[idx + 1] = value;
        next[idx + 2] = value;
    }

    next
}

#[cfg(test)]
mod tests {
    use super::{
        adjust_brightness, adjust_contrast, apply_transform, blur, grayscale, invert_colors,
        threshold, Transform,
    };

    #[test]
    fn grayscale_averages_each_pixel_without_touching_alpha() {
        let pixels = vec![30, 60, 90, 255, 0, 30, 60, 111];
        let transformed = grayscale(&pixels);

        assert_eq!(transformed, vec![60, 60, 60, 255, 30, 30, 30, 111]);
    }

    #[test]
    fn invert_flips_rgb_channels_without_touching_alpha() {
        let pixels = vec![0, 64, 255, 200, 10, 20, 30, 40];
        let transformed = invert_colors(&pixels);

        assert_eq!(transformed, vec![255, 191, 0, 200, 245, 235, 225, 40]);
    }

    #[test]
    fn blur_averages_the_center_pixel_and_keeps_edges_stable() {
        let pixels = vec![
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 255,
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
        ];

        let transformed = blur(&pixels, 3, 3);

        assert_eq!(
            transformed,
            vec![
                0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
                0, 0, 0, 255, 28, 28, 28, 255, 0, 0, 0, 255,
                0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            ]
        );
    }

    #[test]
    fn blur_returns_original_for_small_images() {
        let pixels = vec![12, 34, 56, 255, 78, 90, 12, 255];

        assert_eq!(blur(&pixels, 2, 1), pixels);
    }

    #[test]
    fn apply_transform_dispatches_to_the_requested_operation() {
        let pixels = vec![10, 20, 30, 255];

        assert_eq!(
            apply_transform(&pixels, 1, 1, Transform::Grayscale),
            vec![20, 20, 20, 255]
        );
    }

    #[test]
    fn brightness_adjusts_channels_and_preserves_alpha() {
        let pixels = vec![10, 20, 30, 255, 240, 250, 255, 20];

        assert_eq!(
            adjust_brightness(&pixels, 20),
            vec![30, 40, 50, 255, 255, 255, 255, 20]
        );
        assert_eq!(
            adjust_brightness(&pixels, -25),
            vec![0, 0, 5, 255, 215, 225, 230, 20]
        );
    }

    #[test]
    fn contrast_handles_low_mid_and_high_values() {
        let pixels = vec![64, 128, 192, 255];

        assert_eq!(adjust_contrast(&pixels, 0.0), pixels);
        assert_eq!(adjust_contrast(&pixels, 100.0), vec![0, 128, 255, 255]);
    }

    #[test]
    fn threshold_turns_pixels_black_or_white() {
        let pixels = vec![10, 20, 30, 255, 160, 170, 180, 255];

        assert_eq!(threshold(&pixels, 100), vec![0, 0, 0, 255, 255, 255, 255, 255]);
        assert_eq!(threshold(&pixels, 0), vec![255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(threshold(&pixels, 255), vec![0, 0, 0, 255, 0, 0, 0, 255]);
    }

    #[test]
    fn apply_transform_dispatches_parameterized_transforms() {
        let pixels = vec![100, 110, 120, 255];

        assert_eq!(
            apply_transform(&pixels, 1, 1, Transform::Brightness(10)),
            vec![110, 120, 130, 255]
        );
        assert_eq!(
            apply_transform(&pixels, 1, 1, Transform::Threshold(110)),
            vec![255, 255, 255, 255]
        );
    }
}
