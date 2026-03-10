use std::error::Error;
use std::fmt;

pub const PIXEL_STRIDE: usize = 4;
pub const BRIGHTNESS_MIN: i16 = -255;
pub const BRIGHTNESS_MAX: i16 = 255;
pub const CONTRAST_MIN: f32 = -100.0;
pub const CONTRAST_MAX: f32 = 100.0;
pub const SATURATION_MIN: f32 = -100.0;
pub const SATURATION_MAX: f32 = 100.0;
pub const OPACITY_MIN: f32 = 0.0;
pub const OPACITY_MAX: f32 = 100.0;
pub const GAMMA_MIN: f32 = 0.1;
pub const GAMMA_MAX: f32 = 5.0;
const EXPOSURE_TO_BRIGHTNESS: f32 = 85.0;
const FILTER_STRENGTH_SCALE: f32 = 10.0;

#[derive(Clone, Debug, PartialEq)]
pub enum ResimError {
    InvalidDimensions { width: u32, height: u32 },
    BufferLengthMismatch { expected: usize, actual: usize },
    UnsupportedParameter {
        transform: &'static str,
        min: f32,
        max: f32,
        received: f32,
    },
}

impl fmt::Display for ResimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResimError::InvalidDimensions { width, height } => {
                write!(f, "invalid image dimensions: {}x{}", width, height)
            }
            ResimError::BufferLengthMismatch { expected, actual } => {
                write!(
                    f,
                    "pixel buffer length mismatch: expected {} bytes, received {} bytes",
                    expected, actual
                )
            }
            ResimError::UnsupportedParameter {
                transform,
                min,
                max,
                received,
            } => write!(
                f,
                "unsupported {} parameter: expected {}..={}, received {}",
                transform, min, max, received
            ),
        }
    }
}

