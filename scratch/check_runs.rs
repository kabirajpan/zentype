use cosmic_text::{Buffer, FontSystem, Metrics};

fn main() {
    let mut font_system = FontSystem::new();
    let buffer = Buffer::new(&mut font_system, Metrics::new(24.0, 32.0));
    for run in buffer.layout_runs() {
        // Checking for common coordinate fields
        // let _ = run.line_x;
        // let _ = run.x;
        let _ = run.line_y;
        let _ = run.line_w;
    }
}
