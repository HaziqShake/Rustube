//! Thumbnail / avatar image prep.
//!
//! Iced's `image` widget draws square corners, so YouTube's rounded
//! thumbnails (12px) and circular avatars cannot be done with a container
//! border radius alone: the image would poke out past the rounded corners.
//! Instead we bake the rounding into the image itself once, at download
//! time, and cache the processed PNG bytes. Cheap (one pass over the
//! pixels) and done exactly once per image.

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
            // Taller than 16:9 (e.g. 4:3 hqdefault) - crop black letterbox bars top & bottom
            let new_h = ((w as f64) / target_aspect).round() as u32;
            let y = (h.saturating_sub(new_h)) / 2;
            img.crop_imm(0, y, w, new_h.min(h))
        } else {
            // Wider than 16:9 - crop left and right
            let new_w = ((h as f64) * target_aspect).round() as u32;
            let x = (w.saturating_sub(new_w)) / 2;
            img.crop_imm(x, 0, new_w.min(w), h)
        }
    } else {
        img
    };

    let mut rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let r = radius.min(w / 2).min(h / 2) as i64;

    if r > 0 {
        apply_rounded_mask(&mut rgba, w, h, r);
    }

    encode_png(&DynamicImage::ImageRgba8(rgba))
}

/// Crop an image to a circle (for channel avatars). The source is first
/// centre-cropped to a square, then masked to a circle with smooth anti-aliasing.
pub fn circle(bytes: &[u8]) -> Option<Vec<u8>> {
    let img = image::load_from_memory(bytes).ok()?;
    let (w, h) = img.dimensions();
    let side = w.min(h);
    let x = (w - side) / 2;
    let y = (h - side) / 2;
    let cropped = img.crop_imm(x, y, side, side);
    let mut rgba = cropped.to_rgba8();
    apply_rounded_mask(&mut rgba, side, side, (side / 2) as i64);
    encode_png(&DynamicImage::ImageRgba8(rgba))
}

fn apply_rounded_mask(
    rgba: &mut image::RgbaImage,
    w: u32,
    h: u32,
    r: i64,
) {
    let (w_i, h_i) = (w as i64, h as i64);
    let corners = [
        (r, r),                     // top-left
        (w_i - 1 - r, r),           // top-right
        (r, h_i - 1 - r),           // bottom-left
        (w_i - 1 - r, h_i - 1 - r), // bottom-right
    ];
    let rf = r as f64;
    for (cx, cy) in corners {
        let x0 = (cx - r).max(0);
        let x1 = (cx + r).min(w_i - 1);
        let y0 = (cy - r).max(0);
        let y1 = (cy + r).min(h_i - 1);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let dx = (x - cx) as f64;
                let dy = (y - cy) as f64;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > rf {
                    let delta = dist - rf;
                    if delta < 1.0 {
                        // Smooth subpixel anti-aliasing
                        let orig_a = rgba[(x as u32, y as u32)][3] as f64;
                        rgba[(x as u32, y as u32)][3] = ((1.0 - delta) * orig_a).round() as u8;
                    } else {
                        rgba[(x as u32, y as u32)][3] = 0;
                    }
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
