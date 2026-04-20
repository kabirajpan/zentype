use crate::primitives::{GlyphAtlas, ZentypePipeline as TextPipeline};


use cosmic_text::{Buffer, FontSystem, Metrics, SwashCache};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use crate::types::options::VerticalAlignment;

use crate::renderer::TextRenderer;
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
            renderer_callback: None,
            zentype_callback: None,
        };

        let _ = event_loop.run_app(&mut app);
    }

    /// Level 2: Managed TextRenderer
    pub fn run_renderer<F>(callback: F)
    where
        F: FnMut(&mut TextRenderer, &wgpu::Queue) + 'static,
    {
        let event_loop = EventLoop::new().unwrap();
        let mut app = App {
            state: None,
            callback: Box::new(|_, _| {}), // Dummy
            renderer_callback: Some(Box::new(callback)),
            zentype_callback: None,
        };
        let _ = event_loop.run_app(&mut app);
    }

    /// Level 1: Managed Zentype entry point
    pub fn run_zentype<F>(callback: F)
    where
        F: FnMut(&mut crate::managed::zentype::Zentype, &wgpu::Queue) + 'static,
    {
        let event_loop = EventLoop::new().unwrap();
        let mut app = App {
            state: None,
            callback: Box::new(|_, _| {}), // Dummy
            renderer_callback: None,
            zentype_callback: Some(Box::new(callback)),
        };
        let _ = event_loop.run_app(&mut app);
    }
}



struct App {
    state: Option<AppState>,
    callback: Box<dyn FnMut(&mut FontSystem, &mut Buffer)>,
    renderer_callback: Option<Box<dyn FnMut(&mut TextRenderer, &wgpu::Queue)>>,
    zentype_callback: Option<Box<dyn FnMut(&mut crate::managed::zentype::Zentype, &wgpu::Queue)>>,
}



struct AppState {
    window: Arc<Window>,
    device: Arc<wgpu::Device>,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,

    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: GlyphAtlas,
    pipeline: Option<TextPipeline>,
    buffer: Buffer,

    renderer: Option<TextRenderer>,
    renderer_callback: Option<Box<dyn FnMut(&mut TextRenderer, &wgpu::Queue)>>,
    zentype: Option<crate::managed::zentype::Zentype>,
    zentype_callback: Option<Box<dyn FnMut(&mut crate::managed::zentype::Zentype, &wgpu::Queue)>>,
}




impl AppState {
    async fn new(
        window: Arc<Window>,
        mut callback: Box<dyn FnMut(&mut FontSystem, &mut Buffer)>,
        renderer_callback: Option<Box<dyn FnMut(&mut TextRenderer, &wgpu::Queue)>>,
        zentype_callback: Option<Box<dyn FnMut(&mut crate::managed::zentype::Zentype, &wgpu::Queue)>>,
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
        let device = Arc::new(device);

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

        let renderer = if renderer_callback.is_some() {
            Some(TextRenderer::new(device.clone(), &queue, &config, 
                Box::new(crate::defaults::cosmic_font::CosmicFontProvider::new()),
                Box::new(crate::defaults::swash_raster::SwashRasterizer::new()),
                crate::primitives::atlas::GlyphAtlas::new(&device, 2048)

            ))
        } else {
            None
        };


        let zentype = if zentype_callback.is_some() {
            Some(crate::managed::zentype::Zentype::new(device.clone(), &queue, &config))
        } else {
            None
        };


        Self {
            window,
            device,
            queue,
            surface,
            config,
            font_system,
            swash_cache,
            atlas,
            pipeline: Some(pipeline),
            buffer,
            renderer,
            renderer_callback,
            zentype,
            zentype_callback,
        }
    }



    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        if let Some(pipeline) = &self.pipeline {
            pipeline.update_screen_size(&self.queue, new_size.width as f32, new_size.height as f32);
        }
            
            // Inform the managed renderer about the resize
            if let Some(renderer) = &mut self.renderer {
                renderer.resize(&self.queue, new_size.width, new_size.height);
            }
            if let Some(zentype) = &mut self.zentype {
                zentype.resize(&self.queue, new_size.width, new_size.height);
            }

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