impl Error for ResimError {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transform {
    Grayscale,
    Invert,
    Blur,
    Brightness(i16),
    Contrast(f32),
    Threshold(u8),
    Sharpen,
    Saturation(f32),
    Sepia,
    Opacity(f32),
    Gamma(f32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresetKind {
    CleanBrightEveryday,
    WarmSoftLifestyle,
    MoodyStreet,
    CrispColorPop,
    FilmPortrait,
    VintageFadedFeed,
    MinimalBlackAndWhite,
    SoftCoolEditorial,
    SelfiePortrait,
    BeachTravel,
    NightOut,
}

pub fn apply_transform(
    pixels: &[u8],
    width: u32,
    height: u32,
    transform: Transform,
) -> Result<Vec<u8>, ResimError> {
    validate_image(pixels, width, height)?;
    validate_transform(transform)?;
    apply_transform_unchecked(pixels, width, height, transform)
}

pub fn apply_transform_sequence(
    pixels: &[u8],
    width: u32,
    height: u32,
    transforms: &[Transform],
) -> Result<Vec<u8>, ResimError> {
    validate_image(pixels, width, height)?;

    let mut next_pixels = pixels.to_vec();
    for transform in transforms {
        validate_transform(*transform)?;
        next_pixels = apply_transform_unchecked(&next_pixels, width, height, *transform)?;
    }

    Ok(next_pixels)
}

pub fn resize_image(
    pixels: &[u8],
    width: u32,
    height: u32,
    target_width: u32,
    target_height: u32,
) -> Result<Vec<u8>, ResimError> {
    validate_image(pixels, width, height)?;

    if target_width == 0 || target_height == 0 {
        return Err(ResimError::InvalidDimensions {
            width: target_width,
            height: target_height,
        });
    }

    let mut next = vec![0; target_width as usize * target_height as usize * PIXEL_STRIDE];

    for y in 0..target_height {
        let source_y = y * height / target_height;
        for x in 0..target_width {
            let source_x = x * width / target_width;
            let source_index = ((source_y * width + source_x) as usize) * PIXEL_STRIDE;
            let target_index = ((y * target_width + x) as usize) * PIXEL_STRIDE;
            next[target_index..target_index + PIXEL_STRIDE]
                .copy_from_slice(&pixels[source_index..source_index + PIXEL_STRIDE]);
        }
    }

    Ok(next)
}

pub fn apply_preset(
    pixels: &[u8],
    width: u32,
    height: u32,
    preset: PresetKind,
) -> Result<Vec<u8>, ResimError> {
    validate_image(pixels, width, height)?;

    let (strength, settings) = match preset {
        PresetKind::CleanBrightEveryday => (
            7.2,
            &[
                PresetSetting::Exposure(0.3),
                PresetSetting::Contrast(-0.5),
                PresetSetting::Saturation(-0.5),
                PresetSetting::Temperature(-0.5),
                PresetSetting::Sharpen(1.0),
                PresetSetting::Grain(1.0),
            ][..],
        ),
        PresetKind::WarmSoftLifestyle => (
            6.2,
            &[
                PresetSetting::Exposure(0.5),
                PresetSetting::Contrast(-1.0),
                PresetSetting::Saturation(-0.5),
                PresetSetting::Temperature(0.8),
                PresetSetting::Tint(0.3),
                PresetSetting::Fade(1.0),
                PresetSetting::Grain(1.0),
            ][..],
        ),
        PresetKind::MoodyStreet => (
            7.1,
            &[
                PresetSetting::Exposure(-0.7),
                PresetSetting::Contrast(1.0),
                PresetSetting::Saturation(-1.0),
                PresetSetting::Temperature(-0.5),
                PresetSetting::Shadows(1.0),
                PresetSetting::Grain(2.0),
            ][..],
        ),
        PresetKind::CrispColorPop => (
            5.0,
            &[
                PresetSetting::Exposure(0.2),
                PresetSetting::Contrast(0.8),
                PresetSetting::Saturation(0.5),
                PresetSetting::Temperature(-0.3),
                PresetSetting::Sharpen(1.0),
                PresetSetting::Clarity(1.0),
            ][..],
        ),
        PresetKind::FilmPortrait => (
            6.0,
            &[
                PresetSetting::Exposure(0.3),
                PresetSetting::Contrast(-0.5),
                PresetSetting::Saturation(-0.8),
                PresetSetting::Temperature(0.5),
                PresetSetting::Fade(1.0),
                PresetSetting::Grain(2.0),
            ][..],
        ),
        PresetKind::VintageFadedFeed => (
            5.0,
            &[
                PresetSetting::Exposure(0.4),
                PresetSetting::Contrast(-1.2),
                PresetSetting::Saturation(-1.0),
                PresetSetting::Temperature(0.5),
                PresetSetting::Fade(2.0),
                PresetSetting::Grain(2.0),
                PresetSetting::Vignette(0.5),
            ][..],
        ),
        PresetKind::MinimalBlackAndWhite => (
            8.0,
            &[
                PresetSetting::BlackAndWhite,
                PresetSetting::Contrast(1.0),
                PresetSetting::Exposure(0.2),
                PresetSetting::Fade(1.0),
                PresetSetting::Grain(1.0),
                PresetSetting::Sharpen(1.0),
            ][..],
        ),
        PresetKind::SoftCoolEditorial => (
            5.0,
            &[
                PresetSetting::Exposure(0.2),
                PresetSetting::Contrast(-0.5),
                PresetSetting::Saturation(-1.0),
                PresetSetting::Temperature(-0.8),
                PresetSetting::Fade(1.0),
                PresetSetting::Grain(1.0),
            ][..],
        ),
        PresetKind::SelfiePortrait => (
            6.5,
            &[
                PresetSetting::Exposure(0.4),
                PresetSetting::Contrast(-0.8),
                PresetSetting::Saturation(-0.5),
                PresetSetting::Temperature(0.7),
                PresetSetting::Fade(1.0),
                PresetSetting::Grain(1.0),
            ][..],
        ),
        PresetKind::BeachTravel => (
            5.0,
            &[
                PresetSetting::Exposure(0.3),
                PresetSetting::Contrast(0.5),
                PresetSetting::Saturation(0.3),
                PresetSetting::Temperature(-0.4),
                PresetSetting::Sharpen(1.0),
            ][..],
        ),
        PresetKind::NightOut => (
            7.0,
            &[
                PresetSetting::Exposure(-0.6),
                PresetSetting::Contrast(1.0),
                PresetSetting::Saturation(-1.0),
                PresetSetting::Shadows(1.0),
                PresetSetting::Grain(2.0),
            ][..],
        ),
    };

    let mut next = pixels.to_vec();
    for setting in settings {
        next = apply_preset_setting(&next, width, height, *setting, strength);
    }

    Ok(next)
}

#[derive(Clone, Copy)]
enum PresetSetting {
    Exposure(f32),
    Contrast(f32),
    Saturation(f32),
    Temperature(f32),
    Tint(f32),
    Sharpen(f32),
    Grain(f32),
    Fade(f32),
    Shadows(f32),
    Clarity(f32),
    Vignette(f32),
    BlackAndWhite,
}

fn scale_setting(value: f32, strength: f32) -> f32 {
    value * (strength / FILTER_STRENGTH_SCALE)
}

fn apply_preset_setting(
    pixels: &[u8],
    width: u32,
    height: u32,
    setting: PresetSetting,
    strength: f32,
) -> Vec<u8> {
    match setting {
        PresetSetting::Exposure(amount) => {
            adjust_brightness_float(pixels, scale_setting(amount, strength) * EXPOSURE_TO_BRIGHTNESS)
        }
        PresetSetting::Contrast(amount) => adjust_contrast(pixels, scale_setting(amount, strength) * 18.0),
        PresetSetting::Saturation(amount) => {
            adjust_saturation(pixels, scale_setting(amount, strength) * 20.0)
        }
        PresetSetting::Temperature(amount) => adjust_temperature(pixels, scale_setting(amount, strength)),
        PresetSetting::Tint(amount) => adjust_tint(pixels, scale_setting(amount, strength)),
        PresetSetting::Sharpen(amount) => {
            if scale_setting(amount, strength) >= 0.5 {
                sharpen(pixels, width, height)
            } else {
                pixels.to_vec()
            }
        }
        PresetSetting::Grain(amount) => add_grain(pixels, scale_setting(amount, strength)),
        PresetSetting::Fade(amount) => apply_fade(pixels, scale_setting(amount, strength)),
        PresetSetting::Shadows(amount) => lift_shadows(pixels, scale_setting(amount, strength)),
        PresetSetting::Clarity(amount) => apply_clarity(pixels, scale_setting(amount, strength)),
        PresetSetting::Vignette(amount) => apply_vignette(pixels, width, height, scale_setting(amount, strength)),
        PresetSetting::BlackAndWhite => grayscale(pixels),
    }
}

pub fn validate_image(pixels: &[u8], width: u32, height: u32) -> Result<(), ResimError> {
    if width == 0 || height == 0 {
        return Err(ResimError::InvalidDimensions { width, height });
    }

    let expected_len = width
        .checked_mul(height)
        .and_then(|pixel_count| pixel_count.checked_mul(PIXEL_STRIDE as u32))
        .ok_or(ResimError::InvalidDimensions { width, height })? as usize;

    if pixels.len() != expected_len {
        return Err(ResimError::BufferLengthMismatch {
            expected: expected_len,
            actual: pixels.len(),
        });
    }

    Ok(())
}

fn validate_transform(transform: Transform) -> Result<(), ResimError> {
    match transform {
        Transform::Brightness(amount) => {
            validate_range("brightness", amount as f32, BRIGHTNESS_MIN as f32, BRIGHTNESS_MAX as f32)
        }
        Transform::Contrast(amount) => validate_range("contrast", amount, CONTRAST_MIN, CONTRAST_MAX),
        Transform::Saturation(amount) => {
            validate_range("saturation", amount, SATURATION_MIN, SATURATION_MAX)
        }
        Transform::Opacity(amount) => validate_range("opacity", amount, OPACITY_MIN, OPACITY_MAX),
        Transform::Gamma(amount) => validate_range("gamma", amount, GAMMA_MIN, GAMMA_MAX),
        _ => Ok(()),
    }
}

fn validate_range(
    transform: &'static str,
    received: f32,
    min: f32,
    max: f32,
) -> Result<(), ResimError> {
    if received < min || received > max {
        return Err(ResimError::UnsupportedParameter {
            transform,
            min,
            max,
            received,
        });
    }

    Ok(())
}

fn apply_transform_unchecked(
    pixels: &[u8],
    width: u32,
    height: u32,
    transform: Transform,
) -> Result<Vec<u8>, ResimError> {
    let next_pixels = match transform {
        Transform::Grayscale => grayscale(pixels),
        Transform::Invert => invert_colors(pixels),
        Transform::Blur => blur(pixels, width, height),
        Transform::Brightness(amount) => adjust_brightness(pixels, amount),
        Transform::Contrast(amount) => adjust_contrast(pixels, amount),
        Transform::Threshold(amount) => threshold(pixels, amount),
        Transform::Sharpen => sharpen(pixels, width, height),
        Transform::Saturation(amount) => adjust_saturation(pixels, amount),
        Transform::Sepia => sepia(pixels),
        Transform::Opacity(amount) => adjust_opacity(pixels, amount),
        Transform::Gamma(amount) => adjust_gamma(pixels, amount),
    };

    Ok(next_pixels)
}

fn clamp_channel(value: f32) -> u8 {
    value.clamp(0.0, 255.0).round() as u8
}

pub fn grayscale(pixels: &[u8]) -> Vec<u8> {
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        let average = ((next[idx] as u32 + next[idx + 1] as u32 + next[idx + 2] as u32) / 3) as u8;
        next[idx] = average;
        next[idx + 1] = average;
        next[idx + 2] = average;
    }

    next
}

pub fn invert_colors(pixels: &[u8]) -> Vec<u8> {
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx] = 255 - next[idx];
        next[idx + 1] = 255 - next[idx + 1];
        next[idx + 2] = 255 - next[idx + 2];
    }

