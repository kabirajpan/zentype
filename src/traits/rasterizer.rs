use crate::types::glyph::{GlyphKey, RasterizedGlyph};

/// A trait for components that can rasterize glyphs into pixel data.
pub trait Rasterizer {
    /// Renders a single glyph into a bitmap.
    /// Returns None if the glyph could not be rasterized.
    fn rasterize(&self, key: &GlyphKey) -> Option<RasterizedGlyph>;
}
