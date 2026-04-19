use crate::traits::rasterizer::Rasterizer;
use crate::types::glyph::{GlyphKey, RasterizedGlyph};
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
    fn rasterize(&self, _key: &GlyphKey) -> Option<RasterizedGlyph> {
        // This will be implemented fully once we decide how to handle the global FontSystem.
        None
    }
}
