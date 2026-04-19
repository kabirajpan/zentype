# Zentype — Roadmap & Checklist

Full development plan for Zentype from current state to stable library release.

---

## 🔴 Phase 0 — Housekeeping (Do First)

These are blocking tasks before any real work begins.

- [ ] Rename project from `text_engine` to `zentype` in `Cargo.toml`
- [ ] Update `README.md` — remove all "Zenthra" references
- [ ] Reserve `zentype` name on [crates.io](https://crates.io) with an empty publish
- [ ] Set up `.gitignore` properly
- [ ] Create initial git tag `v0.0.1-pre`

---

## 🟡 Phase 1 — Folder Restructure

Reorganize `src/` from binary layout to library layout.

- [x] Create `src/lib.rs` as the crate root
- [x] Create `src/prelude.rs`
- [x] Create `src/types/` module
  - [x] `color.rs` — `Color` struct
  - [x] `options.rs` — `TextOptions` struct
  - [x] `glyph.rs` — `GlyphKey`, `AtlasEntry`, `RasterizedGlyph`
- [x] Create `src/traits/` module
  - [x] `font_provider.rs` — `FontProvider` trait
  - [x] `rasterizer.rs` — `Rasterizer` trait
  - [x] `atlas.rs` — `Atlas` trait
- [x] Move `src/gpu/atlas.rs` → split into `src/gpu/atlas.rs` + `src/primitives/atlas.rs`
- [x] Move `src/gpu/pipeline.rs` → split into `src/gpu/pipeline.rs` + `src/primitives/pipeline.rs`
- [x] Create `src/primitives/shaped_buffer.rs`
- [x] Create `src/defaults/` module
  - [x] `cosmic_font.rs` — `CosmicFontProvider`
  - [x] `swash_raster.rs` — `SwashRasterizer`
  - [x] `zentype_atlas.rs` — `ZentypeAtlas`
- [x] Create `src/renderer/text_renderer.rs` — `TextRenderer` (Level 2)
- [x] Create `src/managed/zentype.rs` — `Zentype` (Level 1)
- [x] Slim down `src/main.rs` to a thin demo that uses `lib.rs`

---

## 🟢 Phase 2 — Core Types

Build the shared types everything else depends on.

- [x] Implement `Color`
  - [x] `Color::rgb(r, g, b)`
  - [x] `Color::rgba(r, g, b, a)`
  - [x] `Color::hex(str)`
  - [x] `Color::WHITE`, `Color::BLACK`, `Color::TRANSPARENT` constants
- [x] Implement `TextOptions` with `Default`
  - [x] `x`, `y` position
  - [x] `font_size`, `color`, `font_family`, `font_weight`
  - [x] `bg_color`, `bg_padding`, `full_width_bg`
  - [x] `max_width`, `line_height`, `wrap`
- [x] Implement `FontWeight` enum (`Thin`, `Regular`, `Medium`, `Bold`, `Black`)
- [x] Implement `TextWrap` enum (`Word`, `Character`, `None`)
- [x] Implement `GlyphKey` (hash-friendly, used as atlas cache key)
- [x] Implement `AtlasEntry` (UV coords + size on texture)
- [x] Implement `RasterizedGlyph` (pixel data + metrics)

---

## 🔵 Phase 3 — Traits

Define the swappable interfaces.

- [ ] Define `FontProvider` trait
  - [ ] `fn shape(&mut self, text: &str, attrs: &Attrs) -> ShapedBuffer`
  - [ ] `fn load_font(&mut self, data: &[u8])`
- [ ] Define `Rasterizer` trait
  - [ ] `fn rasterize(&mut self, glyph: &ShapedGlyph) -> RasterizedGlyph`
- [ ] Define `Atlas` trait
  - [ ] `fn get_or_insert(&mut self, key: GlyphKey, data: &[u8]) -> AtlasEntry`
  - [ ] `fn texture(&self) -> &wgpu::Texture`
  - [ ] `fn flush(&mut self, queue: &wgpu::Queue)`
- [ ] All traits must be `Send + Sync`
- [ ] Write trait documentation with usage examples

---

## 🟣 Phase 4 — Default Implementations

Implement Zentype's built-in defaults for every trait.

- [ ] `CosmicFontProvider` implements `FontProvider`
  - [ ] wraps `cosmic-text` `FontSystem`
  - [ ] supports font loading from bytes
  - [ ] supports font loading from file path
- [ ] `SwashRasterizer` implements `Rasterizer`
  - [ ] wraps `swash` `ScaleContext`
  - [ ] outputs alpha mask for text glyphs
  - [ ] outputs RGBA for color emoji
- [ ] `ZentypeAtlas` implements `Atlas`
  - [ ] uses `etagere` for allocation
  - [ ] bucketed by glyph size for efficiency
  - [ ] auto-grows texture when full
  - [ ] dirty region tracking for partial GPU uploads

---

## ⚪ Phase 5 — Raw Primitives (Level 3)

Build the lowest-level public API.

- [ ] `ShapedBuffer`
  - [ ] `ShapedBuffer::new(font_system, text, options)`
  - [ ] `fn glyphs(&self) -> &[ShapedGlyph]`
  - [ ] `fn size(&self) -> (f32, f32)`
- [ ] `GlyphAtlas` (public wrapper around `src/gpu/atlas.rs`)
  - [ ] `GlyphAtlas::new(&device)`
  - [ ] `fn get_or_insert(...) -> AtlasEntry`
  - [ ] `fn texture(&self) -> &wgpu::Texture`
  - [ ] `fn flush(&mut self, queue)`
- [ ] `ZentypePipeline` (public wrapper around `src/gpu/pipeline.rs`)
  - [ ] `ZentypePipeline::new(&device, &config)`
  - [ ] `fn render_buffer(render_pass, font_system, swash_cache, atlas, buffer, options)`
  - [ ] `fn set_resolution(&mut self, width, height)`

---

## 🔶 Phase 6 — TextRenderer (Level 2)

Build the semi-managed API on top of Level 3.

- [ ] `TextRenderer::from_device(&device, &queue, &config)`
- [ ] `TextRenderer::builder()` — returns `TextRendererBuilder`
- [ ] `TextRendererBuilder`
  - [ ] `.font_provider(impl FontProvider)`
  - [ ] `.rasterizer(impl Rasterizer)`
  - [ ] `.atlas(impl Atlas)`
  - [ ] `.build(&device, &queue, &config) -> TextRenderer`
- [ ] `fn draw(&mut self, text: &str, options: TextOptions)`
- [ ] `fn draw_buffer(&mut self, buffer: &Buffer, options: TextOptions)`
- [ ] `fn render(&mut self, render_pass: &mut wgpu::RenderPass)`
- [ ] `fn atlas(&self) -> &dyn Atlas`
- [ ] `fn atlas_mut(&mut self) -> &mut dyn Atlas`
- [ ] `fn font_provider(&self) -> &dyn FontProvider`
- [ ] `fn set_resolution(&mut self, width: u32, height: u32)`

---

## 🔷 Phase 7 — Zentype (Level 1)

Build the fully managed API on top of Level 2.

- [ ] `Zentype::new(&window) -> Self` (async)
- [ ] `fn draw(&mut self, text: &str, options: TextOptions)`
- [ ] `fn draw_buffer(&mut self, buffer: &Buffer, options: TextOptions)`
- [ ] `fn begin_frame(&mut self)`
- [ ] `fn render(&mut self)`
- [ ] `fn end_frame(&mut self)`
- [ ] `fn renderer(&self) -> &TextRenderer` (escape hatch)
- [ ] `fn renderer_mut(&mut self) -> &mut TextRenderer` (escape hatch)
- [ ] `fn set_resolution(&mut self, width: u32, height: u32)`
- [ ] `fn set_clear_color(&mut self, color: Color)`

---

## 🧪 Phase 8 — Testing

- [ ] Unit tests for `Color` parsing (`hex`, `rgb`, `rgba`)
- [ ] Unit tests for `TextOptions` defaults
- [ ] Unit tests for `GlyphKey` hashing (no collisions)
- [ ] Unit tests for `ZentypeAtlas` allocation and eviction
- [ ] Integration test — Level 1 renders without panicking
- [ ] Integration test — Level 2 renders with custom device
- [ ] Integration test — Level 3 full manual pipeline runs
- [ ] Integration test — custom `FontProvider` swapped in
- [ ] Integration test — custom `Atlas` swapped in
- [ ] Benchmark — single draw call with 1000 glyphs
- [ ] Benchmark — atlas cache hit rate under repeated draws

---

## 📖 Phase 9 — Documentation

- [ ] Crate-level docs in `lib.rs` with full usage example
- [ ] Doc comments on every public struct, trait, and method
- [ ] `prelude.rs` documented — explain what it re-exports and why
- [ ] `primitives/` module documented — warn about manual responsibility
- [ ] `traits/` module documented — explain the swapping pattern
- [ ] Examples folder
  - [ ] `examples/basic.rs` — Level 1 hello world
  - [ ] `examples/custom_device.rs` — Level 2 with existing wgpu
  - [ ] `examples/raw_pipeline.rs` — Level 3 full manual
  - [ ] `examples/custom_font.rs` — bring your own font provider
  - [ ] `examples/editor_highlight.rs` — full-width line highlight
- [ ] `cargo doc` builds without warnings

---

## 🚀 Phase 10 — Planned Features (Post v0.1)

- [ ] Text selection and cursor logic
- [ ] Syntax highlighting engine
- [ ] SDF / MSDF vector glyph rendering
- [ ] Subpixel antialiasing
- [ ] Emoji support (color glyphs)
- [ ] RTL text support (already in cosmic-text, expose properly)
- [ ] Vertical text layout
- [ ] WebGPU / WASM target support
- [ ] `no_std` compatibility investigation

---

## 🏁 v0.1 Release Checklist

Before publishing to crates.io:

- [ ] All Phase 0–7 tasks complete
- [ ] All Phase 8 tests passing
- [ ] All Phase 9 docs complete
- [ ] `cargo clippy` — zero warnings
- [ ] `cargo fmt` — fully formatted
- [ ] `cargo doc` — builds clean
- [ ] `cargo publish --dry-run` — passes
- [ ] `CHANGELOG.md` written
- [ ] `LICENSE` file added
- [ ] GitHub repo public
- [ ] Publish `zentype = "0.1.0"` to crates.io

---

*Built as part of the Zentype UI Framework suite.*