        if let (Some(zentype), Some(callback)) = (&mut self.zentype, &mut self.zentype_callback) {
            // --- LEVEL 1 MANAGED ZENTYPE PATH ---
            callback(zentype, &self.queue);
            
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                zentype.render(&mut rpass);
            }
            self.queue.submit(std::iter::once(encoder.finish()));
            surface_texture.present();
            return;
        }

        if let (Some(renderer), Some(callback)) = (&mut self.renderer, &mut self.renderer_callback) {
            // --- LEVEL 2 MANAGED RENDERER PATH ---

            callback(renderer, &self.queue);
            
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                renderer.render(&mut rpass);
            }
            self.queue.submit(std::iter::once(encoder.finish()));
            surface_texture.present();
            return;
        }

        // 1. Calculate Vertical Alignment & Safe Areas
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

        // 2. Retrieve the options that were applied by the user in their example
        let options = crate::types::options::get_last_applied();
        let padding = options.padding;

        // --- RASTERIZATION PRE-PASS ---
        // We must ensure all glyphs are in the atlas BEFORE we generate instances.
        for run in self.buffer.layout_runs() {
            for glyph in run.glyphs {
                let physical = glyph.physical((0.0, 0.0), 1.0);
                if self.atlas.get(&physical.cache_key).is_none() {
                    // Rasterize on the fly for the test tool
                    if let Some(image) = self.swash_cache.get_image(&mut self.font_system, physical.cache_key) {
                        self.atlas.insert(
                            &self.queue,
                            physical.cache_key,
                            &crate::types::glyph::RasterizedGlyph {
                                width: image.placement.width,
                                height: image.placement.height,
                                left: image.placement.left,
                                top: image.placement.top,
                                data: image.data.clone(),
                            }
                        );
                    }
                }
            }
        }


        // 3. Convert the raw cosmic-text buffer into a ShapedBuffer for the core pipeline
        let mut shaped_glyphs = Vec::new();
        let mut lines = Vec::new();
        for run in self.buffer.layout_runs() {
            // Correctly calculate the alignment offset for the test tool's shaped buffer
            let layout_width = self.buffer.size().0.unwrap_or(run.line_w);
            let alignment_offset = match options.align {
                Some(crate::types::HorizontalAlignment::Center) => (layout_width - run.line_w) / 2.0,
                Some(crate::types::HorizontalAlignment::Right) => layout_width - run.line_w,
                _ => 0.0,
            };

            lines.push(crate::types::line::LineInfo {
                x: alignment_offset,
                y: run.line_y,
                width: run.line_w,
            });





            for glyph in run.glyphs {
                let physical = glyph.physical((0.0, 0.0), 1.0);
                shaped_glyphs.push(crate::types::shaped_glyph::ShapedGlyph {
                    key: physical.cache_key,
                    x: glyph.x,
                    y: run.line_y + glyph.y,
                    width: glyph.w,
                    height: 0.0,
                });
            }
        }
        let shaped_buffer = crate::primitives::shaped_buffer::ShapedBuffer::new(
            shaped_glyphs, 
            lines, 
            self.config.width as f32, 
            self.config.height as f32
        );

        // 4. Generate instances using the CORE pipeline
        // This automatically handles our "Safe & Tight" backgrounds!
        let instances: Vec<crate::types::glyph::GlyphInstance> = if let Some(pipeline) = &self.pipeline {
            pipeline.generate_instances(
                &shaped_buffer,
                &self.atlas,
                [padding.left, v_offset],
                &options,
            )
        } else {
            Vec::new()
        };

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

            if let Some(pipeline) = &self.pipeline {
                render_pass.set_pipeline(pipeline.pipeline());
                render_pass.set_bind_group(0, pipeline.atlas_bind_group(), &[]);
                render_pass.set_bind_group(1, pipeline.uniform_bind_group(), &[]);
            }

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
        let callback = std::mem::replace(&mut self.callback, Box::new(|_, _| {}));
        let renderer_callback = self.renderer_callback.take();
        let zentype_callback = self.zentype_callback.take();
        self.state = Some(pollster::block_on(AppState::new(window, callback, renderer_callback, zentype_callback)));
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
