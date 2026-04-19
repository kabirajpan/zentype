use zentype::prelude::*;

#[test]
fn test_options_builder_integration() {
    let opts = TextOptions::new()
        .at(10.0, 10.0)
        .font_size(32.0)
        .color(Color::RED);
    
    assert_eq!(opts.font_size, 32.0);
    assert_eq!(opts.color, Color::RED);
}

#[test]
fn test_color_hex_integration() {
    let c = Color::hex("#FF00FF");
    assert_eq!(c, Color::rgb(255, 0, 255));
}
