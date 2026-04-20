use std::collections::HashMap;
// Remove unused CacheKey
use etagere::{AtlasAllocator, size2};
use crate::traits::atlas::Atlas;
use crate::types::glyph::{GlyphKey, AtlasEntry};

/// A high-performance GPU texture atlas using the `etagere` allocator.
pub struct ZentypeAtlas {
    texture: wgpu::Texture,
    allocator: AtlasAllocator,
    cached: HashMap<GlyphKey, AtlasEntry>,
    _size: u32,
    /// Pending writes to the texture.
    pending_writes: Vec<PendingWrite>,
}

struct PendingWrite {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl ZentypeAtlas {
    /// Creates a new ZentypeAtlas with the given size (e.g. 2048).
    pub fn new(device: &wgpu::Device, size: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Zentype Glyph Atlas"),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Self {
            texture,
            allocator: AtlasAllocator::new(size2(size as i32, size as i32)),
            cached: HashMap::new(),
            _size: size,
            pending_writes: Vec::new(),
        }
    }
}

impl Atlas for ZentypeAtlas {
    fn get_or_insert(&mut self, key: GlyphKey, _data: &[u8]) -> AtlasEntry {
        // 1. Check if already cached
        if let Some(entry) = self.cached.get(&key) {
            return *entry;
        }

        // 2. We need width/height but data is just bytes. 
        // This default implementation assumes standard RGBA or Alpha-only 
        // depending on the size of the data. 
        // For Phase 4, we'll assume we know the glyph dimensions from the caller 
        // or just allocate enough space. 
        // Actually, the Rasterizer should probably provide the dimensions.
        
        // Placeholder implementation until we refine the get_or_insert call signature
        // in Step 4/5. 
        AtlasEntry {
            uv_pos: [0.0, 0.0],
            uv_size: [0.0, 0.0],
        }
    }

    fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    fn flush(&mut self, queue: &wgpu::Queue) {
        for write in self.pending_writes.drain(..) {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: write.x,
                        y: write.y,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &write.data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(write.width),
                    rows_per_image: Some(write.height),
                },
                wgpu::Extent3d {
                    width: write.width,
                    height: write.height,
                    depth_or_array_layers: 1,
                },
            );
        }
    }

    fn clear(&mut self) {
        self.allocator.clear();
        self.cached.clear();
        self.pending_writes.clear();
    }
}
