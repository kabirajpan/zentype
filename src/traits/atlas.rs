use crate::types::glyph::{GlyphKey, AtlasEntry};

/// A trait for managing the GPU texture atlas where glyphs are stored.
pub trait Atlas: Send + Sync {
    /// Returns the UV metadata for a glyph, inserting it if not already present.
    fn get_or_insert(&mut self, key: GlyphKey, data: &[u8]) -> AtlasEntry;

    /// Returns a reference to the underlying WGPU texture.
    fn texture(&self) -> &wgpu::Texture;

    /// Flushes any pending data to the GPU queue.
    fn flush(&mut self, queue: &wgpu::Queue);

    /// Clears the atlas (e.g., when the texture is full or the scale changes).
    fn clear(&mut self);
}

