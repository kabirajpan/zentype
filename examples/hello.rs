//! Zentype Interactive Hello World
//!
//! Demonstrates hit-testing and character positioning.
//! Run with: `cargo run --example hello`

use zentype::prelude::*;
use zentype::testing::VisualTester;

fn main() {
    // 1. Use the managed Zentype runner
    VisualTester::run_zentype(|zentype, queue, mouse_pos| {
        let text = "Hello I am Kabiraj Pan. Try hovering over me to see hit-testing in action!";
        let pos = [100.0, 100.0];

        let options = TextOptions::new()
            .font_family("monospace")
            .font_size(24.0)
            .color(Color::WHITE)
            .full_width(true)
            .bg(Color::hex("#1e1e1e"))
            .padding(Padding::all(10.0));

        // 2. Draw text and get the resulting buffer (the "layout brain")
        let buffer = zentype.draw(queue, text, pos, &options);

        // 3. Perform hit-testing using the engine's translation logic
        let index = zentype.hit_test(&buffer, pos, &options, mouse_pos);

        // 4. (Optional) Get the character's exact visual position
        let char_pos = zentype.position_at(&buffer, pos, &options, index);

        // 5. Log the results
        if let Some(cp) = char_pos {
            // We can print info here, or even draw a "mock cursor" using another label!
            println!("Hovering over index: {} | Pos: {:?}", index, cp);

            // Draw a small label above the text to show the current character
            let char_under_mouse = text.chars().nth(index).unwrap_or(' ');
            let label = format!("Char: '{}'", char_under_mouse);
            zentype.print(queue, &label, [cp[0], cp[1] - 30.0], 14.0, Color::YELLOW);
        }
    });
}