    next
}

pub fn blur(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    apply_kernel(
        pixels,
        width,
        height,
        &[1, 1, 1, 1, 1, 1, 1, 1, 1],
        9.0,
        0.0,
    )
}

pub fn sharpen(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    apply_kernel(
        pixels,
        width,
        height,
        &[0, -1, 0, -1, 5, -1, 0, -1, 0],
        1.0,
        0.0,
    )
}

fn apply_kernel(
    pixels: &[u8],
    width: u32,
    height: u32,
    kernel: &[i32; 9],
    divisor: f32,
    bias: f32,
) -> Vec<u8> {
    if width < 3 || height < 3 {
        return pixels.to_vec();
    }

    let mut next = pixels.to_vec();

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut red = 0.0;
            let mut green = 0.0;
            let mut blue = 0.0;

            for (kernel_idx, weight) in kernel.iter().enumerate() {
                let dx = (kernel_idx % 3) as i32 - 1;
                let dy = (kernel_idx / 3) as i32 - 1;
                let pixel_index = (((y as i32 + dy) as u32 * width) + (x as i32 + dx) as u32)
                    as usize
                    * PIXEL_STRIDE;

                red += pixels[pixel_index] as f32 * *weight as f32;
                green += pixels[pixel_index + 1] as f32 * *weight as f32;
                blue += pixels[pixel_index + 2] as f32 * *weight as f32;
            }

            let pixel_index = ((y * width) + x) as usize * PIXEL_STRIDE;
            next[pixel_index] = clamp_channel(red / divisor + bias);
            next[pixel_index + 1] = clamp_channel(green / divisor + bias);
            next[pixel_index + 2] = clamp_channel(blue / divisor + bias);
        }
    }

    next
}

