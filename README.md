# Zentype 🔤

A high-performance, modular text rendering engine for Rust — built on **wgpu**, **cosmic-text**, and **swash**.

Zentype is the rendering core you'd build *under* a text editor or UI framework. It gives you GPU-native text rendering with full control over every layer — no opinions, no restrictions, no high-level magic.

---

## ✨ Features

- **Layered API** — use the fully managed `Zentype` struct, the semi-managed `TextRenderer`, or raw GPU primitives. Pick your level of control.
- **Modular by design** — every core component (font provider, rasterizer, atlas) is a swappable trait. Bring your own, or use Zentype's battle-tested defaults.
- **Direct shaping control** — built on `cosmic-text` for full bi-directional text, font fallbacks, and complex script support.
- **Custom glyph atlas** — high-efficiency texture atlas powered by `etagere`. Glyphs are rasterized on-demand and cached on the GPU.
- **Instanced GPU pipeline** — thousands of characters rendered in a single draw call via an instanced `wgpu` pipeline.
- **Editor-style line highlights** — full-width background bars rendered natively in the shader. No hacks, no separate rectangles.
- **Modern stack** — built on WGPU v29 and Winit v0.30.

---

## 🏗️ Architecture

```
zentype::
├── prelude::*              → everything for level 1 users
├── Zentype                 → level 1 managed struct
├── TextRenderer            → level 2 semi managed
├── TextOptions             → shared config
├── Color                   → color type
├── primitives::
│   ├── GlyphAtlas          → raw atlas
│   ├── ZentypePipeline     → raw pipeline
│   └── ShapedBuffer        → raw glyph data
└── traits::
    ├── FontProvider        → swap your font system
    ├── Rasterizer          → swap your rasterizer
    └── Atlas               → swap your atlas
```

### How data flows

```
CPU Phase:   cosmic-text shapes text → swash rasterizes glyphs
Atlas Phase: pixels uploaded to GPU texture (etagere manages allocation)
GPU Phase:   shader renders instanced quads (text mode + solid highlight mode)
```

---

## 📦 API

### Level 1 — Full Managed

Zentype owns everything. Five lines to get text on screen.

```rust
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

### Level 2 — Semi Managed

You own the wgpu context. Zentype owns fonts, atlas, and pipeline.

```rust
use zentype::TextRenderer;
use zentype::TextOptions;

let mut renderer = TextRenderer::from_device(&device, &queue, &config);

renderer.draw("Hello Zentype", TextOptions::default());
renderer.render(&mut render_pass);
```

### Level 3 — Full Manual

You own everything. Zentype provides raw GPU primitives.

```rust
use zentype::primitives::{GlyphAtlas, ZentypePipeline, ShapedBuffer};

let mut atlas    = GlyphAtlas::new(&device);
let mut pipeline = ZentypePipeline::new(&device, &config);

pipeline.render_buffer(
    &mut render_pass,
    &mut font_system,
    &mut swash_cache,
    &mut atlas,
    &buffer,
    TextOptions::default(),
);
```

### Bring Your Own X

Swap any component. Use Zentype's defaults for everything else.

```rust
use zentype::TextRenderer;
use zentype::traits::{FontProvider, Rasterizer, Atlas};

let renderer = TextRenderer::builder()
    .font_provider(MyCustomFontProvider::new())  // yours
    .rasterizer(SwashRasterizer::new())          // zentype default
    .atlas(ZentypeAtlas::new(&device))           // zentype default
    .build(&device, &queue, &config);
```

### Escape Hatches

Start managed, drop down when you need control.

```rust
let mut zen = Zentype::new(&window).await;

// reach inside at any time
let renderer = zen.renderer_mut();        // level 2
let atlas    = renderer.atlas_mut();      // level 3
```

---

## 🚀 Getting Started

### Prerequisites

- Rust 1.80+
- A GPU supporting Vulkan, Metal, or DX12

### Add to your project

```toml
[dependencies]
zentype = "0.1"
```

### Run the demo

```bash
git clone https://github.com/your-username/zentype
cd zentype
cargo run --release
```

---

## 🛠️ Tech Stack

| Crate | Role |
|---|---|
| [wgpu](https://github.com/gfx-rs/wgpu) | WebGPU-based graphics API |
| [cosmic-text](https://github.com/pop-os/cosmic-text) | Text shaping and layout |
| [swash](https://github.com/dfrg/swash) | Glyph rasterization |
| [etagere](https://github.com/nical/etagere) | Texture atlas allocation |
| [winit](https://github.com/rust-windowing/winit) | Window handling (v0.30+) |

---

## ⚔️ How Zentype Compares

| Library | Level | Limitation |
|---|---|---|
| Glyphon | High | Too opinionated, restricts control |
| wgpu_text | High | Basic, no real layout control |
| ab_glyph | Low | No shaping (no RTL, no fallbacks) |
| Fontdue | Low | CPU only, no GPU pipeline |
| **Zentype** | **Any** | **None — you choose your level** |

---

## 📜 License

Apache 2.0 — see [LICENSE](LICENSE)

---

*Part of the Zentype UI Framework suite.*