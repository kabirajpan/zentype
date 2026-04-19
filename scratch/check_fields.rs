use cosmic_text::{Buffer, FontSystem, Metrics};
use zentype::prelude::*;

fn main() {
    let mut font_system = FontSystem::new();
    let buffer = Buffer::new(&mut font_system, Metrics::new(24.0, 32.0));
    for run in buffer.layout_runs() {
        // This is just to check fields via compiler if I run this,
        // but I will just look at the code.
        let _ = run.line_y;
        // let _ = run.line_x; // This failed
    }
}
