use crate::types::shaped_glyph::ShapedGlyph;

/// A buffer of shaped and positioned glyphs.
/// This is the final output of a text shaping operation.
#[derive(Debug, Clone, Default)]
pub struct ShapedBuffer {
    glyphs: Vec<ShapedGlyph>,
    width: f32,
    height: f32,
}

impl ShapedBuffer {
    /// Creates a new empty ShapedBuffer.
    pub fn new(glyphs: Vec<ShapedGlyph>, width: f32, height: f32) -> Self {
        Self { glyphs, width, height }
    }

    /// Returns the list of shaped glyphs in this buffer.
    pub fn glyphs(&self) -> &[ShapedGlyph] {
        &self.glyphs
    }

    /// Returns the logical (width, height) of the shaped text.
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}

