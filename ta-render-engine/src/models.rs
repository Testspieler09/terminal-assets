use std::path::Path;

// TODO: this still needs major adjusting

pub enum DefaultRatio {
    OneToOne,
}

pub struct Dimensions {
    width: u32,
    height: u32,
}

impl From<DefaultRatio> for Dimensions {
    fn from(ratio: DefaultRatio) -> Self {
        match ratio {
            DefaultRatio::OneToOne => todo!(),
        }
    }
}

pub struct Grid {
    columns: u32,
    rows: u32,
    // cell width and height missing
}

pub struct Image {
    dimensions: Dimensions,
    grid: Grid,
}

pub struct Video {}

pub struct FontSettings<'a> {
    font_path: &'a Path,
    font_size: u32,
}
