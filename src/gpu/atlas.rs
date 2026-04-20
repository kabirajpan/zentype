use std::collections::HashMap;
use cosmic_text::CacheKey;
use etagere::{AtlasAllocator, size2};

pub struct GlyphAtlas {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    allocator: AtlasAllocator,
    cached: HashMap<CacheKey, crate::types::glyph::AtlasEntry>,
    size: u32,
}


impl GlyphAtlas {
    pub fn new(device: &wgpu::Device, size: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glyph Atlas"),
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

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,

            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            allocator: AtlasAllocator::new(size2(size as i32, size as i32)),
            cached: HashMap::new(),
            size,
        }
    }

    pub fn get(&self, key: &CacheKey) -> Option<&crate::types::glyph::AtlasEntry> {
        self.cached.get(key)
    }


    pub fn insert(&mut self, queue: &wgpu::Queue, key: CacheKey, glyph: &crate::types::glyph::RasterizedGlyph) -> crate::types::glyph::AtlasEntry {
        let width = glyph.width;
        let height = glyph.height;

        if width == 0 || height == 0 {
             return crate::types::glyph::AtlasEntry {
                uv_pos: [0.0, 0.0],
                uv_size: [0.0, 0.0],
                pixel_size: [0.0, 0.0],
                pixel_offset: [0.0, 0.0],
             };
        }

        // Add 1px padding on all sides to prevent bleeding
        let pad = 1;
        let alloc_width = width as i32 + (pad * 2);
        let alloc_height = height as i32 + (pad * 2);

        let allocation = self.allocator.allocate(size2(alloc_width, alloc_height))
            .expect("Atlas out of space");
        
        let rect = allocation.rectangle;
        
        // The glyph itself starts at +1, +1 from the allocated rectangle
        let x = rect.min.x as u32 + pad as u32;
        let y = rect.min.y as u32 + pad as u32;

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            &glyph.data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let entry = crate::types::glyph::AtlasEntry {
            uv_pos: [
                x as f32 / self.size as f32,
                y as f32 / self.size as f32,
            ],
            uv_size: [
                width as f32 / self.size as f32,
                height as f32 / self.size as f32,
            ],
            pixel_size: [width as f32, height as f32],
            pixel_offset: [glyph.left as f32, glyph.top as f32],
        };

        
        self.cached.insert(key, entry);
        entry
    }



    /// Clears the atlas and resets the allocator.
    pub fn clear(&mut self) {
        self.allocator.clear();
        self.cached.clear();
    }
}