pub fn adjust_brightness(pixels: &[u8], amount: i16) -> Vec<u8> {
    adjust_brightness_float(pixels, amount as f32)
}

fn adjust_brightness_float(pixels: &[u8], amount: f32) -> Vec<u8> {
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx] = clamp_channel(next[idx] as f32 + amount);
        next[idx + 1] = clamp_channel(next[idx + 1] as f32 + amount);
        next[idx + 2] = clamp_channel(next[idx + 2] as f32 + amount);
    }

    next
}

pub fn adjust_contrast(pixels: &[u8], amount: f32) -> Vec<u8> {
    let factor = (259.0 * (amount + 255.0)) / (255.0 * (259.0 - amount));
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx] = clamp_channel(factor * (next[idx] as f32 - 128.0) + 128.0);
        next[idx + 1] = clamp_channel(factor * (next[idx + 1] as f32 - 128.0) + 128.0);
        next[idx + 2] = clamp_channel(factor * (next[idx + 2] as f32 - 128.0) + 128.0);
    }

    next
}

pub fn adjust_saturation(pixels: &[u8], amount: f32) -> Vec<u8> {
    let factor = 1.0 + amount / 100.0;
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        let red = next[idx] as f32;
        let green = next[idx + 1] as f32;
        let blue = next[idx + 2] as f32;
        let gray = 0.3086 * red + 0.6094 * green + 0.0820 * blue;

        next[idx] = clamp_channel(gray + factor * (red - gray));
        next[idx + 1] = clamp_channel(gray + factor * (green - gray));
        next[idx + 2] = clamp_channel(gray + factor * (blue - gray));
    }

    next
}

