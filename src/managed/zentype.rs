use crate::renderer::TextRenderer;
use crate::defaults::cosmic_font::CosmicFontProvider;
use crate::defaults::swash_raster::SwashRasterizer;

use crate::types::options::TextOptions;
use crate::types::color::Color;

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
    pub fn draw(&mut self, queue: &wgpu::Queue, text: &str, pos: [f32; 2], options: &TextOptions) {
        self.renderer.draw(queue, text, pos, options);
    }

    /// A convenience method for printing simple text labels without manual options.
    pub fn print(&mut self, queue: &wgpu::Queue, text: &str, pos: [f32; 2], size: f32, color: Color) {
        let options = TextOptions::new()
            .font_size(size)
            .color(color);
        
        self.renderer.draw(queue, text, pos, &options);
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
