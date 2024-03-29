use super::color::Color;

pub const WHITE: Color = Color { r: 230, g: 230, b: 230, a: 255 };
pub const LIGHT_GREY: Color = Color { r: 200, g: 200, b: 200, a: 255 };
pub const GREY: Color = Color { r: 85, g: 85, b: 85, a: 255 };
pub const DARK_GREY: Color = Color { r: 50, g: 50, b: 50, a: 255 };
pub const BORDER_COLOR: Color = Color { r: 80, g: 80, b: 80, a: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
pub const SPACING: f32 = 20.0;
pub const BUTTON_PADDING: f32 = 7.0;
pub const TRANSPARENCY: u8 = 140;
pub const BLUR: i32 = 200;
pub const SHADOW_INTENSITY: i32 = 200;
pub const TEXT_INPUT_WIDTH: f32 = 200.0;

pub const GFX_DEFAULT_BUTTON_COLOR: Color = TRANSPARENT;
pub const GFX_DEFAULT_BUTTON_BORDER_COLOR: Color = TRANSPARENT;
pub const GFX_DEFAULT_HOVERED_BUTTON_COLOR: Color = GREY;
pub const GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR: Color = BORDER_COLOR;
pub const GFX_DEFAULT_BUTTON_PADDING: f32 = BUTTON_PADDING;

pub const GFX_DEFAULT_TEXT_INPUT_SHADOW_INTENSITY: i32 = SHADOW_INTENSITY;
pub const GFX_DEFAULT_TEXT_INPUT_WIDTH: f32 = TEXT_INPUT_WIDTH;
pub const GFX_DEFAULT_TEXT_INPUT_COLOR: Color = BLACK.set_a(150);
pub const GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR: Color = TRANSPARENT;
pub const GFX_DEFAULT_TEXT_INPUT_HOVER_COLOR: Color = DARK_GREY;
pub const GFX_DEFAULT_TEXT_INPUT_HOVER_BORDER_COLOR: Color = BORDER_COLOR;
pub const GFX_DEFAULT_TEXT_INPUT_PADDING: f32 = 2.0;
