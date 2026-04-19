use std::collections::HashMap;
use cosmic_text::CacheKey;
use etagere::{AtlasAllocator, size2};

pub struct GlyphAtlas {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    allocator: AtlasAllocator,
    cached: HashMap<CacheKey, (f32, f32, f32, f32)>, // x, y, w, h in UV coords
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
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
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

    pub fn get(&self, key: &CacheKey) -> Option<&(f32, f32, f32, f32)> {
        self.cached.get(key)
    }

    pub fn insert(&mut self, queue: &wgpu::Queue, key: CacheKey, width: u32, height: u32, data: &[u8]) -> (f32, f32, f32, f32) {
        if width == 0 || height == 0 {
             return (0.0, 0.0, 0.0, 0.0);
        }

        let allocation = self.allocator.allocate(size2(width as i32, height as i32))
            .expect("Atlas out of space");
        
        let rect = allocation.rectangle;
        
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: rect.min.x as u32,
                    y: rect.min.y as u32,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            data,
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

        let uv = (
            rect.min.x as f32 / self.size as f32,
            rect.min.y as f32 / self.size as f32,
            width as f32 / self.size as f32,
            height as f32 / self.size as f32,
        );
        
        self.cached.insert(key, uv);
        uv
    }
}