pub fn adjust_temperature(pixels: &[u8], amount: f32) -> Vec<u8> {
    let mut next = pixels.to_vec();
    let red_shift = amount * 22.0;
    let blue_shift = amount * -22.0;

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx] = clamp_channel(next[idx] as f32 + red_shift);
        next[idx + 2] = clamp_channel(next[idx + 2] as f32 + blue_shift);
    }

    next
}

pub fn adjust_tint(pixels: &[u8], amount: f32) -> Vec<u8> {
    let mut next = pixels.to_vec();
    let green_shift = amount * -14.0;
    let red_blue_shift = amount * 10.0;

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx] = clamp_channel(next[idx] as f32 + red_blue_shift);
        next[idx + 1] = clamp_channel(next[idx + 1] as f32 + green_shift);
        next[idx + 2] = clamp_channel(next[idx + 2] as f32 + red_blue_shift);
    }

    next
}

pub fn apply_fade(pixels: &[u8], amount: f32) -> Vec<u8> {
    let mut next = pixels.to_vec();
    let lift = amount * 12.0;
    let preserve = (1.0 - amount * 0.08).max(0.0);

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx] = clamp_channel(next[idx] as f32 * preserve + lift);
        next[idx + 1] = clamp_channel(next[idx + 1] as f32 * preserve + lift);
        next[idx + 2] = clamp_channel(next[idx + 2] as f32 * preserve + lift);
    }

    next
}

pub fn add_grain(pixels: &[u8], amount: f32) -> Vec<u8> {
    let mut next = pixels.to_vec();
    let intensity = amount * 8.0;

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        let pixel_index = (idx / PIXEL_STRIDE) as i32;
        let noise = (((pixel_index * 73) % 29) - 14) as f32 * intensity * 0.15;
        next[idx] = clamp_channel(next[idx] as f32 + noise);
        next[idx + 1] = clamp_channel(next[idx + 1] as f32 + noise);
        next[idx + 2] = clamp_channel(next[idx + 2] as f32 + noise);
    }

    next
}

pub fn lift_shadows(pixels: &[u8], amount: f32) -> Vec<u8> {
    let mut next = pixels.to_vec();
    let lift = amount * 32.0;

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        for channel in 0..3 {
            let value = next[idx + channel] as f32;
            let shadow_weight = 1.0 - value / 255.0;
            next[idx + channel] = clamp_channel(value + lift * shadow_weight);
        }
    }

    next
}

pub fn apply_clarity(pixels: &[u8], amount: f32) -> Vec<u8> {
    let mut next = pixels.to_vec();
    let factor = 1.0 + amount * 0.22;

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx] = clamp_channel((next[idx] as f32 - 128.0) * factor + 128.0);
        next[idx + 1] = clamp_channel((next[idx + 1] as f32 - 128.0) * factor + 128.0);
        next[idx + 2] = clamp_channel((next[idx + 2] as f32 - 128.0) * factor + 128.0);
    }

    next
}

