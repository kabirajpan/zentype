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
        self.inner.get(key).map(|&(x, y, w, h)| AtlasEntry {
            uv_pos: [x, y],
            uv_size: [w, h],
        })
    }

    pub fn insert(&mut self, queue: &wgpu::Queue, key: GlyphKey, width: u32, height: u32, data: &[u8]) -> AtlasEntry {
        let (x, y, w, h) = self.inner.insert(queue, key, width, height, data);
        AtlasEntry {
            uv_pos: [x, y],
            uv_size: [w, h],
        }
    }

    pub fn texture(&self) -> &wgpu::Texture {
        &self.inner.texture
    }
}
