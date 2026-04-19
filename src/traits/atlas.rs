use crate::types::glyph::{GlyphKey, AtlasEntry, RasterizedGlyph};

/// A trait for managing the GPU texture atlas where glyphs are stored.
pub trait Atlas {
    /// Returns the UV metadata for a glyph if it's already in the atlas.
    fn get(&self, key: &GlyphKey) -> Option<AtlasEntry>;

    /// Inserts a new glyph into the atlas and returns its UV metadata.
    /// This may involve uploading data to the GPU.
    fn insert(&mut self, key: GlyphKey, glyph: &RasterizedGlyph) -> AtlasEntry;

    /// Clears the atlas (e.g., when the texture is full or the scale changes).
    fn clear(&mut self);
}
