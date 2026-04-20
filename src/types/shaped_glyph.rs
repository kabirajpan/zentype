use crate::types::glyph::GlyphKey;

/// A glyph that has been shaped and positioned in screen space.
/// This is the output of the FontProvider and the input for the Renderer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapedGlyph {
    /// Unique identifier for the font and glyph index.
    pub key: GlyphKey,
    /// Byte index of the character in the input string.
    pub cluster: usize,
    /// X coordinate in logical pixels.
    pub x: f32,

    /// Y coordinate in logical pixels.
    pub y: f32,
    /// Width of the glyph (may include padding/advancement).
    pub width: f32,
    /// Height of the glyph.
    pub height: f32,
}
