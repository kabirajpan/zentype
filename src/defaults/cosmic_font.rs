use crate::primitives::shaped_buffer::ShapedBuffer;
use crate::traits::font_provider::{FontMetrics, FontProvider};
use crate::types::options::TextOptions;
use crate::types::shaped_glyph::ShapedGlyph;
use cosmic_text::{Align, Buffer, FontSystem, Metrics, Shaping};
use std::path::Path;

/// A FontProvider that uses the `cosmic-text` library for shaping and layout.
pub struct CosmicFontProvider {
    font_system: FontSystem,
    buffer: Buffer,
}

impl CosmicFontProvider {
    /// Creates a new CosmicFontProvider with system fonts loaded.
    pub fn new() -> Self {
        let mut font_system = FontSystem::new();
        let buffer = Buffer::new(&mut font_system, Metrics::new(16.0, 24.0));
        Self {
            font_system,
            buffer,
        }
    }

    /// Access the underlying font system.
    pub fn font_system(&self) -> &FontSystem {
        &self.font_system
    }

    /// Access the underlying font system mutably.
    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }
}

impl Default for CosmicFontProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FontProvider for CosmicFontProvider {
    fn shape(&mut self, text: &str, options: &TextOptions) -> ShapedBuffer {
        // 1. Convert our options to cosmic-text metrics
        let metrics = Metrics::new(options.font_size, options.font_size * options.line_height);
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

        // 5. Convert layout runs into our internal primitives
        let mut shaped_glyphs = Vec::new();
        let mut lines = Vec::new();
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for run in self.buffer.layout_runs() {
            max_height = max_height.max(run.line_y + metrics.line_height);

            // Capture line geometry for automatic backgrounds
            // We calculate the alignment offset manually since cosmic-text doesn't expose it directly in LayoutRun
            let layout_width = self.buffer.size().0.unwrap_or(run.line_w);
            let alignment_offset = match options.align {
                Some(crate::types::HorizontalAlignment::Center) => {
                    (layout_width - run.line_w) / 2.0
                }
                Some(crate::types::HorizontalAlignment::Right) => layout_width - run.line_w,
                _ => 0.0,
            };

            lines.push(crate::types::line::LineInfo {
                x: alignment_offset,
                y: run.line_y,
                width: run.line_w,
            });

            for glyph in run.glyphs {
                max_width = max_width.max(glyph.x + glyph.w);

                // Get the cache key representing this unique glyph at this size/weight/style
                let physical = glyph.physical((0.0, 0.0), 1.0);
                shaped_glyphs.push(ShapedGlyph {
                    key: physical.cache_key,
                    cluster: glyph.start,
                    x: glyph.x,
                    y: run.line_y + glyph.y,
                    width: glyph.w,
                    height: 0.0, // Precision height managed by TextRenderer via Atlas
                });

            }
        }

        ShapedBuffer::new(shaped_glyphs, lines, max_width, max_height)
    }

    fn load_font(&mut self, data: Vec<u8>) {
        self.font_system.db_mut().load_font_data(data);
    }

    fn load_font_path(&mut self, path: &Path) -> std::io::Result<()> {
        self.font_system.db_mut().load_font_file(path)
    }

    fn metrics(&self, options: &TextOptions) -> FontMetrics {
        let font_size = options.font_size;
        let line_height = font_size * options.line_height;

        // Simple heuristic for metrics - Phase 7 will refine this with actual font data
        FontMetrics {
            ascent: font_size,
            descent: -(line_height - font_size),
            line_gap: 0.0,
        }
    }

    fn set_layout_size(&mut self, width: f32, height: f32) {
        self.buffer
            .set_size(&mut self.font_system, Some(width), Some(height));
    }
}
