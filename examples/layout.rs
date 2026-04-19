use zentype::prelude::*;
use zentype::testing::VisualTester;
use cosmic_text::{Attrs, Shaping};

fn main() {
    VisualTester::run(|font_system, buffer| {
        let text = "LAYOUT & WRAPPING\n\n\
                    This is a long paragraph designed to test word wrapping \n\
                    when a maximum width is constrained.zentype will handle \n\
                    the metrics and line height automatically based on \n\
                    your options.";

        let options = TextOptions::new()
            .font_size(24.0)
            .line_height(1.8) // Spacious layout
            .max_width(400.0)
            .wrap(TextWrap::Word)
            .align(TextAlignment::Justified);

        options.apply(font_system, buffer);

        // Applying the width manually for now since high-level layout is Phase 5
        buffer.set_size(font_system, Some(400.0), None);
        buffer.set_text(font_system, text, &Attrs::new(), Shaping::Advanced, None);
    });
}