pub fn apply_vignette(pixels: &[u8], width: u32, height: u32, amount: f32) -> Vec<u8> {
    let mut next = pixels.to_vec();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_distance = (center_x * center_x + center_y * center_y).sqrt().max(1.0);

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) as usize) * PIXEL_STRIDE;
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let normalized = (dx * dx + dy * dy).sqrt() / max_distance;
            let falloff = 1.0 - amount * normalized * normalized * 0.75;
            next[idx] = clamp_channel(next[idx] as f32 * falloff);
            next[idx + 1] = clamp_channel(next[idx + 1] as f32 * falloff);
            next[idx + 2] = clamp_channel(next[idx + 2] as f32 * falloff);
        }
    }

    next
}

pub fn sepia(pixels: &[u8]) -> Vec<u8> {
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        let red = next[idx] as f32;
        let green = next[idx + 1] as f32;
        let blue = next[idx + 2] as f32;

        next[idx] = clamp_channel(0.393 * red + 0.769 * green + 0.189 * blue);
        next[idx + 1] = clamp_channel(0.349 * red + 0.686 * green + 0.168 * blue);
        next[idx + 2] = clamp_channel(0.272 * red + 0.534 * green + 0.131 * blue);
    }

    next
}

pub fn adjust_opacity(pixels: &[u8], amount: f32) -> Vec<u8> {
    let factor = amount / 100.0;
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx + 3] = clamp_channel(next[idx + 3] as f32 * factor);
    }

    next
}

pub fn adjust_gamma(pixels: &[u8], amount: f32) -> Vec<u8> {
    let inverse = 1.0 / amount;
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
        next[idx] = clamp_channel(255.0 * (next[idx] as f32 / 255.0).powf(inverse));
        next[idx + 1] = clamp_channel(255.0 * (next[idx + 1] as f32 / 255.0).powf(inverse));
        next[idx + 2] = clamp_channel(255.0 * (next[idx + 2] as f32 / 255.0).powf(inverse));
    }

    next
}

