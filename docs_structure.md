Here's how the `docs/` folder should look for Zentype:

```
docs/
├── index.md                        → what is zentype, quick start
│
├── getting-started/
│   ├── installation.md             → cargo add, prerequisites
│   ├── first-render.md             → hello world in 5 lines
│   └── choosing-your-level.md      → when to use L1 vs L2 vs L3
│
├── api/
│   ├── overview.md                 → full API surface at a glance
│   ├── text-options.md             → TextOptions fields explained
│   ├── color.md                    → Color type, constants, helpers
│   ├── font-weight.md              → FontWeight enum
│   └── text-wrap.md                → TextWrap enum
│
├── levels/
│   ├── level-1-managed.md          → Zentype struct, full examples
│   ├── level-2-semi-managed.md     → TextRenderer, builder pattern
│   └── level-3-raw-primitives.md   → GlyphAtlas, ZentypePipeline, ShapedBuffer
│
├── traits/
│   ├── overview.md                 → what traits are, why they exist
│   ├── font-provider.md            → FontProvider trait + impl guide
│   ├── rasterizer.md               → Rasterizer trait + impl guide
│   └── atlas.md                    → Atlas trait + impl guide
│
├── guides/
│   ├── bring-your-own-x.md         → swapping components, builder pattern
│   ├── escape-hatches.md           → dropping down levels at runtime
│   ├── line-highlights.md          → full-width editor-style highlights
│   ├── custom-fonts.md             → loading fonts from file or bytes
│   ├── integrating-wgpu.md         → using zentype inside existing wgpu app
│   └── performance.md              → atlas tuning, batching, benchmarks
│
├── architecture/
│   ├── overview.md                 → how the layers connect internally
│   ├── gpu-pipeline.md             → wgpu pipeline, instanced quads
│   ├── glyph-atlas.md              → etagere, texture packing, GPU upload
│   ├── shaping.md                  → cosmic-text, swash, how text flows
│   └── shader.md                   → shader.wgsl explained, text vs solid mode
│
└── contributing/
    ├── setup.md                    → dev environment, running tests
    ├── code-style.md               → clippy, fmt, naming conventions
    └── releasing.md                → version bumping, crates.io publish
```

---

**The rule for each file:**
- `getting-started/` → for complete beginners, zero assumed knowledge
- `api/` → dry reference docs, one page per type
- `levels/` → deep dive into each API level with full examples
- `traits/` → how to implement your own components
- `guides/` → task-focused, "how do I do X"
- `architecture/` → for contributors and curious power users
- `contributing/` → for people opening PRs

