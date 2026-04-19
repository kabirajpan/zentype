use crate::types::color::Color;

pub struct TextOptions {
    // position
    pub x: f32,
    pub y: f32,

    // text style
    pub font_size: f32,
    pub color: Color,
    pub font_family: Option<String>,
    pub font_weight: FontWeight,

    // background
    pub bg_color: Option<Color>,
    pub bg_padding: f32,
    pub full_width_bg: bool, // editor-style line highlight

    // layout
    pub max_width: Option<f32>,
    pub line_height: f32,
    pub wrap: TextWrap,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            font_size: 16.0,
            color: Color::WHITE,
            font_family: None,
            font_weight: FontWeight::Regular,
            bg_color: None,
            bg_padding: 4.0,
            full_width_bg: false,
            max_width: None,
            line_height: 1.5,
            wrap: TextWrap::Word,
        }
    }
}

pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
}

pub enum TextWrap {
    Word,
    Character,
    None,
}
