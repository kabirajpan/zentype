//! Zentype Hello World
//!
//! The simplest possible way to render text in the current version.
//! Run with: `cargo run --example hello`

use cosmic_text::{Attrs, Shaping};
use zentype::prelude::*;
use zentype::testing::VisualTester;

fn main() {
    // VisualTester opens the window and gives us the drawing tools
    VisualTester::run(|font_system, buffer| {
        // 1. Set the text
        let text = "Hello Zentype!";

        // 2. Set the options (minimal)
        let options = TextOptions::new().font_size(24.0).color(Color::WHITE);

        // 3. Apply options to the buffer
        options.apply(font_system, buffer);

        // 4. Shape the text (passing options.as_attrs() to apply the color)
        buffer.set_text(
            font_system,
            text,
            &options.as_attrs(),
            Shaping::Advanced,
            None,
        );
    });
}
