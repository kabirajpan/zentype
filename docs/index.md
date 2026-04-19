# Zentype

**Designer-grade typography for Rust applications with zero fuss.**

Zentype is a high-performance text rendering engine built on top of `cosmic-text` and `wgpu`. It is designed to bridge the gap between low-level GPU primitives and high-level designer needs, providing features like symmetric padded highlights, multi-axis alignment, and automatic text wrapping.

---

## Core Features

- **Pixel-Perfect Rendering**: High-fidelity glyph rasterization and shaping.
- **Background Engine**: Professional line highlights with 4-way individual padding.
- **Smart Alignment**: Multi-axis alignment (Horizontal & Vertical) with automated safe-area insets.
- **Level-Based API**: Choose your level of control, from fully managed to raw primitives.
- **Swappable Architecture**: Trait-based design allows you to bring your own Font Provider or Rasterizer.

---

## Quick Start

Ready to render? Head over to the [First Render](./getting-started/first-render.md) guide to see how to get a window up and running in just a few lines of code.

### Popular Guides
- [Line Highlights & Backgrounds](./guides/line-highlights.md)
- [Text Alignment & Safe Areas](./api/text-options.md)
- [Installation Guide](./getting-started/installation.md)
