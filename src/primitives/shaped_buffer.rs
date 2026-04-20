use crate::types::shaped_glyph::ShapedGlyph;
use crate::types::line::LineInfo;

/// A buffer of shaped and positioned glyphs.
/// This is the final output of a text shaping operation.
#[derive(Debug, Clone, Default)]
pub struct ShapedBuffer {
    glyphs: Vec<ShapedGlyph>,
    lines: Vec<LineInfo>,
    width: f32,
    height: f32,
}

impl ShapedBuffer {
    /// Creates a new empty ShapedBuffer.
    pub fn new(glyphs: Vec<ShapedGlyph>, lines: Vec<LineInfo>, width: f32, height: f32) -> Self {
        Self { glyphs, lines, width, height }
    }

    /// Returns the list of shaped glyphs in this buffer.
    pub fn glyphs(&self) -> &[ShapedGlyph] {
        &self.glyphs
    }

    /// Returns the tracked line information for this buffer.
    pub fn lines(&self) -> &[LineInfo] {
        &self.lines
    }

    /// Returns the number of glyphs in the buffer.
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Returns true if the buffer contains no glyphs.
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    /// Returns the logical width and height of the text content itself (no padding).
    pub fn content_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    /// Returns the visual width and height including the provided padding.
    pub fn outer_size(&self, padding: &crate::types::options::Padding) -> (f32, f32) {
        (
            self.width + padding.left + padding.right,
            self.height + padding.top + padding.bottom,
        )
    }

    /// Returns the logical (width, height) used during the last shaping pass.
    /// This is an alias for `content_size()`.
    pub fn size(&self) -> (f32, f32) {
        self.content_size()
    }

    /// Finds the character byte index closest to the given (x, y) coordinates.
    pub fn index_at(&self, x: f32, y: f32) -> usize {
        if self.lines.is_empty() || self.glyphs.is_empty() {
            return 0;
        }

        // 1. Find the line with the closest Y coordinate
        let mut best_line_idx = 0;
        let mut min_dist_y = f32::MAX;

        for (i, line) in self.lines.iter().enumerate() {
            let dist = (y - line.y).abs();
            if dist < min_dist_y {
                min_dist_y = dist;
                best_line_idx = i;
            }
        }

        let best_line = &self.lines[best_line_idx];

        // 2. Find the glyph on that line with the closest X coordinate
        let mut best_cluster = 0;
        let mut min_dist_x = f32::MAX;

        // Note: For now we assume glyphs on a line share the same Y.
        // In a complex multi-font line, Y might vary slightly, so we use a threshold.
        let mut found_glyph = false;
        for glyph in &self.glyphs {
            if (glyph.y - best_line.y).abs() < 1.0 {
                let center_x = glyph.x + glyph.width / 2.0;
                let dist = (x - center_x).abs();
                if dist < min_dist_x {
                    min_dist_x = dist;
                    best_cluster = glyph.cluster;
                    found_glyph = true;
                }
            }
        }

        if !found_glyph {
            // Fallback to first/last cluster if no glyphs on the "best" line were found
            return self.glyphs.first().map(|g| g.cluster).unwrap_or(0);
        }

        best_cluster
    }

    /// Returns the (x, y) coordinates for a given character byte index.
    pub fn position_at(&self, index: usize) -> Option<(f32, f32)> {
        self.glyphs.iter()
            .find(|g| g.cluster == index)
            .map(|g| (g.x, g.y))
    }
}



