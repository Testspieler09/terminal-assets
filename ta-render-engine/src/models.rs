use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CellSize {
    pub width: u32,
    pub height: u32,
}

impl Default for CellSize {
    /// A sensible monospace default (8x16 is classic, matches many bitmap fonts)
    fn default() -> Self {
        Self {
            width: 8,
            height: 16,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Grid {
    pub columns: u32,
    pub rows: u32,
    pub cell_size: CellSize,
}

impl Grid {
    /// Construct from explicit cell count + cell size.
    pub fn new(columns: u32, rows: u32, cell_size: CellSize) -> Self {
        Self {
            columns,
            rows,
            cell_size,
        }
    }

    /// Construct by dividing a known pixel canvas into cells.
    /// Truncates - canvas may be slightly larger than columns*rows*cell.
    pub fn from_dimensions(dimensions: Dimensions, cell_size: CellSize) -> Self {
        Self {
            columns: dimensions.width / cell_size.width,
            rows: dimensions.height / cell_size.height,
            cell_size,
        }
    }

    /// The exact pixel canvas this grid occupies.
    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.columns * self.cell_size.width,
            height: self.rows * self.cell_size.height,
        }
    }
}

/// Named canvas sizes. Convert to [`Dimensions`] before use.
#[derive(Clone, Copy)]
pub enum AspectPreset {
    OneToOne,
    SixteenToNine,
    /// Nintendo 3DS top screen (400x240)
    CtrTopScreen,
    /// Nintendo 3DS bottom screen (320x240)
    CtrBottomScreen,
    Custom {
        width: u32,
        height: u32,
    },
}

impl From<AspectPreset> for Dimensions {
    fn from(preset: AspectPreset) -> Self {
        match preset {
            AspectPreset::OneToOne => Self {
                width: 512,
                height: 512,
            },
            AspectPreset::SixteenToNine => Self {
                width: 1280,
                height: 720,
            },
            AspectPreset::CtrTopScreen => Self {
                width: 400,
                height: 240,
            },
            AspectPreset::CtrBottomScreen => Self {
                width: 320,
                height: 240,
            },
            AspectPreset::Custom { width, height } => Self { width, height },
        }
    }
}

#[derive(Clone)]
pub struct ImageOutput {
    pub grid: Grid,
}

impl ImageOutput {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }

    /// Convenience: build from a preset canvas size + cell size.
    pub fn from_preset(preset: AspectPreset, cell_size: CellSize) -> Self {
        let dims = Dimensions::from(preset);
        Self {
            grid: Grid::from_dimensions(dims, cell_size),
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        self.grid.dimensions()
    }
}

#[derive(Clone)]
pub struct VideoOutput {
    pub grid: Grid,
    pub frames_per_second: u32,
    pub frames: usize,
}

impl VideoOutput {
    pub fn new(grid: Grid, frames_per_second: u32, frames: usize) -> Self {
        Self {
            grid,
            frames_per_second,
            frames,
        }
    }

    pub fn duration_secs(&self) -> f64 {
        self.frames as f64 / self.frames_per_second as f64
    }

    pub fn dimensions(&self) -> Dimensions {
        self.grid.dimensions()
    }
}

#[derive(Clone)]
pub struct FontSettings {
    pub font_path: PathBuf,
    /// Point size passed to ab_glyph's [`PxScale`]
    pub font_size: f32,
}

impl FontSettings {
    /// Estimate the cell size this font will produce at the given size.
    /// You'll want to replace this with a real measurement from ab_glyph
    /// once you can load the font (call [`ab_glyph::Font::glyph_bounds`]).
    pub fn estimated_cell_size(&self) -> CellSize {
        CellSize::default()
    }
}
