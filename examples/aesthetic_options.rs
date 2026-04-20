//! Zentype — Aesthetic Options Example
//!
//! Showcases the "premium" color constants and builder API for TextOptions.
//! Uses the VisualTester to render a beautifully styled snippet.
//! Run with: `cargo run --example aesthetic_options`

use cosmic_text::Shaping;
use zentype::prelude::*;
use zentype::testing::VisualTester;

fn main() {
    VisualTester::run(|font_system, buffer| {
        // 1. Define a "premium" design using the builder pattern
        let options = TextOptions::new()
            .font_size(32.0)
            .color(Color::AMBER) // Premium color constant
            .bg(Color::DARK_GRAY) // Contrast background
            // Spacious padding
            .font_weight(FontWeight::Bold)
            .align(HorizontalAlignment::Center);

        // 2. Apply options to the existing cosmic-text buffer
        // (VisualTester uses the bridge methods we built)
        options.apply(font_system, buffer);

        // 3. Render some text
        let text = "AMBER ON DARK GRAY PREMIUM TYPOGRAPHY";

        buffer.set_text(
            font_system,
            text,
            &options.as_attrs(),
            Shaping::Advanced,
            None,
        );

        println!("Rendering with options: {:?}", options);
    });
}
