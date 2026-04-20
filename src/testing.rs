use crate::primitives::{GlyphAtlas, ZentypePipeline as TextPipeline};
use crate::types::glyph::GlyphInstance;
use cosmic_text::{Buffer, FontSystem, Metrics, SwashCache};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use crate::types::options::VerticalAlignment;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

/// A utility for quickly creating visual tests with Zentype.
/// It handles all the winit and wgpu boilerplate.
pub struct VisualTester;

impl VisualTester {
    /// Launches a window and runs the provided callback to set up the text buffer.
    /// The callback receives a `&mut FontSystem` and `&mut Buffer`.
    pub fn run<F>(callback: F)
    where
        F: FnMut(&mut FontSystem, &mut Buffer) + 'static,
    {
        let event_loop = EventLoop::new().unwrap();
        let mut app = App {
            state: None,
            callback: Box::new(callback),
        };
        let _ = event_loop.run_app(&mut app);
    }
}

struct App {
    state: Option<AppState>,
    callback: Box<dyn FnMut(&mut FontSystem, &mut Buffer)>,
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
    async fn new(
        window: Arc<Window>,
        mut callback: Box<dyn FnMut(&mut FontSystem, &mut Buffer)>,
    ) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
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
        buffer.set_size(
            &mut font_system,
            Some(size.width as f32),
            Some(size.height as f32),
        );

        // Execute the user provided callback
        callback(&mut font_system, &mut buffer);

        // Safe Sticky Alignment: Only re-apply if the user explicitly set a global alignment
        let options = crate::types::options::get_last_applied();
        
        // RE-SET SIZE with padding insets if necessary
        let padding = options.padding;
        buffer.set_size(
            &mut font_system,
            Some(size.width as f32 - padding.left - padding.right),
            Some(size.height as f32 - padding.top - padding.bottom),
        );

        // Shape first, THEN apply alignment once glyphs are measured
        buffer.shape_until_scroll(&mut font_system, false);

        if let Some(alignment) = options.align {
            let align: cosmic_text::Align = alignment.into();
            for line in buffer.lines.iter_mut() {
                line.set_align(Some(align));
            }
            // Re-shape once more to reflect alignment positions
            buffer.shape_until_scroll(&mut font_system, false);
        }

