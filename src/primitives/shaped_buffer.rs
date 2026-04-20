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

    /// Returns the logical (width, height) used during the last shaping pass.
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}


