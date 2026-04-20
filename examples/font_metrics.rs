//! Zentype — Font Metrics Example
//!
//! Demonstrates how to extract and use font metrics via the FontProvider trait.
//! Run with: `cargo run --example font_metrics`

use zentype::prelude::*;
use zentype::traits::FontProvider;

fn main() {
    let provider = CosmicFontProvider::new();
    let sizes = [12.0, 16.0, 24.0, 32.0, 48.0, 64.0];

    println!(
        "{:<8} | {:<10} | {:<10} | {:<10} | {:<10}",
        "Size", "Ascent", "Descent", "Line Gap", "Line Height"
    );
    println!("{:-<60}", "");

    for size in sizes {
        let options = TextOptions::new().font_size(size);
        let metrics = provider.metrics(&options);

        println!(
            "{:<8.1} | {:<10.2} | {:<10.2} | {:<10.2} | {:<10.2}",
            size,
            metrics.ascent,
            metrics.descent,
            metrics.line_gap,
            metrics.line_height()
        );
    }
}
