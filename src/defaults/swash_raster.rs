use crate::traits::rasterizer::Rasterizer;
use crate::types::glyph::RasterizedGlyph;
use crate::types::shaped_glyph::ShapedGlyph;
use cosmic_text::{SwashCache, FontSystem};

/// A Rasterizer implementation that uses the `swash` engine.
/// 
/// This default implementation leverages `cosmic-text`'s SwashCache, 
/// but wraps it to satisfy the Zentype Rasterizer trait.
pub struct SwashRasterizer {
    cache: SwashCache,
    // Note: We maintain a local FontSystem here to allow the Rasterizer to be 
    // standalone for simple cases. In high-performance rendering (Phase 6+), 
    // this will likely be shared.
    font_system: FontSystem,
}

impl SwashRasterizer {
    /// Creates a new SwashRasterizer.
    pub fn new() -> Self {
        Self {
            cache: SwashCache::new(),
            font_system: FontSystem::new(),
        }
    }

    /// Access the internal font system (to synchronize fonts).
    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }
}

impl Default for SwashRasterizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Rasterizer for SwashRasterizer {
    fn rasterize(&mut self, glyph: &ShapedGlyph) -> Option<RasterizedGlyph> {
        // Use the cache to get the pixel data
        let image = self.cache.get_image(&mut self.font_system, glyph.key).as_ref()?;
        
        Some(RasterizedGlyph {
            width: image.placement.width,
            height: image.placement.height,
            data: image.data.clone(),
        })
    }
}
