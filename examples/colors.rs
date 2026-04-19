use zentype::prelude::*;
use zentype::testing::VisualTester;
use cosmic_text::{Attrs, Shaping};

fn main() {
    VisualTester::run(|font_system, buffer| {
        let options = TextOptions::new().font_size(32.0);
        options.apply(font_system, buffer);

        // We use set_rich_text to apply DIFFERENT colors to each line
        let spans = [
            ("PREMIUM COLORS\n\n", options.as_attrs().color(Color::WHITE.into())),
            ("- Sharp Red (RED)\n", options.as_attrs().color(Color::RED.into())),
            ("- Deep Blue (BLUE)\n", options.as_attrs().color(Color::BLUE.into())),
            ("- Teal (TEAL)\n", options.as_attrs().color(Color::TEAL.into())),
            ("- Indigo (INDIGO)\n", options.as_attrs().color(Color::INDIGO.into())),
            ("- Cyan (CYAN)\n", options.as_attrs().color(Color::CYAN.into())),
            ("- Magenta (MAGENTA)\n", options.as_attrs().color(Color::MAGENTA.into())),
            ("- Sky Blue (SKY_BLUE)\n", options.as_attrs().color(Color::SKY_BLUE.into())),
            ("- Mint (MINT)\n", options.as_attrs().color(Color::MINT.into())),
            ("- Emerald (EMERALD)\n", options.as_attrs().color(Color::EMERALD.into())),
            ("- Amber (AMBER)\n", options.as_attrs().color(Color::AMBER.into())),
            ("- Lavender (LAVENDER)\n", options.as_attrs().color(Color::LAVENDER.into())),
            ("- Crimson (CRIMSON)\n", options.as_attrs().color(Color::CRIMSON.into())),
        ];

        buffer.set_rich_text(
            font_system, 
            spans, 
            &options.as_attrs(), 
            Shaping::Advanced,
            None
        );
    });
}
