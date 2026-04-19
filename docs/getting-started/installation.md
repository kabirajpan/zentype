# Installation

Zentype is designed to be easily added to any Rust project. It requires the latest stable version of Rust (2021 Edition or later).

## Adding Zentype

Add Zentype to your `Cargo.toml` dependencies:

```toml
[dependencies]
zentype = "0.1.0-alpha.1"
```

### **System Prerequisites**

Since Zentype uses `wgpu` for hardware-accelerated rendering, you may need to ensure your system has the appropriate drivers:

- **Linux**: Vulkan (Vulkan-SDK) or OpenGL (Mesa) drivers.
- **Windows**: DirectX 12 or Vulkan.
- **macOS**: Metal.

## Feature Flags

Zentype includes several optional features to keep the binary size small.

| Feature | Description |
| :--- | :--- |
| `testing` | (Default) Includes the `VisualTester` harness. |
| `atlas` | (Default) High-performance glyph caching. |
| `serde` | Enable serialization for `TextOptions` and `Color`. |

## Next Steps
Head over to the [First Render](./first-render.md) guide to get your first window up and running.
