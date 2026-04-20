use zentype::prelude::*;
use zentype::testing::VisualTester;

fn main() {
    // Level 1: The Zero-Config Managed Entry Point
    // Everything (Shaper, Rasterizer, Atlas, Renderer) is handled for you!
    VisualTester::run_zentype(|zentype, queue| {
        
        // 1. Simple fast printing
        zentype.print(queue, "Zentype Level 1", [50.0, 50.0], 48.0, Color::rgb(255, 128, 0));

        // 2. Full control with TextOptions
        zentype.draw(queue, 
            "The Zero-Config API is here. No manual atlas management, no manual shaping buffers. Just high-fidelity typography at the speed of thought.", 
            [50.0, 150.0], 
            &TextOptions::new()
                .font_size(24.0)
                .color(Color::WHITE)
                .max_width(600.0)
                .align(HorizontalAlignment::Left)
        );

        // 3. Easy alignment
        zentype.draw(queue, 
            "Perfect Centering.", 
            [0.0, 350.0], 
            &TextOptions::new()
                .font_size(32.0)
                .color(Color::rgb(0, 255, 255))
                .align(HorizontalAlignment::Center)
        );

    });
}
