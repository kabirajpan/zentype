use bytemuck::{Pod, Zeroable};
use cosmic_text::CacheKey;

/// Unique identifier for a glyph at a specific size and style.
/// Directly maps to `cosmic_text::CacheKey`.
pub type GlyphKey = CacheKey;

/// Metadata for a glyph stored in the GPU atlas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtlasEntry {
    /// Normalized UV coordinates (top-left) in the atlas texture.
    pub uv_pos: [f32; 2],
    /// Normalized UV size (width, height) in the atlas texture.
    pub uv_size: [f32; 2],
}

/// Raw pixel data and metrics for a rasterized glyph.
pub struct RasterizedGlyph {
    /// Width of the rasterized bitmap.
    pub width: u32,
    /// Height of the rasterized bitmap.
    pub height: u32,
    /// Grayscale or RGBA pixel data.
    pub data: Vec<u8>,
}

/// A single instance of a glyph to be rendered on the GPU.
/// Uses a compact layout for efficient instanced drawing.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct GlyphInstance {
    /// Screen position [x, y].
    pub pos: [f32; 2],
    /// Screen size [width, height].
    pub size: [f32; 2],
    /// UV position [u, v].
    pub uv_pos: [f32; 2],
    /// UV size [u_width, v_height].
    pub uv_size: [f32; 2],
    /// Text color [r, g, b, a].
    pub color: [f32; 4],
    /// Background color [r, g, b, a].
    pub bg_color: [f32; 4],
}

impl GlyphInstance {
    /// Returns the vertex buffer layout for this struct.
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GlyphInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 },
                wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x2 },
                wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x2 },
                wgpu::VertexAttribute { offset: 24, shader_location: 3, format: wgpu::VertexFormat::Float32x2 },
                wgpu::VertexAttribute { offset: 32, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 48, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
            ],
        }
    }
}
