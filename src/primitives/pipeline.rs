use crate::primitives::atlas::GlyphAtlas;

pub struct ZentypePipeline {
    pub(crate) inner: crate::gpu::pipeline::TextPipeline,
}

impl ZentypePipeline {
    pub fn new(
        device: &wgpu::Device,
        atlas: &GlyphAtlas,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
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

    /// Generates raw GPU instance data for a shaped buffer at a specific position.
    pub fn generate_instances(
        &self,
        buffer: &crate::primitives::shaped_buffer::ShapedBuffer,
        atlas: &GlyphAtlas,
        pos: [f32; 2],
        options: &crate::types::options::TextOptions,
    ) -> Vec<crate::types::glyph::GlyphInstance> {
        // We pre-allocate space for backgrounds and glyphs
        let has_bg = options.bg_color.map(|c| c.a > 0).unwrap_or(false);

        let bg_count = if has_bg { buffer.lines().len() } else { 0 };
        let mut instances = Vec::with_capacity(buffer.len() + bg_count);

        // --- 1. AUTOMATIC BACKGROUND GENERATION ---
        if has_bg {
            let padding = options.padding;
            let bg_color = options.bg_color.unwrap();

            for line in buffer.lines() {
                let width = if options.full_width_bg {
                    options.max_width.map(|w| w + padding.left + padding.right)
                        .unwrap_or(line.width + padding.left + padding.right)
                } else {
                    line.width + padding.left + padding.right
                };

                let line_x = if options.full_width_bg {
                    pos[0]
                } else {
                    pos[0] + line.x
                };

                // --- THE "SOLID BLOCK" GAP-FREE ALIGNMENT MATH ---
                let baseline = pos[1] + line.y;
                let font_size = options.font_size;
                let lh = options.line_height;

                // We use the full line height to ensure consecutive lines touch (no gaps).
                let box_height = font_size * lh;

                // Centering: split the extra line-height space above and below.
                // Standard font box is ~1.0 total (0.8 up, 0.2 down).
                // Extra space is (lh - 1.0).
                let visual_ascent = font_size * (0.8 + (lh - 1.0) / 2.0);

                let final_y_top = baseline - visual_ascent;
                let final_height = box_height + padding.top + padding.bottom;

                instances.push(crate::types::glyph::GlyphInstance {
                    // Background starts exactly at line_x, spanning the required width.
                    // Previous version was subtracting padding.left from x, which was wrong.
                    pos: [line_x, final_y_top],
                    size: [width, final_height],

                    uv_pos: [0.0, 0.0],
                    uv_size: [0.0, 0.0],
                    color: [0.0, 0.0, 0.0, 0.0],
                    bg_color: bg_color.into(),
                });
            }
        }

        // --- 2. GLYPH RENDERING ---
        let color = options.color;
        for glyph in buffer.glyphs() {
            if let Some(entry) = atlas.get(&glyph.key) {
                // Precise rendering:
                // 1. pos uses the logical glyph position + the physical raster offset.
                // 2. size uses the exact pixel dimensions from the atlas entries.
                // 3. Subtract top offset from y because cosmic-text's top is positive UP from baseline.
                instances.push(crate::types::glyph::GlyphInstance {
                    pos: [
                        pos[0] + glyph.x + entry.pixel_offset[0] + options.padding.left,
                        pos[1] + glyph.y - entry.pixel_offset[1] + options.padding.top,
                    ],
                    size: entry.pixel_size,
                    uv_pos: entry.uv_pos,
                    uv_size: entry.uv_size,
                    color: color.into(),
                    bg_color: [0.0, 0.0, 0.0, 0.0],
                });
            }
        }

        instances
    }

    /// Records the commands to draw a shaped buffer using the provided atlas and instance buffer.
    pub fn draw_buffer<'a>(
        &'a self,
        rpass: &mut wgpu::RenderPass<'a>,
        buffer: &crate::primitives::shaped_buffer::ShapedBuffer,
        _atlas: &GlyphAtlas, // Atlas is already bound inside the pipeline, but we keep it in the API for safety/consistency
        instance_buffer: &'a wgpu::Buffer,
    ) {
        self.inner.draw(rpass, instance_buffer, buffer.len() as u32);
    }
}
