use crate::types::glyph::RasterizedGlyph;
use crate::types::shaped_glyph::ShapedGlyph;

/// A trait for components that can rasterize glyphs into pixel data.
pub trait Rasterizer: Send + Sync {
    /// Renders a single shaped glyph into a bitmap.
    /// Returns None if the glyph could not be rasterized.
    fn rasterize(&mut self, glyph: &ShapedGlyph) -> Option<RasterizedGlyph>;
}

