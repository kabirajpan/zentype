use crate::types::options::TextOptions;
use crate::types::shaped_glyph::ShapedGlyph;

/// A trait for components that can load fonts and shape text into glyphs.
pub trait FontProvider {
    /// Shapes a string of text into a list of positioned glyphs.
    /// This handles layout, wrapping, and alignment internally.
    fn shape(&mut self, text: &str, options: &TextOptions) -> Vec<ShapedGlyph>;

    /// Returns the metrics for a given set of options (e.g., line height).
    fn metrics(&self, options: &TextOptions) -> FontMetrics;
}

/// Basic vertical metrics for a font at a specific size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

impl FontMetrics {
    pub fn line_height(&self) -> f32 {
        self.ascent - self.descent + self.line_gap
    }
}
