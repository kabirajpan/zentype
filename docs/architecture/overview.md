# Architecture Overview

Zentype is designed as a **layered typography stack**. This approach allows it to be incredibly flexible: beginners can use the managed "Layer 1" API, while engine developers can drop down to "Layer 3" for raw GPU control.

## The Three Layers

### **Layer 1: Managed Zentype**
The highest level. Zentype handles the `winit` event loop (optionally), font loading, and the permanent glyph atlas. You just provide text and options.

### **Layer 2: TextRenderer**
The semi-managed layer. You provide the `wgpu` Device and Queue, and Zentype gives you a `TextRenderer` that you can call in your own render pass.

### **Layer 3: Raw Primitives**
The lowest level. Access the `ShapedBuffer`, `GlyphAtlas`, and `ZentypePipeline` directly. This is where the core math and GPU shaders live.

---

## The Pipeline Flow

1. **Shaping**: `cosmic-text` converts UTF-8 strings into a list of glyph IDs and pixel positions.
2. **Rasterization**: `swash` generates alpha-masked bitmaps for any new glyphs.
3. **Atlas Management**: New bitmaps are packed into a GPU texture atlas.
4. **Drawing**: instanced quads are sent to the Zentype shader, which samples the atlas and draws them to the screen.

---

## Technical Details
- [GPU Pipeline](./gpu-pipeline.md)
- [Glyph Atlas Management](./glyph-atlas.md)
- [Shaping & Flow](./shaping.md)
