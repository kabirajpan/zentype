# First Render

Getting your first text on screen with Zentype is intentionally simple. This guide uses the `VisualTester` harness to demonstrate how the engine handles metrics and shaping.

## The Minimal Example

In just a few lines, you can configure your text size, line height, and color.

```rust
use zentype::prelude::*;
use zentype::testing::VisualTester;

fn main() {
    // 1. Run the visual tester harness
    VisualTester::run(|font_system, buffer| {
        
        // 2. Set metrics (font size 32.0, line height 42.0)
        buffer.set_metrics(font_system, Metrics::new(32.0, 42.0));
        
        // 3. Set the text content
        buffer.set_text(
            font_system, 
            "Hello, Zentype!", 
            &Attrs::new().color(Color::WHITE.into()), 
            Shaping::Advanced, 
            None
        );
        
        // 4. Finalize the layout pass
        buffer.shape_until_scroll(font_system, false);
    });
}
```

![Hello World Output](../../images/hello/hello.png)

---

## Automatic Text Wrapping

One of Zentype's core strengths is its high-performance wrapping engine. By default, Zentype will automatically wrap your text to fit the available window width without any manual line breaks.

```rust
let text = "This is a long paragraph that stretches across the window \
            without any manual line breaks. Zentype handles the wrapping \
            automatically based on the window width.";

buffer.set_text(font_system, text, &Attrs::new(), Shaping::Advanced, None);
```

![Paragraph Wrapping](../../images/hello/para.png)

---

## Next Steps
Now that you have your first text on screen, learn how to style it with [Background Highlights](../guides/line-highlights.md).
