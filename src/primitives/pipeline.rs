use crate::primitives::atlas::GlyphAtlas;

pub struct ZentypePipeline {
    pub(crate) inner: crate::gpu::pipeline::TextPipeline,
}

impl ZentypePipeline {
    pub fn new(device: &wgpu::Device, atlas: &GlyphAtlas, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            inner: crate::gpu::pipeline::TextPipeline::new(device, &atlas.inner, config),
        }
    }

    pub fn update_screen_size(&self, queue: &wgpu::Queue, width: f32, height: f32) {
        self.inner.update_screen_size(queue, width, height);
    }

    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.inner.pipeline
    }

    pub fn atlas_bind_group(&self) -> &wgpu::BindGroup {
        &self.inner.atlas_bind_group
    }

    pub fn uniform_bind_group(&self) -> &wgpu::BindGroup {
        &self.inner.uniform_bind_group
    }
}
