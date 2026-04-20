//! Zentype — Styling Sandbox
//!
//! A minimal playground for manually testing different styles.
//! Run with: `cargo run --example styling`

use cosmic_text::Shaping;
use zentype::prelude::*;
use zentype::testing::VisualTester;

fn main() {
    VisualTester::run(|font_system, buffer| {
        // --- FONT NAMES YOU CAN TRY ---
        // "sans-serif", "serif", "monospace", "Inter", "Roboto", "Ubuntu", "Fira Sans"

        let family = "serif";

        let options = TextOptions::new()
            .font_size(48.0)
            .font_family(family)
            .font_weight(FontWeight::Thin)
            .font_style(FontStyle::Italic) // Test Italics here!
            .color(Color::AMBER)
            .align(HorizontalAlignment::Center);
        // --------------------------------------

        let text = "Styling Sandbox Active";

        buffer.set_text(
            font_system,
            text,
            &options.as_attrs(),
            Shaping::Advanced,
            None,
        );

        // Apply alignment AFTER setting text so it doesn't get reset
        options.apply(font_system, buffer);
    });
}
