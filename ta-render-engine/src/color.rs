use ratatui::style::Color;

/// RGB triple.
#[derive(Clone, Copy, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// Per-target color configuration.
#[derive(Clone)]
pub struct ColorConfig {
    /// Overridable ANSI 16-color palette.
    pub ansi_palette: [Rgb; 16],
    /// Color used when fg is `Color::Reset`.
    pub default_fg: Rgb,
    /// Color used when bg is `Color::Reset`.
    pub default_bg: Rgb,
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

/// Predefined color schemes for convenience.
#[derive(Clone, Copy, Default)]
pub enum ColorScheme {
    #[default]
    Classic,
    SolarizedDark,
    Dracula,
    Nord,
}

impl From<ColorScheme> for ColorConfig {
    fn from(scheme: ColorScheme) -> Self {
        match scheme {
            ColorScheme::Classic => ColorConfig {
                ansi_palette: [
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
                ],
                default_fg: Rgb(255, 255, 255),
                default_bg: Rgb(0, 0, 0),
            },
            ColorScheme::SolarizedDark => ColorConfig {
                ansi_palette: [
                    Rgb(7, 54, 66),     // Black
                    Rgb(220, 50, 47),   // Red
                    Rgb(133, 153, 0),   // Green
                    Rgb(181, 137, 0),   // Yellow
                    Rgb(38, 139, 210),  // Blue
                    Rgb(211, 54, 130),  // Magenta
                    Rgb(42, 161, 152),  // Cyan
                    Rgb(238, 232, 213), // White
                    Rgb(0, 43, 54),     // BrightBlack
                    Rgb(203, 75, 22),   // BrightRed
                    Rgb(88, 110, 117),  // BrightGreen
                    Rgb(101, 123, 131), // BrightYellow
                    Rgb(131, 148, 150), // BrightBlue
                    Rgb(108, 113, 196), // BrightMagenta
                    Rgb(147, 161, 161), // BrightCyan
                    Rgb(253, 246, 227), // BrightWhite
                ],
                default_fg: Rgb(131, 148, 150),
                default_bg: Rgb(0, 43, 54),
            },
            ColorScheme::Dracula => ColorConfig {
                ansi_palette: [
                    Rgb(40, 42, 54),    // Black
                    Rgb(255, 85, 85),   // Red
                    Rgb(80, 250, 123),  // Green
                    Rgb(241, 250, 140), // Yellow
                    Rgb(189, 147, 249), // Blue
                    Rgb(255, 121, 198), // Magenta
                    Rgb(139, 233, 253), // Cyan
                    Rgb(248, 248, 242), // White
                    Rgb(68, 71, 90),    // BrightBlack
                    Rgb(255, 110, 110), // BrightRed
                    Rgb(105, 255, 148), // BrightGreen
                    Rgb(255, 255, 165), // BrightYellow
                    Rgb(214, 172, 255), // BrightBlue
                    Rgb(255, 146, 223), // BrightMagenta
                    Rgb(164, 255, 255), // BrightCyan
                    Rgb(255, 255, 255), // BrightWhite
                ],
                default_fg: Rgb(248, 248, 242),
                default_bg: Rgb(40, 42, 54),
            },
            ColorScheme::Nord => ColorConfig {
                ansi_palette: [
                    Rgb(46, 52, 64),    // Black
                    Rgb(191, 97, 106),  // Red
                    Rgb(163, 190, 140), // Green
                    Rgb(235, 203, 139), // Yellow
                    Rgb(129, 161, 193), // Blue
                    Rgb(180, 142, 173), // Magenta
                    Rgb(136, 192, 208), // Cyan
                    Rgb(229, 233, 240), // White
                    Rgb(59, 66, 82),    // BrightBlack
                    Rgb(191, 97, 106),  // BrightRed
                    Rgb(163, 190, 140), // BrightGreen
                    Rgb(235, 203, 139), // BrightYellow
                    Rgb(129, 161, 193), // BrightBlue
                    Rgb(180, 142, 173), // BrightMagenta
                    Rgb(136, 192, 208), // BrightCyan
                    Rgb(236, 239, 244), // BrightWhite
                ],
                default_fg: Rgb(216, 222, 233),
                default_bg: Rgb(46, 52, 64),
            },
        }
    }
}
