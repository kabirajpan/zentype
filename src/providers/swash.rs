use crate::traits::rasterizer::Rasterizer;
use crate::types::glyph::{GlyphKey, RasterizedGlyph};
use crate::types::shaped_glyph::ShapedGlyph;
use cosmic_text::{SwashCache, FontSystem};

/// A Rasterizer that uses the `swash` library (via cosmic-text) to render glyphs.
pub struct SwashRasterizer {
    cache: SwashCache,
}

impl SwashRasterizer {
    /// Creates a new SwashRasterizer.
    pub fn new() -> Self {
        Self {
            cache: SwashCache::new(),
        }
    }

    /// Renders a glyph using an external FontSystem.
    pub fn rasterize_with_system(&mut self, font_system: &mut FontSystem, key: &GlyphKey) -> Option<RasterizedGlyph> {
        let image = self.cache.get_image(font_system, *key).as_ref()?;
        Some(RasterizedGlyph {
            width: image.placement.width,
            height: image.placement.height,
            data: image.data.clone(),
        })
    }
}

impl Rasterizer for SwashRasterizer {
    fn rasterize(&mut self, _glyph: &ShapedGlyph) -> Option<RasterizedGlyph> {
        // Implement by using the inner cache and the key from the shaped glyph
        // Note: This currently lacks a FontSystem, which will be addressed in Phase 4/5.
        // For now, we use a placeholder or the existing rasterize_with_system logic if possible.
        None
    }
}

