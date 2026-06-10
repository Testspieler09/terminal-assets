use ab_glyph::{Font, FontVec, Glyph, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};
use ratatui::{buffer::Buffer, style::Modifier};

use crate::{
    color::{ColorConfig, Rgb},
    models::{CellSize, FontSettings},
};

// TODO: add the z index filter here by warping the raster / moving the cells

pub struct Rasterizer<'a> {
    font: &'a FontVec,
    scale: PxScale,
    cell_size: CellSize,
}

impl<'a> Rasterizer<'a> {
    pub fn new(settings: &FontSettings, font: &'a FontVec) -> Self {
        let scale = PxScale::from(settings.font_size);
        let cell_size = measure_cell(font, scale);
        Self {
            font,
            scale,
            cell_size,
        }
    }

    pub fn cell_size(&self) -> CellSize {
        self.cell_size
    }

    /// Rasterize a ratatui `Buffer` into an `RgbaImage`.
    pub fn rasterize(&self, buffer: &Buffer, colors: &ColorConfig) -> RgbaImage {
        let cols = buffer.area.width as u32;
        let rows = buffer.area.height as u32;
        let img_w = cols * self.cell_size.width;
        let img_h = rows * self.cell_size.height;

        let mut img = RgbaImage::new(img_w, img_h);

        for (index, cell) in buffer.content.iter().enumerate() {
            let col = (index as u32) % cols;
            let row = (index as u32) / cols;

            let fg = colors.resolve_fg(cell.fg);
            let bg = colors.resolve_bg(cell.bg);
            let bold = cell.modifier.contains(Modifier::BOLD);

            let px = col * self.cell_size.width;
            let py = row * self.cell_size.height;

            // fill cell background
            fill_rect(
                &mut img,
                px,
                py,
                self.cell_size.width,
                self.cell_size.height,
                bg,
            );

            // draw glyph
            let symbol = cell.symbol();
            if !symbol.is_empty() && symbol != " " {
                self.draw_glyph(&mut img, symbol, px, py, fg, bold);
            }
        }

        img
    }

    fn draw_glyph(
        &self,
        img: &mut RgbaImage,
        symbol: &str,
        cell_x: u32,
        cell_y: u32,
        fg: Rgb,
        bold: bool,
    ) {
        let scale = if bold {
            // TODO: find a better solution here
            PxScale::from(self.scale.x) // swap for a bold font if available
        } else {
            self.scale
        };

        let scaled = self.font.as_scaled(scale);

        for ch in symbol.chars() {
            let glyph_id = self.font.glyph_id(ch);
            let glyph: Glyph = glyph_id.with_scale_and_position(
                scale,
                ab_glyph::point(cell_x as f32, cell_y as f32 + scaled.ascent()),
            );

            if let Some(outlined) = self.font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                outlined.draw(|gx, gy, cov| {
                    let px = bounds.min.x as u32 + gx;
                    let py = bounds.min.y as u32 + gy;
                    // TODO: check if this is even needed or if it makes the image look less premium
                    if px < img.width() && py < img.height() {
                        let alpha = (cov * 255.0) as u8;
                        let pixel = img.get_pixel_mut(px, py);
                        // alpha-blend over background
                        *pixel = blend(*pixel, fg, alpha);
                    }
                });
            }
        }
    }
}

fn measure_cell(font: &FontVec, scale: PxScale) -> CellSize {
    // TODO: find a better solution here
    let scaled = font.as_scaled(scale);
    // Use 'M' as the reference glyph for monospace cell size
    let glyph_id = font.glyph_id('M');
    let advance = scaled.h_advance(glyph_id);
    let height = scaled.ascent() - scaled.descent() + scaled.line_gap();
    CellSize {
        width: advance.ceil() as u32,
        height: height.ceil() as u32,
    }
}

fn fill_rect(img: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32, color: Rgb) {
    let pixel = Rgba([color.0, color.1, color.2, 255]);
    for dy in 0..h {
        for dx in 0..w {
            let px = x + dx;
            let py = y + dy;
            if px < img.width() && py < img.height() {
                img.put_pixel(px, py, pixel);
            }
        }
    }
}

fn blend(base: Rgba<u8>, fg: Rgb, alpha: u8) -> Rgba<u8> {
    let a = alpha as f32 / 255.0;
    Rgba([
        (fg.0 as f32 * a + base[0] as f32 * (1.0 - a)) as u8,
        (fg.1 as f32 * a + base[1] as f32 * (1.0 - a)) as u8,
        (fg.2 as f32 * a + base[2] as f32 * (1.0 - a)) as u8,
        255,
    ])
}
