use ratatui::style::Color;

/// RGB triple.
#[derive(Clone, Copy, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// The classic 16-color ANSI palette.
/// Order: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,
///        + bright variants.
pub const DEFAULT_ANSI_PALETTE: [Rgb; 16] = [
    Rgb(0, 0, 0),       // Black
    Rgb(170, 0, 0),     // Red
    Rgb(0, 170, 0),     // Green
    Rgb(170, 170, 0),   // Yellow
    Rgb(0, 0, 170),     // Blue
    Rgb(170, 0, 170),   // Magenta
    Rgb(0, 170, 170),   // Cyan
    Rgb(170, 170, 170), // White
    Rgb(85, 85, 85),    // BrightBlack
    Rgb(255, 85, 85),   // BrightRed
    Rgb(85, 255, 85),   // BrightGreen
    Rgb(255, 255, 85),  // BrightYellow
    Rgb(85, 85, 255),   // BrightBlue
    Rgb(255, 85, 255),  // BrightMagenta
    Rgb(85, 255, 255),  // BrightCyan
    Rgb(255, 255, 255), // BrightWhite
];

/// Per-target color configuration.
#[derive(Clone, Debug)]
pub struct ColorConfig {
    /// Overridable ANSI 16-color palette.
    pub ansi_palette: [Rgb; 16],
    /// Color used when fg is `Color::Reset`.
    pub default_fg: Rgb,
    /// Color used when bg is `Color::Reset`.
    pub default_bg: Rgb,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            ansi_palette: DEFAULT_ANSI_PALETTE,
            default_fg: Rgb(255, 255, 255),
            default_bg: Rgb(0, 0, 0),
        }
    }
}

impl ColorConfig {
    pub fn resolve_fg(&self, color: Color) -> Rgb {
        self.resolve(color, self.default_fg)
    }

    pub fn resolve_bg(&self, color: Color) -> Rgb {
        self.resolve(color, self.default_bg)
    }

    fn resolve(&self, color: Color, reset_fallback: Rgb) -> Rgb {
        match color {
            Color::Reset => reset_fallback,
            Color::Black => self.ansi_palette[0],
            Color::Red => self.ansi_palette[1],
            Color::Green => self.ansi_palette[2],
            Color::Yellow => self.ansi_palette[3],
            Color::Blue => self.ansi_palette[4],
            Color::Magenta => self.ansi_palette[5],
            Color::Cyan => self.ansi_palette[6],
            Color::Gray => self.ansi_palette[7],
            Color::DarkGray => self.ansi_palette[8],
            Color::LightRed => self.ansi_palette[9],
            Color::LightGreen => self.ansi_palette[10],
            Color::LightYellow => self.ansi_palette[11],
            Color::LightBlue => self.ansi_palette[12],
            Color::LightMagenta => self.ansi_palette[13],
            Color::LightCyan => self.ansi_palette[14],
            Color::White => self.ansi_palette[15],
            Color::Rgb(r, g, b) => Rgb(r, g, b),
            // 256-color: map into ansi palette for the first 16,
            // otherwise best-effort fallback to default
            Color::Indexed(i) => {
                if (i as usize) < 16 {
                    self.ansi_palette[i as usize]
                } else {
                    reset_fallback
                }
            }
        }
    }
}
