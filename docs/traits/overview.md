# Traits Overview

Zentype is built on a modular, trait-based architecture. Instead of forcing you into a single font engine or rasterizer, it provides a set of core traits that you can implement to customize the engine's behavior.

## Why Traits?

Most text engines are "hard-coded." If you want to use a different glyph cache or a custom font source, you have to fork the entire library. Zentype is different.

### **Key Traits**

| Trait | Responsibility |
| :--- | :--- |
| `FontProvider` | Responsible for loading and caching font families/faces. |
| `Rasterizer` | Converts vector glyphs into alpha-masked bitmaps on demand. |
| `Atlas` | Manages the GPU texture packing for shaped glyphs. |

---

## The Builder Pattern

When initializing the Zentype engine, you can "swap" any of these components at runtime:

```rust
let renderer = TextRenderer::builder()
    .with_font_provider(MyCustomProvider::new())
    .with_rasterizer(MyFastRasterizer::new())
    .build();
```

## Creating Your Own
Check the specific trait guides for implementation details:
- [Implementing a Font Provider](./font-provider.md)
- [Implementing a Rasterizer](./rasterizer.md)
