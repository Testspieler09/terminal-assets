use ratatui::{buffer::Buffer, layout::Rect};

use crate::depth_effect::DepthEffect;

pub struct LayeredBuffer {
    /// (z_index, buffer)
    layers: Vec<(i32, Buffer)>,
    area: Rect,
}

impl LayeredBuffer {
    pub fn new(area: Rect) -> Self {
        Self {
            layers: vec![],
            area,
        }
    }

    /// Get or create a layer at the given z-index to render into.
    pub fn layer(&mut self, z: i32) -> &mut Buffer {
        if let Some(pos) = self.layers.iter().position(|(z_idx, _)| *z_idx == z) {
            return &mut self.layers[pos].1;
        }
        self.layers.push((z, Buffer::empty(self.area)));
        self.layers.sort_by_key(|(z, _)| *z);
        let pos = self
            .layers
            .iter()
            .position(|(z_idx, _)| *z_idx == z)
            .unwrap();
        &mut self.layers[pos].1
    }

    /// Composite all layers into a single flat buffer (lowest z at the bottom).
    pub fn flatten(&self) -> Buffer {
        let mut out = Buffer::empty(self.area);
        for (_, layer) in &self.layers {
            for (i, cell) in layer.content.iter().enumerate() {
                if cell.symbol() != " " && cell.symbol() != "\x00" {
                    out.content[i] = cell.clone();
                }
            }
        }
        out
    }

    /// Return layers sorted by z for the rasterizer to render with offsets.
    pub fn layers(&self) -> &[(i32, Buffer)] {
        &self.layers
    }

    pub fn area(&self) -> Rect {
        self.area
    }
}

pub enum FrameBuffer {
    Flat(Buffer),
    Layered(LayeredBuffer, DepthEffect),
}

impl From<Buffer> for FrameBuffer {
    fn from(b: Buffer) -> Self {
        Self::Flat(b)
    }
}

impl From<(LayeredBuffer, DepthEffect)> for FrameBuffer {
    fn from((b, e): (LayeredBuffer, DepthEffect)) -> Self {
        Self::Layered(b, e)
    }
}
