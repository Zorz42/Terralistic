use crate::color::Color;

pub const WHITE: Color = Color{r: 230, g: 230, b: 230, a: 255};
pub const LIGHT_GREY: Color = Color{r: 200, g: 200, b: 200, a: 255};
pub const GREY: Color = Color{r: 85, g: 85, b: 85, a: 255};
pub const DARK_GREY: Color = Color{r: 50, g: 50, b: 50, a: 255};
pub const BORDER_COLOR: Color = Color{r: 100, g: 100, b: 100, a: 255};
pub const BLACK: Color = Color{r: 0, g: 0, b: 0, a: 255};
pub const TRANSPARENT: Color = Color{r: 0, g: 0, b: 0, a: 0};
pub const SPACING: i32 = 20;
pub const BUTTON_MARGIN: i32 = 7;
pub const TRANSPARENCY: u8 = 140;
pub const BLUR: i32 = 200;
pub const SHADOW_INTENSITY: i32 = 200;
pub const TEXT_INPUT_WIDTH: i32 = 200;

pub(crate) const GFX_DEFAULT_BUTTON_COLOR: Color = TRANSPARENT;
pub(crate) const GFX_DEFAULT_BUTTON_BORDER_COLOR: Color = TRANSPARENT;
pub(crate) const GFX_DEFAULT_HOVERED_BUTTON_COLOR: Color = GREY;
pub(crate) const GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR: Color = BORDER_COLOR;
pub(crate) const GFX_DEFAULT_TEXT_COLOR: Color = WHITE;
pub(crate) const GFX_SHADOW_BLUR: i32 = BLUR;
pub(crate) const GFX_DEFAULT_BUTTON_MARGIN: i32 = BUTTON_MARGIN;
pub(crate) const GFX_DEFAULT_TEXT_BOX_SHADOW_INTENSITY: i32 = SHADOW_INTENSITY;
pub(crate) const GFX_DEFAULT_TEXT_INPUT_WIDTH: i32 = TEXT_INPUT_WIDTH;
