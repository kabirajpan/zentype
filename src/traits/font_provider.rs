use std::path::Path;
use crate::types::options::TextOptions;
use crate::primitives::shaped_buffer::ShapedBuffer;
// Removed unused Attrs import

/// A trait for components that can load fonts and shape text into glyphs.
pub trait FontProvider: Send + Sync {
    /// Shapes a string of text into a buffered list of positioned glyphs.
    /// This handles layout, wrapping, and alignment internally.
    fn shape(&mut self, text: &str, options: &TextOptions) -> ShapedBuffer;

    /// Loads a font from raw byte data.
    fn load_font(&mut self, data: Vec<u8>);

    /// Loads a font from a file path.
    fn load_font_path(&mut self, path: &Path) -> std::io::Result<()>;

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

