use std::sync::Arc;
use anyhow::Result;
use zentype::prelude::*;
use zentype::primitives::{GlyphAtlas, ZentypePipeline as TextPipeline};
use zentype::types::glyph::GlyphInstance;
use cosmic_text::{Attrs, FontSystem, Metrics, SwashCache, Buffer, Color};
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

struct App {
    state: Option<AppState>,
}

struct AppState {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: GlyphAtlas,
    pipeline: TextPipeline,
    
    buffer: Buffer,
}

impl AppState {
    async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.expect("Failed to find wgpu adapter");

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;
        
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let atlas = GlyphAtlas::new(&device, 2048);
        let pipeline = TextPipeline::new(&device, &atlas, &config);
        pipeline.update_screen_size(&queue, size.width as f32, size.height as f32);

        let mut buffer = Buffer::new(&mut font_system, Metrics::new(24.0, 32.0));
        buffer.set_size(&mut font_system, Some(size.width as f32), Some(size.height as f32));
        
        let long_para = "Text rendering is one of the hardest problems in UI systems. \
            Even browsers, game engines, and professional frameworks like Flutter or Skia \
            all have complex custom text engines to handle the nuances of shaping, \
            bidirectional text, font fallback, and sub-pixel positioning. By using cosmic-text \
            directly with a custom wgpu pipeline, we bypass the limitations of high-level \
            wrappers while maintaining absolute pixel-perfect control over every glyph. \
            This allows Zenthra to scale efficiently, batching thousands of characters into \
            minimal draw calls while ensuring typography remains crisp and premium.";

        buffer.set_text(&mut font_system, long_para, &Attrs::new(), cosmic_text::Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut font_system, false);

        Ok(Self {
            window,
            device,
            queue,
            surface,
            config,
            font_system,
            swash_cache,
            atlas,
            pipeline,
            buffer,
        })
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.pipeline.update_screen_size(&self.queue, new_size.width as f32, new_size.height as f32);
            self.buffer.set_size(&mut self.font_system, Some(new_size.width as f32), Some(new_size.height as f32));
            self.buffer.shape_until_scroll(&mut self.font_system, false);
        }
    }

    fn render(&mut self) -> Result<()> {
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) => t,
            _ => return Ok(()),
        };
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });

        let mut instances = Vec::new();
        let metrics = self.buffer.metrics();

        // Phase 1: Draw continuous background highlights per line
        for run in self.buffer.layout_runs() {
            instances.push(GlyphInstance {
                pos: [
                    0.0, // Start highlight from left
                    run.line_y - metrics.font_size, // Use font_size as proxy for ascent
                ],
                size: [
                    self.config.width as f32, // Fill the full window width
                    metrics.line_height,
                ],
                uv_pos: [0.0, 0.0],
                uv_size: [0.0, 0.0], // Trigger solid color mode in shader
                color: [1.0, 1.0, 1.0, 1.0], // Not used in solid mode
                bg_color: [0.1, 0.4, 0.1, 1.0], // Green highlight
            });
        }

        // Phase 2: Draw glyphs on top
        for run in self.buffer.layout_runs() {
            for glyph in run.glyphs {
                let physical_glyph = glyph.physical((0.0, 0.0), 1.0);
                
                let image = self.swash_cache.get_image(&mut self.font_system, physical_glyph.cache_key);
                let image = match image {
                    Some(img) => img,
                    None => continue,
                };

                let uv = if let Some(uv) = self.atlas.get(&physical_glyph.cache_key) {
                    uv
                } else {
                    self.atlas.insert(
                        &self.queue, 
                        physical_glyph.cache_key, 
                        image.placement.width, 
                        image.placement.height, 
                        &image.data
                    )
                };

                if uv.uv_size[0] == 0.0 || uv.uv_size[1] == 0.0 { continue; }

                let color = glyph.color_opt.unwrap_or(Color::rgb(255, 255, 255));
                
                instances.push(GlyphInstance {
                    pos: [
                        (physical_glyph.x + image.placement.left) as f32,
                        (run.line_y as i32 + physical_glyph.y - image.placement.top) as f32,
                    ],
                    size: [
                        image.placement.width as f32,
                        image.placement.height as f32,
                    ],
                    uv_pos: uv.uv_pos,
                    uv_size: uv.uv_size,
                    color: [
                        color.r() as f32 / 255.0,
                        color.g() as f32 / 255.0,
                        color.b() as f32 / 255.0,
                        color.a() as f32 / 255.0,
                    ],
                    bg_color: [0.0, 0.0, 0.0, 0.0], // Transparent for the glyph quads
                });
            }
        }

        let instance_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.01, g: 0.01, b: 0.01, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(self.pipeline.pipeline());
            render_pass.set_bind_group(0, self.pipeline.atlas_bind_group(), &[]);
            render_pass.set_bind_group(1, self.pipeline.uniform_bind_group(), &[]);
            render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
            render_pass.draw(0..4, 0..instances.len() as u32);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window = Arc::new(event_loop.create_window(WindowAttributes::default().with_title("Zenthra Text Engine")).unwrap());
            self.state = Some(pollster::block_on(AppState::new(window)).unwrap());
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: winit::window::WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::RedrawRequested => {
                state.render().unwrap();
                state.window.request_redraw();
            }
            _ => {
                 state.window.request_redraw();
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let event_loop = EventLoop::new()?;
    let mut app = App { state: None };
    event_loop.run_app(&mut app).map_err(Into::into)
}
