# Color API

The `Color` type in Zentype is a lightweight wrapper around an `[f32; 4]` representing Red, Green, Blue, and Alpha channels in the **0.0 to 1.0** range.

## Constants

Zentype provides common colors as static constants for quick styling.

```rust
use zentype::prelude::Color;

let white = Color::WHITE;
let black = Color::BLACK;
let red   = Color::RED;
let green = Color::GREEN;
let blue  = Color::BLUE;
```

## Creating Colors

You can create custom colors from RGB or RGBA values.

### **From RGBA (0.0 to 1.0)**
```rust
let my_color = Color::new(0.5, 0.0, 1.0, 1.0); // Purple
```

### **From Hex**
```rust
// Coming Soon
// let my_hex = Color::from_hex("#FF00FF");
```

## Conversions

Zentype colors can be easily converted to other formats for interoperability with `wgpu` or `cosmic-text`.

```rust
let (r, g, b, a) = my_color.to_rgba();
let cosmic_color = my_color.into(); // Converts to cosmic_text::Color
```
