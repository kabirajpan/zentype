use cosmic_text::{Attrs, Metrics, Shaping};
use zentype::prelude::*;
use zentype::testing::VisualTester;

fn main() {
    VisualTester::run(|font_system, buffer| {
        // A single, continuous paragraph with no manual newlines (\n)
        let text = "Kabiraj Pan";

        // Set metrics
        buffer.set_metrics(font_system, Metrics::new(24.0, 32.0));

        let options = TextOptions::new()
            .font_size(24.0)
            .line_height(1.4)
            .color(Color::WHITE)
            .bg(Color::GREEN)
            .padding(30.0)
            .align(HorizontalAlignment::Center)
            .valign(VerticalAlignment::Center);

        options.apply(font_system, buffer);

        buffer.set_text(
            font_system,
            text,
            &options.as_attrs(),
            Shaping::Advanced,
            None,
        );

        // Required to trigger the auto-wrapping logic
        buffer.shape_until_scroll(font_system, false);
    });
}
