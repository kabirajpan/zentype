use zentype::prelude::*;

#[test]
fn test_color_hex_full_range() {
    // 3 digits
    assert_eq!(Color::hex("#000"), Color::rgb(0, 0, 0));
    assert_eq!(Color::hex("#F0A"), Color::rgb(255, 0, 170));

    // 4 digits (with alpha)
    assert_eq!(Color::hex("#0000"), Color::rgba(0, 0, 0, 0));
    assert_eq!(Color::hex("#F0A8"), Color::rgba(255, 0, 170, 136));

    // 6 digits
    assert_eq!(Color::hex("#000000"), Color::rgb(0, 0, 0));
    assert_eq!(Color::hex("#FF5733"), Color::rgb(255, 87, 51));

    // 8 digits
    assert_eq!(Color::hex("#00000000"), Color::rgba(0, 0, 0, 0));
    assert_eq!(Color::hex("#FF573380"), Color::rgba(255, 87, 51, 128));
}

#[test]
fn test_color_utilities() {
    let color = Color::hex("#FF5733");

    // to_u32
    assert_eq!(color.to_u32(), 0xFF5733FF);

    // with_alpha
    let transparent_color = color.with_alpha(0);
    assert_eq!(transparent_color.a, 0);
    assert_eq!(transparent_color.to_u32(), 0xFF573300);
}

#[test]
fn test_text_options_complex_builder() {
    let opts = TextOptions::new()
        .at(100.0, 100.0)
        .font_size(32.0)
        .color(Color::BLUE)
        .font_family("JetBrains Mono")
        .font_weight(FontWeight::Bold)
        .bg(Color::BLACK)
        .padding(10.0)
        .full_width(true)
        .max_width(500.0)
        .line_height(1.8)
        .wrap(TextWrap::Character)
        .align(HorizontalAlignment::Right);

    assert_eq!(opts.x, 100.0);
    assert_eq!(opts.y, 100.0);
    assert_eq!(opts.font_size, 32.0);
    assert_eq!(opts.color, Color::BLUE);
    assert_eq!(opts.font_family, Some("JetBrains Mono".to_string()));
    assert_eq!(opts.font_weight, FontWeight::Bold);
    assert_eq!(opts.bg_color, Some(Color::BLACK));
    assert_eq!(opts.padding, Padding::all(10.0));
    assert!(opts.full_width_bg);
    assert_eq!(opts.max_width, Some(500.0));
    assert_eq!(opts.line_height, 1.8);
    assert_eq!(opts.wrap, TextWrap::Character);
    assert_eq!(opts.align, Some(HorizontalAlignment::Right));
}

#[test]
fn test_color_constants() {
    assert_eq!(Color::WHITE, Color::rgb(255, 255, 255));
    assert_eq!(Color::BLACK, Color::rgb(0, 0, 0));
    assert_eq!(Color::TRANSPARENT, Color::rgba(0, 0, 0, 0));
    // Verify some premium color constants
    assert_eq!(Color::RED, Color::rgb(255, 59, 48));
}
