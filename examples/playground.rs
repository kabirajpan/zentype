//! Zentype Playground
//! 
//! Now using the separated VisualTester utility!
//! 
//! Run this with: `cargo run --example playground`

use zentype::prelude::*;
use zentype::testing::VisualTester;
use cosmic_text::{Attrs, Shaping};

fn main() {
    VisualTester::run(|font_system, buffer| {
        // 1. Define your text
        let text = "Welcome to the Clean Playground!\n\n\
                    The window boilerplate is now hidden in the library.\n\
                    You can focus entirely on testing features here.";

        // 2. Your Style
        let options = TextOptions::new()
            .font_size(32.0)
            .color(Color::hex("#FFD700"))
            .align(TextAlignment::Center);

        // 3. Apply options & text
        options.apply(font_system, buffer);
        buffer.set_text(
            font_system, 
            text, 
            &Attrs::new(), 
            Shaping::Advanced, 
            None
        );
    });
}
