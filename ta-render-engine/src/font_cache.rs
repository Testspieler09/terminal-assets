use std::{collections::HashMap, path::PathBuf};

use ab_glyph::FontVec;

#[derive(Default)]
pub struct FontCache {
    fonts: HashMap<PathBuf, FontVec>,
}

impl FontCache {
    pub fn load_and_insert_font(&mut self, font_key: PathBuf) -> Result<&FontVec, FontCacheError> {
        if !self.fonts.contains_key(&font_key) {
            let bytes = std::fs::read(&font_key)
                .map_err(|e| FontCacheError::FileNotFound(e.to_string()))?;
            let font = FontVec::try_from_vec(bytes)
                .map_err(|e| FontCacheError::FontLoad(e.to_string()))?;
            self.fonts.insert(font_key.clone(), font);
        }
        Ok(self.fonts.get(&font_key).unwrap())
    }
}

#[derive(Debug)]
pub enum FontCacheError {
    FileNotFound(String),
    FontLoad(String),
}

impl std::fmt::Display for FontCacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound(e) => write!(f, "file not found: {e}"),
            Self::FontLoad(e) => write!(f, "font load failed: {e}"),
        }
    }
}

impl std::error::Error for FontCacheError {}
