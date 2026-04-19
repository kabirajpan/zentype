use zentype::prelude::*;
use zentype::testing::VisualTester;
use cosmic_text::{Attrs, Shaping, Metrics, Align};

fn main() {
    VisualTester::run(|font_system, buffer| {
        // 1. Define separate lines and their intended alignments
        let items = [
            ("LEFT ALIGNMENT", HorizontalAlignment::Left),
            ("CENTER ALIGNMENT", HorizontalAlignment::Center),
            ("RIGHT ALIGNMENT", HorizontalAlignment::Right),
        ];

        // 2. Build the combined string with spacing
        let text = items.iter()
            .map(|(t, _)| *t)
            .collect::<Vec<_>>()
            .join("\n\n");

        // 3. Set Metrics (Matrices)
        let metrics = Metrics::new(32.0, 48.0);
        buffer.set_metrics(font_system, metrics);
        
        // 4. Set the text
        buffer.set_text(font_system, &text, &Attrs::new().color(Color::WHITE.into()), Shaping::Advanced, None);

        // 5. Apply alignments and style each line
        for (i, (_, align)) in items.iter().enumerate() {
            // Because we used "\n\n", each item is 2 lines apart in the buffer
            let buffer_line_index = i * 2;
            if let Some(line) = buffer.lines.get_mut(buffer_line_index) {
                line.set_align(Some((*align).into()));
            }
        }

        // 6. Final layout pass
        buffer.shape_until_scroll(font_system, false);
    });
}
