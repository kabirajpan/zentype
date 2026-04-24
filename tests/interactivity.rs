use zentype::prelude::*;

use cosmic_text::{CacheKey, Weight};

#[test]
fn test_hit_testing_translation() {
    let (dummy_key, _, _) = CacheKey::new(
        cosmic_text::fontdb::ID::dummy(), 
        0, 
        16.0, // font_size
        (0.0, 0.0), 
        Weight::NORMAL, 
        cosmic_text::CacheKeyFlags::empty()
    );

    let buffer = ShapedBuffer::new(
        vec![
            ShapedGlyph { key: dummy_key, cluster: 0, x: 0.0, y: 0.0, width: 10.0, height: 20.0 },
            ShapedGlyph { key: dummy_key, cluster: 1, x: 10.0, y: 0.0, width: 10.0, height: 20.0 },
            ShapedGlyph { key: dummy_key, cluster: 2, x: 20.0, y: 0.0, width: 10.0, height: 20.0 },
        ],
        vec![LineInfo { x: 0.0, y: 0.0, width: 30.0 }],
        100.0,
        100.0,
    );

    // 1. Local hit [2, 5] -> cluster 0
    let index = buffer.index_at(2.0, 5.0);
    assert_eq!(index, 0);

    // 2. Local hit [12, 5] -> cluster 1
    let index = buffer.index_at(12.0, 5.0);
    assert_eq!(index, 1);
}

#[test]
fn test_position_lookup() {
     let (dummy_key, _, _) = CacheKey::new(
        cosmic_text::fontdb::ID::dummy(), 
        0, 
        16.0, // font_size
        (0.0, 0.0), 
        Weight::NORMAL, 
        cosmic_text::CacheKeyFlags::empty()
    );

    let buffer = ShapedBuffer::new(
        vec![
            ShapedGlyph { key: dummy_key, cluster: 0, x: 0.0, y: 0.0, width: 10.0, height: 20.0 },
            ShapedGlyph { key: dummy_key, cluster: 1, x: 10.0, y: 0.0, width: 10.0, height: 20.0 },
        ],
        vec![LineInfo { x: 0.0, y: 0.0, width: 20.0 }],
        100.0,
        100.0,
    );

    // Position of cluster 1 in LOCAL space
    let pos = buffer.position_at(1).unwrap();
    assert_eq!(pos, (10.0, 0.0));
}

#[test]
fn test_hit_testing_with_padding() {
    let (dummy_key, _, _) = CacheKey::new(
        cosmic_text::fontdb::ID::dummy(), 
        0, 
        16.0, 
        (0.0, 0.0), 
        Weight::NORMAL, 
        cosmic_text::CacheKeyFlags::empty()
    );

    let buffer = ShapedBuffer::new(
        vec![
            ShapedGlyph { key: dummy_key, cluster: 0, x: 0.0, y: 0.0, width: 10.0, height: 20.0 },
        ],
        vec![LineInfo { x: 0.0, y: 0.0, width: 10.0 }],
        10.0,
        20.0,
    );

    // Simulation of TextRenderer::hit_test logic
    let object_pos = [100.0, 100.0];
    let padding_left = 20.0;
    let padding_top = 10.0;
    
    // Mouse is at [125.0, 115.0]
    // Relative to object: [25.0, 15.0]
    // Relative to content (after padding): [5.0, 5.0] -> Should hit cluster 0
    let mouse_pos = [125.0, 115.0];
    
    let local_x = mouse_pos[0] - object_pos[0] - padding_left;
    let local_y = mouse_pos[1] - object_pos[1] - padding_top;
    
    let index = buffer.index_at(local_x, local_y);
    assert_eq!(index, 0);

    // Mouse is at [105.0, 105.0] (Inside padding, but NOT over glyph)
    // local_x = -15.0, local_y = -5.0
    // buffer.index_at should still return 0 (closest cluster) or we might want it to return None?
    // Current implementation returns 0 as fallback or closest.
    let mouse_pos = [105.0, 105.0];
    let local_x = mouse_pos[0] - object_pos[0] - padding_left;
    let local_y = mouse_pos[1] - object_pos[1] - padding_top;
    let index = buffer.index_at(local_x, local_y);
    assert_eq!(index, 0);
}

#[test]
fn test_hit_testing_centered_alignment() {
    let (dummy_key, _, _) = CacheKey::new(
        cosmic_text::fontdb::ID::dummy(), 
        0, 
        16.0, 
        (0.0, 0.0), 
        Weight::NORMAL, 
        cosmic_text::CacheKeyFlags::empty()
    );

    // Buffer is 100 wide, content is 30 wide.
    // Centered: offset = (100 - 30) / 2 = 35. 
    // Glyph 0 at local content X=0 -> Global content X=35.
    let buffer = ShapedBuffer::new(
        vec![
            ShapedGlyph { key: dummy_key, cluster: 0, x: 0.0, y: 0.0, width: 10.0, height: 20.0 },
            ShapedGlyph { key: dummy_key, cluster: 1, x: 10.0, y: 0.0, width: 10.0, height: 20.0 },
            ShapedGlyph { key: dummy_key, cluster: 2, x: 20.0, y: 0.0, width: 10.0, height: 20.0 },
        ],
        vec![LineInfo { x: 35.0, y: 0.0, width: 30.0 }],
        100.0,
        100.0,
    );

    // Hit at local content X=40 (Over glyph 0)
    let index = buffer.index_at(40.0, 5.0);
    assert_eq!(index, 0);

    // Hit at local content X=60 (Over glyph 2)
    let index = buffer.index_at(60.0, 5.0);
    assert_eq!(index, 2);
}

#[test]
fn test_hit_testing_vertical_alignment() {
    let (dummy_key, _, _) = CacheKey::new(
        cosmic_text::fontdb::ID::dummy(), 
        0, 
        16.0, 
        (0.0, 0.0), 
        Weight::NORMAL, 
        cosmic_text::CacheKeyFlags::empty()
    );

    // Buffer is 100 high, line is 20 high.
    // Bottom-aligned: line.y = 100 - 20 = 80.
    let buffer = ShapedBuffer::new(
        vec![
            ShapedGlyph { key: dummy_key, cluster: 0, x: 0.0, y: 80.0, width: 10.0, height: 20.0 },
        ],
        vec![LineInfo { x: 0.0, y: 80.0, width: 10.0 }],
        100.0,
        100.0,
    );

    // Hit at local content Y=90 (Inside the line at the bottom)
    let index = buffer.index_at(5.0, 90.0);
    assert_eq!(index, 0);

    // Hit at local content Y=10 (Top of buffer, far from line)
    // Should still return 0 (fallback to closest line)
    let index = buffer.index_at(5.0, 10.0);
    assert_eq!(index, 0);
}
