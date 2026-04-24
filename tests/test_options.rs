use zentype::prelude::*;

#[test]
fn test_options_defaults() {
    let opts = TextOptions::default();
    assert_eq!(opts.font_size, 16.0);
    assert_eq!(opts.color, Color::WHITE);
    assert_eq!(opts.line_height, 1.5);
    assert_eq!(opts.padding.top, 0.0);
    assert_eq!(opts.align, None);
    assert_eq!(opts.valign, None);
}

#[test]
fn test_options_builder() {
    let opts = TextOptions::new()
        .at(10.0, 20.0)
        .font_size(24.0)
        .color(Color::RED)
        .padding_all(5.0)
        .align(HorizontalAlignment::Center)
        .valign(VerticalAlignment::Bottom);
        
    assert_eq!(opts.x, 10.0);
    assert_eq!(opts.y, 20.0);
    assert_eq!(opts.font_size, 24.0);
    assert_eq!(opts.color, Color::RED);
    assert_eq!(opts.padding.top, 5.0);
    assert_eq!(opts.padding.bottom, 5.0);
    assert_eq!(opts.align, Some(HorizontalAlignment::Center));
    assert_eq!(opts.valign, Some(VerticalAlignment::Bottom));
}

#[test]
fn test_options_padding_specifics() {
    let opts = TextOptions::new()
        .padding_horizontal(10.0)
        .padding_top(5.0);
        
    assert_eq!(opts.padding.left, 10.0);
    assert_eq!(opts.padding.right, 10.0);
    assert_eq!(opts.padding.top, 5.0);
    assert_eq!(opts.padding.bottom, 0.0);
}

#[test]
fn test_options_as_attrs() {
    let opts = TextOptions::new()
        .font_weight(FontWeight::Bold)
        .font_style(FontStyle::Italic);
        
    let attrs = opts.as_attrs();
    // We can't easily check internal cosmic_text::Attrs without public accessors,
    // but we can verify our conversion logic doesn't panic.
    assert_eq!(format!("{:?}", attrs.weight), "Weight(700)");
    assert_eq!(format!("{:?}", attrs.style), "Italic");
}
