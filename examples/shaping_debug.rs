//! Zentype — Shaping Debug Example
//!
//! This example demonstrates the Level 3 (Primitives) and Traits API.
//! It uses the CosmicFontProvider to shape text and prints the raw buffer data.
//! Run with: `cargo run --example shaping_debug`

use zentype::prelude::*;
use zentype::traits::FontProvider;

fn main() {
    println!("--- Zentype Shaping Debug ---");

    // 1. Initialize the provider
    let mut provider = CosmicFontProvider::new();

    // 2. Define text and options
    let text = "Zentype: Modular Text Engine.";
    let options = TextOptions::new()
        .font_size(24.0)
        .line_height(1.5)
        .color(Color::WHITE);

    println!("Shaping text: \"{}\"", text);
    println!(
        "Options: size={}, height={}",
        options.font_size, options.line_height
    );

    // 3. Perform shaping
    let buffer = provider.shape(text, &options);

    // 4. Inspect result
    let (width, height) = buffer.size();
    let glyphs = buffer.glyphs();

    println!("\nResulting ShapedBuffer:");
    println!("- Total width:  {:.2}px", width);
    println!("- Total height: {:.2}px", height);
    println!("- Glyph count:  {}", glyphs.len());

    if !glyphs.is_empty() {
        println!("\nFirst 3 Glyphs:");
        for (i, glyph) in glyphs.iter().take(3).enumerate() {
            println!(
                "  [{}] Key={:?}, x={:.2}, y={:.2}",
                i, glyph.key, glyph.x, glyph.y
            );
        }
    }

    println!("\nDone.");
}
