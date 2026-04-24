use zentype::prelude::*;

#[test]
fn test_hex_parsing_standard() {
    assert_eq!(Color::hex("#FFF"), Color::rgb(255, 255, 255));
    assert_eq!(Color::hex("#FFFFFF"), Color::rgb(255, 255, 255));
    assert_eq!(Color::hex("#000"), Color::rgb(0, 0, 0));
    assert_eq!(Color::hex("#000000"), Color::rgb(0, 0, 0));
}

#[test]
fn test_hex_parsing_alpha() {
    assert_eq!(Color::hex("#F00F"), Color::rgba(255, 0, 0, 255));
    assert_eq!(Color::hex("#FF0000FF"), Color::rgba(255, 0, 0, 255));
    assert_eq!(Color::hex("#0000"), Color::rgba(0, 0, 0, 0));
}

#[test]
fn test_hex_parsing_no_hash() {
    assert_eq!(Color::hex("FFF"), Color::WHITE);
    assert_eq!(Color::hex("FF0000"), Color::rgb(255, 0, 0));
}

#[test]
fn test_hex_parsing_invalid() {
    // Should fallback to BLACK for invalid lengths/formats
    assert_eq!(Color::hex(""), Color::BLACK);
    assert_eq!(Color::hex("#FF"), Color::BLACK);
    assert_eq!(Color::hex("#FFFFF"), Color::BLACK);
}

#[test]
fn test_color_utilities() {
    let red = Color::RED;
    assert_eq!(red.r, 255);
    assert_eq!(red.g, 59);
    assert_eq!(red.b, 48);
    
    let transparent_red = red.with_alpha(100);
    assert_eq!(transparent_red.a, 100);
    assert_eq!(transparent_red.r, 255);
}

#[test]
fn test_color_conversions() {
    let black = Color::BLACK;
    let u32_val = black.to_u32();
    assert_eq!(u32_val, 0x000000FF); // R=0, G=0, B=0, A=255
    
    let white = Color::WHITE;
    assert_eq!(white.to_u32(), 0xFFFFFFFF);
    
    let f32_arr = white.to_f32_array();
    assert_eq!(f32_arr, [1.0, 1.0, 1.0, 1.0]);
}
