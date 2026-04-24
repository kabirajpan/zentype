use std::sync::Arc;
use crate::prelude::*;
use crate::traits::font_provider::FontProvider;
use crate::traits::rasterizer::Rasterizer;
use crate::primitives::atlas::GlyphAtlas;
use crate::primitives::pipeline::ZentypePipeline;
use crate::types::glyph::GlyphInstance;
use wgpu::util::DeviceExt;

/// A managed renderer for Zentype that handles atlas management,
/// pipeline state, and efficient batching.
pub struct TextRenderer {
    font_provider: Box<dyn FontProvider>,
    rasterizer: Box<dyn Rasterizer>,
    atlas: GlyphAtlas,
    pipeline: ZentypePipeline,
    
    // GPU Resources
    instance_buffer: Option<wgpu::Buffer>,
    instances: Vec<GlyphInstance>,
    device: Arc<wgpu::Device>,
    
    // Metrics
    screen_size: [f32; 2],
}

impl TextRenderer {
    /// Creates a new TextRenderer with the provided engine implementations.
    pub fn new(
        device: Arc<wgpu::Device>, 
        queue: &wgpu::Queue, 
        config: &wgpu::SurfaceConfiguration,
        font_provider: Box<dyn FontProvider>,
        rasterizer: Box<dyn Rasterizer>,
        atlas: GlyphAtlas,
    ) -> Self {
        let pipeline = ZentypePipeline::new(&device, &atlas, config);
        
        let screen_size = [config.width as f32, config.height as f32];
        pipeline.update_screen_size(queue, screen_size[0], screen_size[1]);

        Self {
            font_provider,
            rasterizer,
            atlas,
            pipeline,
            instance_buffer: None,
            instances: Vec::new(),
            device,
            screen_size,
        }
    }

    /// Sets a custom font provider.
    pub fn set_font_provider(&mut self, provider: Box<dyn FontProvider>) {
        self.font_provider = provider;
    }

    /// Sets a custom rasterizer.
    pub fn set_rasterizer(&mut self, rasterizer: Box<dyn Rasterizer>) {
        self.rasterizer = rasterizer;
    }

    /// Prepares a string for rendering in the current frame at the specified position.
    /// This will shape the text and ensure all required glyphs are in the atlas.
    /// Returns the ShapedBuffer which can be used for hit-testing and interactivity.
    pub fn draw(&mut self, queue: &wgpu::Queue, text: &str, pos: [f32; 2], options: &TextOptions) -> ShapedBuffer {
        // 1. Ensure the shaper knows the available width for alignment/wrapping
        let layout_width = options.max_width.unwrap_or(self.screen_size[0] - pos[0]) - options.padding.left - options.padding.right;
        let layout_height = self.screen_size[1] - pos[1] - options.padding.top - options.padding.bottom;

        self.font_provider.set_layout_size(layout_width, layout_height);
        
        // --- Handle wrapping within the padded area ---
        let mut final_options = options.clone();
        if final_options.max_width.is_none() {
            // Default to the calculated layout width
            final_options.max_width = Some(layout_width);
        }
        
        // 2. Shape text with the final bounding options
        let buffer = self.font_provider.shape(text, &final_options);

        // 3. Ensure glyphs are in atlas
        for glyph in buffer.glyphs() {
            if self.atlas.get(&glyph.key).is_none() {
                // Rasterize on demand
                if let Some(rasterized) = self.rasterizer.rasterize(glyph) {
                    self.atlas.insert(queue, glyph.key, &rasterized);
                }
            }
        }

        // 4. Calculate vertical alignment offset
        let y_offset = self.calculate_valign_offset(&buffer, pos, &final_options);

        // 5. Generate instances with the provided offset
        let mut render_pos = pos;
        render_pos[1] += y_offset;

        let mut new_instances = self.pipeline.generate_instances(&buffer, &self.atlas, render_pos, &final_options);
        self.instances.append(&mut new_instances);

        buffer
    }

    fn calculate_valign_offset(&self, buffer: &ShapedBuffer, pos: [f32; 2], options: &TextOptions) -> f32 {
        if let Some(valign) = options.valign {
            let (_, content_height) = buffer.content_size();
            let max_h = options.max_height.unwrap_or(self.screen_size[1] - pos[1]);
            let available_height = max_h;
            
            match valign {
                VerticalAlignment::Top => 0.0,
                VerticalAlignment::Center => (available_height - content_height) / 2.0,
                VerticalAlignment::Bottom => available_height - content_height,
            }
        } else {
            0.0
        }
    }

    /// Finds the character index at the given screen-space coordinates.
    /// This handles the translation from screen space to text-local space, 
    /// accounting for position and padding.
    pub fn hit_test(&self, buffer: &ShapedBuffer, pos: [f32; 2], options: &TextOptions, mouse_pos: [f32; 2]) -> usize {
        let padding = options.padding;
        let y_offset = self.calculate_valign_offset(buffer, pos, options);
        
        let x = mouse_pos[0] - pos[0] - padding.left; 
        let y = mouse_pos[1] - pos[1] - padding.top - y_offset; 
        
        buffer.index_at(x, y)
    }

    /// Returns the screen-space position for a given character index.
    pub fn position_at(&self, buffer: &ShapedBuffer, pos: [f32; 2], options: &TextOptions, index: usize) -> Option<[f32; 2]> {
        let padding = options.padding;
        let y_offset = self.calculate_valign_offset(buffer, pos, options);
        
        buffer.position_at(index).map(|(lx, ly)| {
            [
                lx + pos[0] + padding.left,
                ly + pos[1] + padding.top + y_offset,
            ]
        })
    }

    /// Submits the accumulated text instances to the provided RenderPass.
    pub fn render<'a>(&'a mut self, rpass: &mut wgpu::RenderPass<'a>) {
        if self.instances.is_empty() {
            return;
        }

        // 1. Update instance buffer if necessary
        let required_size = (self.instances.len() * std::mem::size_of::<GlyphInstance>()) as wgpu::BufferAddress;
        
        // Reallocate if null or too small
        let recreate = match &self.instance_buffer {
            Some(buf) => buf.size() < required_size,
            None => true,
        };

        if recreate {
            self.instance_buffer = Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Zentype Instance Buffer"),
                contents: bytemuck::cast_slice(&self.instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }));
        } else if let Some(_buf) = &self.instance_buffer {
            // In a real high-performance app, we would use a staging buffer or queue.write_buffer
            // but for Level 2 default, we prioritize simplicity until Phase 10.
            // Actually, we can't write to a vertex buffer easily without a queue here.
            // Let's just recreate for now (simple) and optimize in Phase 8.
            self.instance_buffer = Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Zentype Instance Buffer"),
                contents: bytemuck::cast_slice(&self.instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }));
        }

        // 2. Perform the draw call
        if let Some(buf) = &self.instance_buffer {
            rpass.set_pipeline(self.pipeline.pipeline());
            rpass.set_bind_group(0, self.pipeline.atlas_bind_group(), &[]);
            rpass.set_bind_group(1, self.pipeline.uniform_bind_group(), &[]);
            rpass.set_vertex_buffer(0, buf.slice(..));
            rpass.draw(0..4, 0..self.instances.len() as u32);
        }

        // 3. Clear instances for the next frame
        self.instances.clear();
    }

    /// Updates the projection matrix to match the new surface size.
    pub fn resize(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.screen_size = [width as f32, height as f32];
        self.pipeline.update_screen_size(queue, self.screen_size[0], self.screen_size[1]);
    }
}
