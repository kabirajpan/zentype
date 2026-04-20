use crate::types::glyph::{GlyphKey, AtlasEntry};

pub struct GlyphAtlas {
    pub(crate) inner: crate::gpu::atlas::GlyphAtlas,
}

impl GlyphAtlas {
    pub fn new(device: &wgpu::Device, size: u32) -> Self {
        Self {
            inner: crate::gpu::atlas::GlyphAtlas::new(device, size),
        }
    }

    pub fn get(&self, key: &GlyphKey) -> Option<AtlasEntry> {
        self.inner.get(key).copied()
    }

    pub fn insert(&mut self, queue: &wgpu::Queue, key: GlyphKey, glyph: &crate::types::glyph::RasterizedGlyph) -> AtlasEntry {
        self.inner.insert(queue, key, glyph)
    }


    pub fn texture(&self) -> &wgpu::Texture {
        &self.inner.texture
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.inner.view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.inner.sampler
    }

    /// Clears the atlas and resets the allocator.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

