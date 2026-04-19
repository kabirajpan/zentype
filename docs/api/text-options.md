# TextOptions API

`TextOptions` is the primary way to configure the appearance and behavior of text in Zentype. It uses a **fluent builder pattern**, allowing you to chain configuration methods.

## Background & Highlights

Zentype supports professional, symmetric background rendering.

### **Padding (4-Way)**
You can set padding uniformly or for each side independently.

| Method | Description |
| :--- | :--- |
| `.padding(f32)` | Sets all four sides equally. |
| `.padding_horizontal(f32)` | Sets Left and Right padding. |
| `.padding_vertical(f32)` | Sets Top and Bottom padding. |
| `.padding_left(f32)` | Sets only the Left side. |
| `.padding_top(f32)` | Sets only the Top side. |

### **Configuration**
| Method | Description |
| :--- | :--- |
| `.bg(Color)` | Sets the background color. |
| `.full_width(bool)` | If `true`, the highlight spans the full width of the container. |

---

## Alignment & Internal Layout

Zentype features a "Safe Area" system that automatically insets your text based on the padding you've provided.

### **Horizontal Alignment**
Use `HorizontalAlignment` (Left, Center, Right).

```rust
.align(HorizontalAlignment::Center)
```

### **Vertical Alignment**
Use `VerticalAlignment` (Top, Center, Bottom).

```rust
.valign(VerticalAlignment::Bottom)
```

> [!IMPORTANT]
> **Sticky Alignment**: Zentype's rendering pipeline is designed to be "sticky." If you apply alignment to a buffer, it will persist across multiple `set_text` calls until explicitly changed.

---

## Typography & Color

| Method | Description |
| :--- | :--- |
| `.font_size(f32)` | Sets the glyph size in logical pixels. |
| `.color(Color)` | Sets the foreground text color. |
| `.line_height(f32)` | Sets the vertical spacing (e.g., `1.5` for 150%). |
| `.font_weight(Weight)` | Uses the `FontWeight` enum (Thin, Normal, Bold, etc.). |
