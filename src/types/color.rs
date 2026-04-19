/// Represents a color in RGBA format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Color> for cosmic_text::Color {
    fn from(c: Color) -> Self {
        cosmic_text::Color::rgba(c.r, c.g, c.b, c.a)
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        c.to_f32_array()
    }
}

impl Color {
    /// Creates a new RGB color with full opacity.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Creates a new RGBA color.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Parses a color from a hex string.
    /// Supports: #RGB, #RGBA, #RRGGBB, #RRGGBBAA
    pub fn hex(hex_str: &str) -> Self {
        let h = hex_str.trim_start_matches('#');
        match h.len() {
            3 => {
                let r = u8::from_str_radix(&h[0..1].repeat(2), 16).unwrap_or(0);
                let g = u8::from_str_radix(&h[1..2].repeat(2), 16).unwrap_or(0);
                let b = u8::from_str_radix(&h[2..3].repeat(2), 16).unwrap_or(0);
                Self::rgb(r, g, b)
            }
            4 => {
                let r = u8::from_str_radix(&h[0..1].repeat(2), 16).unwrap_or(0);
                let g = u8::from_str_radix(&h[1..2].repeat(2), 16).unwrap_or(0);
                let b = u8::from_str_radix(&h[2..3].repeat(2), 16).unwrap_or(0);
                let a = u8::from_str_radix(&h[3..4].repeat(2), 16).unwrap_or(255);
                Self::rgba(r, g, b, a)
            }
            6 => {
                let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
                Self::rgb(r, g, b)
            }
            8 => {
                let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&h[6..8], 16).unwrap_or(255);
                Self::rgba(r, g, b, a)
            }
            _ => Self::BLACK, // Fallback
        }
    }

    /// Returns a new color with the specified alpha value.
    pub const fn with_alpha(self, a: u8) -> Self {
        Self { r: self.r, g: self.g, b: self.b, a }
    }

    /// Converts the color to a `[f32; 4]` representing RGBA in 0.0 to 1.0 range.
    pub fn to_f32_array(self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    /// Converts the color to a `u32` (0xRRGGBBAA).
    pub const fn to_u32(self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }

    // Common Colors
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
    pub const RED: Color = Color::rgb(255, 59, 48);
    pub const GREEN: Color = Color::rgb(52, 199, 89);
    pub const BLUE: Color = Color::rgb(0, 122, 255);
    pub const YELLOW: Color = Color::rgb(255, 204, 0);
    pub const ORANGE: Color = Color::rgb(255, 149, 0);
    pub const PURPLE: Color = Color::rgb(175, 82, 222);
    pub const PINK: Color = Color::rgb(255, 45, 85);
    pub const GRAY: Color = Color::rgb(142, 142, 147);
    pub const LIGHT_GRAY: Color = Color::rgb(209, 209, 214);
    pub const DARK_GRAY: Color = Color::rgb(28, 28, 30);
    pub const TEAL: Color = Color::rgb(48, 176, 199);
    pub const INDIGO: Color = Color::rgb(88, 86, 214);
    pub const CYAN: Color = Color::rgb(50, 173, 230);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    pub const SKY_BLUE: Color = Color::rgb(135, 206, 235);
    pub const MINT: Color = Color::rgb(0, 199, 190);
    pub const EMERALD: Color = Color::rgb(52, 199, 89);
    pub const AMBER: Color = Color::rgb(255, 191, 0);
    pub const LAVENDER: Color = Color::rgb(175, 82, 222);
    pub const CRIMSON: Color = Color::rgb(220, 20, 60);
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parsing() {
        assert_eq!(Color::hex("#FFF"), Color::WHITE);
        assert_eq!(Color::hex("#FFFFFF"), Color::WHITE);
        assert_eq!(Color::hex("#000000"), Color::BLACK);
        assert_eq!(Color::hex("#FF0000"), Color::rgb(255, 0, 0));
        assert_eq!(Color::hex("#00FF00FF"), Color::rgba(0, 255, 0, 255));
        assert_eq!(Color::hex("FF00FF"), Color::rgb(255, 0, 255)); // No # sign
    }

    #[test]
    fn test_with_alpha() {
        let c = Color::WHITE.with_alpha(128);
        assert_eq!(c.a, 128);
        assert_eq!(c.r, 255);
    }

    #[test]
    fn test_to_f32() {
        let c = Color::rgb(255, 0, 127);
        let arr = c.to_f32_array();
        assert_eq!(arr[0], 1.0);
        assert_eq!(arr[1], 0.0);
        assert!((arr[2] - 127.0 / 255.0).abs() < 0.001);
        assert_eq!(arr[3], 1.0);
    }
}
