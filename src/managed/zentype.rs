use crate::renderer::TextRenderer;
use crate::defaults::cosmic_font::CosmicFontProvider;
use crate::defaults::swash_raster::SwashRasterizer;

use crate::types::options::TextOptions;
use crate::types::color::Color;
use crate::primitives::shaped_buffer::ShapedBuffer;


/// The "Zero-Config" entry point for the Zentype engine.
/// 
/// This managed API handles the entire lifecycle of font shaping, 
/// rasterization, and GPU atlas management internally.
pub struct Zentype {
    renderer: TextRenderer,
}

impl Zentype {
    /// Creates a new Zentype instance with all default engines enabled.
    pub fn new(device: std::sync::Arc<wgpu::Device>, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) -> Self {
        // Initialize default engines
        let shaper = Box::new(CosmicFontProvider::new());
        let rasterizer = Box::new(SwashRasterizer::new());
        let atlas = crate::primitives::atlas::GlyphAtlas::new(&device, 2048);
        
        // Build the managed renderer
        let renderer = TextRenderer::new(device, queue, config, shaper, rasterizer, atlas);

        
        Self { renderer }
    }


    /// Prepares text for drawing in the current frame at the specified position.
    /// Returns the ShapedBuffer for interactivity.
    pub fn draw(&mut self, queue: &wgpu::Queue, text: &str, pos: [f32; 2], options: &TextOptions) -> ShapedBuffer {
        self.renderer.draw(queue, text, pos, options)
    }

    /// A convenience method for printing simple text labels.
    pub fn print(&mut self, queue: &wgpu::Queue, text: &str, pos: [f32; 2], size: f32, color: Color) -> ShapedBuffer {
        let options = TextOptions::new()
            .font_size(size)
            .color(color);
        
        self.renderer.draw(queue, text, pos, &options)
    }

    /// Finds the character index at the given screen-space coordinates for a specific buffer.
    pub fn hit_test(&self, buffer: &ShapedBuffer, pos: [f32; 2], options: &TextOptions, mouse_pos: [f32; 2]) -> usize {
        self.renderer.hit_test(buffer, pos, options, mouse_pos)
    }

    /// Returns the screen-space position for a given character index in a specific buffer.
    pub fn position_at(&self, buffer: &ShapedBuffer, pos: [f32; 2], options: &TextOptions, index: usize) -> Option<[f32; 2]> {
        self.renderer.position_at(buffer, pos, options, index)
    }



    /// Resizes the engine's projection to match the window dimensions.
    pub fn resize(&mut self, queue: &wgpu::Queue, width: u32, height: u32) {
        self.renderer.resize(queue, width, height);
    }

    /// Renders all accumulated text instances to the provided RenderPass.
    pub fn render<'a>(&'a mut self, rpass: &mut wgpu::RenderPass<'a>) {
        self.renderer.render(rpass);
    }

    /// Access the underlying renderer for advanced usage.
    pub fn renderer_mut(&mut self) -> &mut TextRenderer {
        &mut self.renderer
    }
}