pub fn threshold(pixels: &[u8], cutoff: u8) -> Vec<u8> {
    let mut next = pixels.to_vec();

    for idx in (0..next.len()).step_by(PIXEL_STRIDE) {
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
        adjust_brightness, adjust_contrast, adjust_gamma, adjust_opacity, adjust_saturation,
        apply_preset, apply_transform, apply_transform_sequence, blur, grayscale, invert_colors,
        resize_image, sepia, sharpen, threshold, validate_image, PresetKind, ResimError,
        Transform, BRIGHTNESS_MAX, CONTRAST_MAX, GAMMA_MIN,
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
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255, 255, 0, 0,
            0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
        ];

        let transformed = blur(&pixels, 3, 3);

        assert_eq!(
            transformed,
            vec![
                0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 28, 28, 28, 255, 0, 0,
                0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            ]
        );
    }

    #[test]
    fn sharpen_increases_center_contrast() {
        let pixels = vec![
            10, 10, 10, 255, 10, 10, 10, 255, 10, 10, 10, 255, 10, 10, 10, 255, 80, 80, 80,
            255, 10, 10, 10, 255, 10, 10, 10, 255, 10, 10, 10, 255, 10, 10, 10, 255,
        ];

        let transformed = sharpen(&pixels, 3, 3);

        assert_eq!(transformed[16], 255);
        assert_eq!(transformed[17], 255);
        assert_eq!(transformed[18], 255);
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
            apply_transform(&pixels, 1, 1, Transform::Grayscale).unwrap(),
            vec![20, 20, 20, 255]
        );
    }

    #[test]
    fn apply_transform_sequence_runs_multiple_operations_in_order() {
        let pixels = vec![10, 20, 30, 255];
        let sequence = [Transform::Brightness(20), Transform::Invert];

        assert_eq!(
            apply_transform_sequence(&pixels, 1, 1, &sequence).unwrap(),
            vec![225, 215, 205, 255]
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
    fn saturation_allows_desaturation_and_boost() {
        let pixels = vec![200, 100, 50, 255];

        assert_eq!(adjust_saturation(&pixels, -100.0), vec![127, 127, 127, 255]);
        assert!(adjust_saturation(&pixels, 100.0)[0] >= pixels[0]);
    }

    #[test]
    fn sepia_recolors_rgb_without_touching_alpha() {
        let pixels = vec![100, 80, 60, 200];

        assert_eq!(sepia(&pixels), vec![112, 100, 78, 200]);
    }

    #[test]
    fn opacity_scales_alpha_channel() {
        let pixels = vec![10, 20, 30, 200];

        assert_eq!(adjust_opacity(&pixels, 50.0), vec![10, 20, 30, 100]);
    }

    #[test]
    fn gamma_adjusts_brightness_curve() {
        let pixels = vec![64, 128, 192, 255];

        assert_eq!(adjust_gamma(&pixels, 1.0), pixels);
        assert!(adjust_gamma(&pixels, 2.0)[0] > pixels[0]);
    }

    #[test]
    fn threshold_turns_pixels_black_or_white() {
        let pixels = vec![10, 20, 30, 255, 160, 170, 180, 255];

        assert_eq!(threshold(&pixels, 100), vec![0, 0, 0, 255, 255, 255, 255, 255]);
        assert_eq!(threshold(&pixels, 0), vec![255, 255, 255, 255, 255, 255, 255, 255]);
        assert_eq!(threshold(&pixels, 255), vec![0, 0, 0, 255, 0, 0, 0, 255]);
    }

    #[test]
    fn validate_image_rejects_invalid_dimensions() {
        assert_eq!(
            validate_image(&[], 0, 1),
            Err(ResimError::InvalidDimensions { width: 0, height: 1 })
        );
    }

    #[test]
    fn validate_image_rejects_malformed_buffers() {
        assert_eq!(
            validate_image(&[1, 2, 3], 1, 1),
            Err(ResimError::BufferLengthMismatch {
                expected: 4,
                actual: 3,
            })
        );
    }

    #[test]
    fn apply_transform_rejects_out_of_range_parameters() {
        let pixels = vec![10, 20, 30, 255];

        assert_eq!(
            apply_transform(&pixels, 1, 1, Transform::Brightness(BRIGHTNESS_MAX + 1)),
            Err(ResimError::UnsupportedParameter {
                transform: "brightness",
                min: -255.0,
                max: 255.0,
                received: 256.0,
            })
        );
        assert_eq!(
            apply_transform(&pixels, 1, 1, Transform::Contrast(CONTRAST_MAX + 1.0)),
            Err(ResimError::UnsupportedParameter {
                transform: "contrast",
                min: -100.0,
                max: 100.0,
                received: 101.0,
            })
        );
        assert_eq!(
            apply_transform(&pixels, 1, 1, Transform::Gamma(GAMMA_MIN / 2.0)),
            Err(ResimError::UnsupportedParameter {
                transform: "gamma",
                min: 0.1,
                max: 5.0,
                received: 0.05,
            })
        );
    }

    #[test]
    fn resize_image_scales_to_new_dimensions() {
        let pixels = vec![
            10, 0, 0, 255, 20, 0, 0, 255,
            30, 0, 0, 255, 40, 0, 0, 255,
        ];

        assert_eq!(
            resize_image(&pixels, 2, 2, 1, 1).unwrap(),
            vec![10, 0, 0, 255]
        );
        assert_eq!(
            resize_image(&pixels, 2, 2, 4, 1).unwrap(),
            vec![
                10, 0, 0, 255, 10, 0, 0, 255, 20, 0, 0, 255, 20, 0, 0, 255,
            ]
        );
    }

    #[test]
    fn apply_preset_modifies_pixels_deterministically() {
        let pixels = vec![120, 90, 60, 255];

        let transformed = apply_preset(&pixels, 1, 1, PresetKind::WarmSoftLifestyle).unwrap();

        assert_ne!(transformed, pixels);
        assert_eq!(transformed[3], 255);
    }
}
