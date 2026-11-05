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

/// Round the corners of an image (JPEG/PNG/WebP in, PNG out).
/// `radius` is in pixels, relative to the source image size.
pub fn rounded(bytes: &[u8], radius: u32) -> Option<Vec<u8>> {
    let img = image::load_from_memory(bytes).ok()?;
    let mut rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let r = radius.min(w / 2).min(h / 2) as i64;

    if r > 0 {
        apply_rounded_mask(&mut rgba, w, h, r);
    }

    encode_png(&DynamicImage::ImageRgba8(rgba))
}

/// Crop an image to a circle (for channel avatars). The source is first
/// centre-cropped to a square, then masked to a circle.
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
    let (w, h) = (w as i64, h as i64);
    // Corner centres, in pixel coordinates.
    let corners = [
        (r, r),            // top-left
        (w - 1 - r, r),    // top-right
        (r, h - 1 - r),    // bottom-left
        (w - 1 - r, h - 1 - r), // bottom-right
    ];
    for (cx, cy) in corners {
        // Only scan the corner square.
        let x0 = (cx - r).max(0);
        let x1 = (cx + r).min(w - 1);
        let y0 = (cy - r).max(0);
        let y1 = (cy + r).min(h - 1);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy > r * r {
                    rgba[(x as u32, y as u32)][3] = 0;
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
