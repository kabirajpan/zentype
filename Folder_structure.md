Here's the full folder structure:

---

```
zentype/
├── Cargo.toml
├── README.md
│
└── src/
    ├── lib.rs                  → public API, re-exports everything
    │
    ├── prelude.rs              → use zentype::prelude::*
    │
    ├── managed/
    │   ├── mod.rs              → pub use
    │   └── zentype.rs          → Zentype struct (level 1)
    │
    ├── renderer/
    │   ├── mod.rs              → pub use
    │   └── text_renderer.rs    → TextRenderer struct (level 2)
    │
    ├── primitives/
    │   ├── mod.rs              → pub use
    │   ├── atlas.rs            → GlyphAtlas (raw)
    │   ├── pipeline.rs         → ZentypePipeline (raw)
    │   └── shaped_buffer.rs    → ShapedBuffer (raw)
    │
    ├── traits/
    │   ├── mod.rs              → pub use
    │   ├── font_provider.rs    → FontProvider trait
    │   ├── rasterizer.rs       → Rasterizer trait
    │   └── atlas.rs            → Atlas trait
    │
    ├── defaults/
    │   ├── mod.rs              → pub use
    │   ├── cosmic_font.rs      → CosmicFontProvider (default impl)
    │   ├── swash_raster.rs     → SwashRasterizer (default impl)
    │   └── zentype_atlas.rs    → ZentypeAtlas (default impl)
    │
    ├── types/
    │   ├── mod.rs              → pub use
    │   ├── color.rs            → Color type
    │   ├── options.rs          → TextOptions struct
    │   └── glyph.rs            → GlyphKey, AtlasEntry, RasterizedGlyph
    │
    └── gpu/
        ├── mod.rs              → pub use
        ├── pipeline.rs         → wgpu pipeline setup
        ├── atlas.rs            → texture atlas logic
        └── shader.wgsl         → custom WGSL shader
```

---

### How They Connect

```
lib.rs
  ├── prelude.rs          pulls from managed + renderer + types
  ├── managed/            uses renderer/ internally
  ├── renderer/           uses primitives/ + defaults/ internally
  ├── primitives/         uses gpu/ + types/ internally
  ├── traits/             standalone, no dependencies
  ├── defaults/           implements traits/ using gpu/
  ├── types/              standalone, shared everywhere
  └── gpu/                raw wgpu code, used by primitives + defaults
```

---

### The Rule

> `gpu/` is the engine room. Nobody outside touches it directly except `primitives/` and `defaults/`. Everything else talks through traits and types.

---

### Your Current Files Map Here

| Now | Goes to |
|---|---|
| `src/main.rs` | stays as demo binary |
| `src/gpu/atlas.rs` | → `src/gpu/atlas.rs` + `src/primitives/atlas.rs` |
| `src/gpu/pipeline.rs` | → `src/gpu/pipeline.rs` + `src/primitives/pipeline.rs` |
| `src/gpu/shader.wgsl` | → `src/gpu/shader.wgsl` |

---
