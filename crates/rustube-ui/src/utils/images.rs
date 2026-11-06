//! Thumbnail / avatar image prep.

use image::{DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;

/// Crop an image to 16:9 and round the corners (JPEG/PNG/WebP in, PNG out).
/// `radius` is in pixels, relative to the source image size.
pub fn rounded(bytes: &[u8], radius: u32) -> Option<Vec<u8>> {
    let img = image::load_from_memory(bytes).ok()?;
    let (w, h) = img.dimensions();

    // Crop to 16:9 to eliminate letterbox/pillarbox bars from 4:3 YouTube thumbnails
    let target_aspect = 16.0 / 9.0;
    let current_aspect = w as f64 / h as f64;
    let img = if (current_aspect - target_aspect).abs() > 0.05 {
        if current_aspect < target_aspect {
            let new_h = ((w as f64) / target_aspect).round() as u32;
            let y = (h.saturating_sub(new_h)) / 2;
            img.crop_imm(0, y, w, new_h.min(h))
        } else {
            let new_w = ((h as f64) * target_aspect).round() as u32;
            let x = (w.saturating_sub(new_w)) / 2;
            img.crop_imm(x, 0, new_w.min(w), h)
        }
    } else {
        img
    };

    let mut rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let r = radius.min(w / 2).min(h / 2);

    if r > 0 {
        apply_rounded_mask(&mut rgba, w, h, r);
    }

    encode_png(&DynamicImage::ImageRgba8(rgba))
}

/// Crop an image to a circle (for channel avatars).
pub fn circle(bytes: &[u8]) -> Option<Vec<u8>> {
    let img = image::load_from_memory(bytes).ok()?;
    let (w, h) = img.dimensions();
    let side = w.min(h);
    let x = (w - side) / 2;
    let y = (h - side) / 2;
    let cropped = img.crop_imm(x, y, side, side);
    let mut rgba = cropped.to_rgba8();
    apply_circle_mask(&mut rgba, side);
    encode_png(&DynamicImage::ImageRgba8(rgba))
}

fn apply_rounded_mask(rgba: &mut image::RgbaImage, w: u32, h: u32, r: u32) {
    let rf = r as f64;
    let wf = w as f64;
    let hf = h as f64;

    // Only inspect pixels inside each corner's outer quadrant:
    // (x_range, y_range, arc_center_x, arc_center_y)
    let quadrants = [
        (0..r, 0..r, rf, rf),                             // Top-left
        (w - r..w, 0..r, wf - rf, rf),                    // Top-right
        (0..r, h - r..h, rf, hf - rf),                    // Bottom-left
        (w - r..w, h - r..h, wf - rf, hf - rf),           // Bottom-right
    ];

    for (x_range, y_range, cx, cy) in quadrants {
        for y in y_range {
            let py = y as f64 + 0.5;
            let dy = py - cy;
            for x in x_range.clone() {
                let px = x as f64 + 0.5;
                let dx = px - cx;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > rf {
                    let delta = dist - rf;
                    if delta < 1.0 {
                        let orig_a = rgba[(x, y)][3] as f64;
                        rgba[(x, y)][3] = ((1.0 - delta) * orig_a).round() as u8;
                    } else {
                        rgba[(x, y)][3] = 0;
                    }
                }
            }
        }
    }
}

fn apply_circle_mask(rgba: &mut image::RgbaImage, side: u32) {
    let radius = side as f64 / 2.0;
    let center = radius;
    for y in 0..side {
        let py = y as f64 + 0.5;
        let dy = py - center;
        for x in 0..side {
            let px = x as f64 + 0.5;
            let dx = px - center;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist > radius {
                let delta = dist - radius;
                if delta < 1.0 {
                    let orig_a = rgba[(x, y)][3] as f64;
                    rgba[(x, y)][3] = ((1.0 - delta) * orig_a).round() as u8;
                } else {
                    rgba[(x, y)][3] = 0;
                }
            }
        }
    }
}

fn encode_png(img: &DynamicImage) -> Option<Vec<u8>> {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Png).ok()?;
    Some(buf.into_inner())
}
