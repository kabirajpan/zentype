Here's the full API design:

---

### `TextOptions` — Shared Config
```rust
pub struct TextOptions {
    // position
    pub x: f32,
    pub y: f32,

    // text style
    pub font_size:  f32,
    pub color:      Color,
    pub font_family: Option<String>,
    pub font_weight: FontWeight,

    // background
    pub bg_color:   Option<Color>,
    pub bg_padding: f32,
    pub full_width_bg: bool,   // editor-style line highlight

    // layout
    pub max_width:  Option<f32>,
    pub line_height: f32,
    pub wrap:       TextWrap,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            font_size: 16.0,
            color: Color::WHITE,
            font_family: None,
            font_weight: FontWeight::Regular,
            bg_color: None,
            bg_padding: 4.0,
            full_width_bg: false,
            max_width: None,
            line_height: 1.5,
            wrap: TextWrap::Word,
        }
    }
}
```

---

### `Color` — Color Type
```rust
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self;
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self;
    pub fn hex(hex: &str) -> Self;

    // built in colors
    pub const WHITE: Color;
    pub const BLACK: Color;
    pub const TRANSPARENT: Color;
}
```

---

### Level 1 — `Zentype` Struct
```rust
pub struct Zentype { ... }

impl Zentype {
    // init
    pub async fn new(window: &Window) -> Self;

    // drawing
    pub fn draw(&mut self, text: &str, options: TextOptions);
    pub fn draw_buffer(&mut self, buffer: &Buffer, options: TextOptions);

    // rendering
    pub fn begin_frame(&mut self);
    pub fn render(&mut self);
    pub fn end_frame(&mut self);

    // escape hatches
    pub fn renderer(&self)     -> &TextRenderer;
    pub fn renderer_mut(&mut self) -> &mut TextRenderer;

    // config
    pub fn set_resolution(&mut self, width: u32, height: u32);
    pub fn set_clear_color(&mut self, color: Color);
}
```

---

### Level 2 — `TextRenderer` Struct
```rust
pub struct TextRenderer { ... }

impl TextRenderer {
    // init
    pub fn from_device(
        device: &wgpu::Device,
        queue:  &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self;

    // builder for custom components
    pub fn builder() -> TextRendererBuilder;

    // drawing
    pub fn draw(&mut self, text: &str, options: TextOptions);
    pub fn draw_buffer(&mut self, buffer: &Buffer, options: TextOptions);

    // rendering
    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass);

    // escape hatches
    pub fn atlas(&self)         -> &dyn Atlas;
    pub fn atlas_mut(&mut self) -> &mut dyn Atlas;
    pub fn font_provider(&self) -> &dyn FontProvider;

    // config
    pub fn set_resolution(&mut self, width: u32, height: u32);
}
```

---

### `TextRendererBuilder` — Mix & Match
```rust
pub struct TextRendererBuilder { ... }

impl TextRendererBuilder {
    pub fn font_provider<F: FontProvider>(self, f: F) -> Self;
    pub fn rasterizer<R: Rasterizer>(self, r: R)     -> Self;
    pub fn atlas<A: Atlas>(self, a: A)               -> Self;

    pub fn build(
        self,
        device: &wgpu::Device,
        queue:  &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> TextRenderer;
}
```

---

### Level 3 — Raw Primitives
```rust
// atlas
impl GlyphAtlas {
    pub fn new(device: &wgpu::Device) -> Self;
    pub fn get_or_insert(&mut self, key: GlyphKey, data: &[u8]) -> AtlasEntry;
    pub fn texture(&self) -> &wgpu::Texture;
    pub fn flush(&mut self, queue: &wgpu::Queue);
}

// pipeline
impl ZentypePipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self;
    pub fn render_buffer(
        &mut self,
        render_pass:  &mut wgpu::RenderPass,
        font_system:  &mut FontSystem,
        swash_cache:  &mut SwashCache,
        atlas:        &mut dyn Atlas,
        buffer:       &Buffer,
        options:      TextOptions,
    );
}

// shaped buffer
impl ShapedBuffer {
    pub fn new(font_system: &mut FontSystem, text: &str, options: &TextOptions) -> Self;
    pub fn glyphs(&self) -> &[ShapedGlyph];
    pub fn size(&self) -> (f32, f32);
}
```

---

### Traits
```rust
pub trait FontProvider: Send + Sync {
    fn shape(&mut self, text: &str, attrs: &Attrs) -> ShapedBuffer;
    fn load_font(&mut self, data: &[u8]);
}

pub trait Rasterizer: Send + Sync {
    fn rasterize(&mut self, glyph: &ShapedGlyph) -> RasterizedGlyph;
}

pub trait Atlas: Send + Sync {
    fn get_or_insert(&mut self, key: GlyphKey, data: &[u8]) -> AtlasEntry;
    fn texture(&self) -> &wgpu::Texture;
    fn flush(&mut self, queue: &wgpu::Queue);
}
```

---

### The Full Flow In Code
```rust
// level 1 user — 5 lines total
use zentype::prelude::*;

let mut zen = Zentype::new(&window).await;
zen.draw("Hello Zentype", TextOptions {
    font_size: 24.0,
    color: Color::WHITE,
    bg_color: Some(Color::rgb(30, 30, 30)),
    full_width_bg: true,
    ..Default::default()
});
zen.render();
```

---

This is the complete API surface. Clean, layered, and every piece is swappable.