        Self {
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
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.pipeline.update_screen_size(&self.queue, new_size.width as f32, new_size.height as f32);
            
            // Re-set the buffer size to match new window dimensions (accounting for padding)
            let options = crate::types::options::get_last_applied();
            let padding = options.padding;
            self.buffer.set_size(
                &mut self.font_system, 
                Some(new_size.width as f32 - padding.left - padding.right), 
                Some(new_size.height as f32 - padding.top - padding.bottom)
            );
        }
    }

    fn render(&mut self) {
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) => t,
            _ => return,
        };
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let mut instances = Vec::new();
        let options = crate::types::options::get_last_applied();

        // 1. Calculate Vertical Alignment & Safe Areas
        let metrics = self.buffer.metrics();
        let padding = options.padding;
        let safe_height = self.config.height as f32 - padding.top - padding.bottom;
        
        // Calculate the TRUE visual height of the highlight block
        let mut visual_height = 0.0;
        if let Some(last_run) = self.buffer.layout_runs().last() {
            let font_size = metrics.font_size as f32;
            let line_height = metrics.line_height as f32;
            let visual_center_offset = font_size * 0.4;
            
            // The bottom-most coordinate of our green box
            let bg_bottom = last_run.line_y + (line_height / 2.0) - visual_center_offset + padding.bottom;
            visual_height = bg_bottom; 
        }
        
        // Determine vertical start (relative to window top)
        let valign = options.valign.unwrap_or(VerticalAlignment::Top);
        let v_offset = match valign {
            VerticalAlignment::Top => padding.top,
            VerticalAlignment::Center => padding.top + (safe_height - visual_height).max(0.0) / 2.0,
            VerticalAlignment::Bottom => (self.config.height as f32 - visual_height).max(0.0),
        };

        // 2. Render Backgrounds first
        if let Some(bg_color) = options.bg_color {
            for run in self.buffer.layout_runs() {
                // Get the horizontal start of the text in this run
                let line_x = run.glyphs.first().map(|g| g.x).unwrap_or(0.0);
                
                // Calculate bounds (relative to window)
                let width = if options.full_width_bg { self.config.width as f32 } else { run.line_w + padding.left + padding.right };
                
                // pos.x needs an extra 'padding' offset because the whole line is inset
                let final_x = if options.full_width_bg { 0.0 } else { line_x };
                
                // Background instance (UV size 0 means solid color)
                let line_height = metrics.line_height as f32;
                let font_size = metrics.font_size as f32;
                
                // We center the highlight vertically around the text's visual center
                // Visual center is approx (ascent - descent)/2 above the baseline.
                let visual_center_offset = font_size * 0.4; 
                let y_top = v_offset + run.line_y as f32 - visual_center_offset - (line_height / 2.0) - padding.top;

                instances.push(GlyphInstance {
                    pos: [final_x, y_top],
                    size: [width, (line_height + padding.top + padding.bottom)],
                    uv_pos: [0.0, 0.0],
                    uv_size: [0.0, 0.0],
                    color: [0.0, 0.0, 0.0, 0.0],
                    bg_color: bg_color.into(),
                });
            }
        }

        // 3. Render Glyphs
        for run in self.buffer.layout_runs() {
            let padding = options.padding;
            for glyph in run.glyphs {
                let physical_glyph = glyph.physical((padding.left, v_offset), 1.0);
                let image = match self
                    .swash_cache
                    .get_image(&mut self.font_system, physical_glyph.cache_key)
                {
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
                        &image.data,
                    )
                };

                let color = glyph
                    .color_opt
                    .unwrap_or(cosmic_text::Color::rgb(255, 255, 255));

                instances.push(GlyphInstance {
                    pos: [
                        (physical_glyph.x + image.placement.left) as f32,
                        (run.line_y as i32 + physical_glyph.y - image.placement.top) as f32,
                    ],
                    size: [image.placement.width as f32, image.placement.height as f32],
                    uv_pos: uv.uv_pos,
                    uv_size: uv.uv_size,
                    color: [
                        color.r() as f32 / 255.0,
                        color.g() as f32 / 255.0,
                        color.b() as f32 / 255.0,
                        color.a() as f32 / 255.0,
                    ],
                    bg_color: [0.0, 0.0, 0.0, 0.0],
                });
            }
        }

        if instances.is_empty() {
            // Nothing to render, but we still want to clear the screen
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.01, g: 0.01, b: 0.01, a: 1.0 }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    ..Default::default()
                });
            }
            self.queue.submit(std::iter::once(encoder.finish()));
            surface_texture.present();
            return;
        }

        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX,
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });

            render_pass.set_pipeline(self.pipeline.pipeline());
            render_pass.set_bind_group(0, self.pipeline.atlas_bind_group(), &[]);
            render_pass.set_bind_group(1, self.pipeline.uniform_bind_group(), &[]);
            render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
            render_pass.draw(0..4, 0..instances.len() as u32);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_title("Zentype Visual Tester"))
                .unwrap(),
        );
        // We need to take the callback here and pass it to AppState
        // Since we can't easily move Box<dyn FnMut> into an async block multiple times,
        // we assume resumed is called only once.
        let callback = std::mem::replace(&mut self.callback, Box::new(|_, _| {}));
        self.state = Some(pollster::block_on(AppState::new(window, callback)));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(state) = self.state.as_mut() {
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    state.resize(state.window.inner_size());
                }
                WindowEvent::RedrawRequested => {
                    state.render();
                    state.window.request_redraw();
                }
                _ => {
                    state.window.request_redraw();
                }
            }
        }
    }
}
