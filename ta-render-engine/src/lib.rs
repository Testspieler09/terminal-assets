pub mod codec;
pub mod models;

use ratatui::{buffer::Buffer, layout::Rect};

use crate::models::{FontSettings, ImageOutput, VideoOutput};

/// What a target produces.
#[derive(Clone)]
pub enum OutputConfig {
    Image(ImageOutput),
    Video(VideoOutput),
}

impl OutputConfig {
    pub fn grid(&self) -> &crate::models::Grid {
        match self {
            Self::Image(i) => &i.grid,
            Self::Video(v) => &v.grid,
        }
    }

    pub fn rect(&self) -> Rect {
        let grid = self.grid();
        Rect {
            x: 0,
            y: 0,
            width: grid.columns as u16,
            height: grid.rows as u16,
        }
    }
}

/// A single render target: one output config, one font, one layout.
pub struct SceneTarget {
    pub output: OutputConfig,
    pub font: FontSettings,
}

impl SceneTarget {
    pub fn new(output: OutputConfig, font: FontSettings) -> Self {
        Self { output, font }
    }

    /// Total frames to render. Images are always 1.
    pub fn frame_count(&self) -> usize {
        match &self.output {
            OutputConfig::Image(_) => 1,
            OutputConfig::Video(v) => v.frames,
        }
    }
}

/// The trait every scene must implement.
pub trait Scene {
    /// Unique identifier used for CLI `--scenes` filtering and output naming.
    fn name(&self) -> &str;

    /// All targets this scene renders to. Called once by main at startup.
    fn targets(&self) -> Vec<SceneTarget>;

    /// Render a single frame into a fresh buffer.
    ///
    /// - `target`  - the target currently being rendered
    /// - `frame`   - current frame index (always 0 for images)
    ///
    /// The buffer is pre-sized to `target.output.rect()`.
    fn render_frame(&self, target: &SceneTarget, frame: usize) -> Buffer {
        let rect = target.output.rect();
        let mut buffer = Buffer::empty(rect);
        self.draw(target, frame, rect, &mut buffer);
        buffer
    }

    /// Where the actual ratatui widget drawing happens.
    /// Implement this instead of `render_frame` directly.
    fn draw(&self, target: &SceneTarget, frame: usize, area: Rect, buffer: &mut Buffer);
}
