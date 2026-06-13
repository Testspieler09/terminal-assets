use std::{collections::HashMap, path::PathBuf};

use ab_glyph::FontVec;

pub struct FontCache {
    font_dirs: Vec<PathBuf>,
    fonts: HashMap<PathBuf, FontVec>,
}

impl FontCache {
    pub fn new(font_dirs: Vec<PathBuf>) -> Self {
        Self {
            font_dirs,
            fonts: Default::default(),
        }
    }

    pub fn load_and_insert_font(&mut self, font_key: PathBuf) -> Result<&FontVec, FontCacheError> {
        if !self.fonts.contains_key(&font_key) {
            let resolved = if font_key.exists() {
                font_key.clone()
            } else {
                self.font_dirs
                    .iter()
                    .map(|dir| dir.join(&font_key))
                    .find(|p| p.exists())
                    .ok_or_else(|| {
                        let searched = self
                            .font_dirs
                            .iter()
                            .map(|d| format!("  - {}", d.join(&font_key).display()))
                            .collect::<Vec<_>>()
                            .join("\n");
                        FontCacheError::FileNotFound(format!(
                            "font '{}' not found at any of:\n{}",
                            font_key.display(),
                            searched
                        ))
                    })?
            };

            let bytes = std::fs::read(&resolved)
                .map_err(|e| FontCacheError::FileNotFound(e.to_string()))?;
            let font = FontVec::try_from_vec(bytes)
                .map_err(|e| FontCacheError::FontLoad(e.to_string()))?;
            self.fonts.insert(font_key.clone(), font);
        }
        Ok(self.fonts.get(&font_key).unwrap())
    }

    pub fn load_font_pair(
        &mut self,
        font_path: PathBuf,
        bold_font_path: Option<PathBuf>,
    ) -> Result<(&FontVec, Option<&FontVec>), FontCacheError> {
        self.load_and_insert_font(font_path.clone())?;
        if let Some(ref p) = bold_font_path {
            self.load_and_insert_font(p.clone())?;
        }
        let font = self.fonts.get(&font_path).unwrap();
        let bold = bold_font_path.as_ref().and_then(|p| self.fonts.get(p));
        Ok((font, bold))
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
