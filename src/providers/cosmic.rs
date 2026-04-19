use crate::traits::font_provider::{FontProvider, FontMetrics};
use crate::types::options::TextOptions;
use crate::types::shaped_glyph::ShapedGlyph;
use cosmic_text::{Buffer, FontSystem, Metrics, Align, Shaping};

/// A FontProvider that uses the `cosmic-text` library for shaping and layout.
pub struct CosmicFontProvider {
    font_system: FontSystem,
    buffer: Buffer,
}

impl CosmicFontProvider {
    /// Creates a new CosmicFontProvider.
    pub fn new() -> Self {
        let mut font_system = FontSystem::new();
        let buffer = Buffer::new(&mut font_system, Metrics::new(24.0, 32.0));
        Self {
            font_system,
            buffer,
        }
    }
}

impl FontProvider for CosmicFontProvider {
    fn shape(&mut self, text: &str, options: &TextOptions) -> Vec<ShapedGlyph> {
        // 1. Convert our options to cosmic-text metrics
        let metrics = Metrics::new(
            options.font_size,
            options.font_size * options.line_height,
        );
        self.buffer.set_metrics(&mut self.font_system, metrics);

        // 2. Set the text and initial shaping
        self.buffer.set_text(
            &mut self.font_system,
            text,
            &options.as_attrs(),
            Shaping::Advanced,
            None,
        );

        // 3. Apply alignment if explicitly requested
        if let Some(alignment) = options.align {
            let align: Align = alignment.into();
            for line in self.buffer.lines.iter_mut() {
                line.set_align(Some(align));
            }
        }

        // 4. Perform the final layout pass
        self.buffer.shape_until_scroll(&mut self.font_system, false);

        // 5. Convert cosmic-text layout runs into our internal ShapedGlyphs
        let mut shaped_glyphs = Vec::new();
        for run in self.buffer.layout_runs() {
            for glyph in run.glyphs {
                // Accessing physical position to get the cache key
                let physical = glyph.physical((0.0, 0.0), 1.0);
                shaped_glyphs.push(ShapedGlyph {
                    key: physical.cache_key,
                    x: glyph.x,
                    y: run.line_y as f32 + glyph.y,
                    width: glyph.w,
                    height: metrics.line_height,
                });
            }
        }

        shaped_glyphs
    }

    fn metrics(&self, options: &TextOptions) -> FontMetrics {
        let font_size = options.font_size;
        let line_height = font_size * options.line_height;
        
        FontMetrics {
            ascent: font_size, 
            descent: - (line_height - font_size),
            line_gap: 0.0,
        }
    }
}
