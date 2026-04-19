use cosmic_text::{Buffer, FontSystem, Metrics};

fn main() {
    let mut font_system = FontSystem::new();
    let metrics = Metrics::new(24.0, 32.0);
    let mut buffer = Buffer::new(&mut font_system, metrics);
    
    // Probing for ascent/descent access
    for run in buffer.layout_runs() {
        // In some versions these are on the run, in others we need to calculate them
        // println!("ascent: {}, descent: {}", run.ascent, run.descent);
    }
}
