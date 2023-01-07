mod color;
pub use color::Color;

mod theme;
pub use theme::{
    WHITE,
    LIGHT_GREY,
    GREY,
    DARK_GREY,
    BORDER_COLOR,
    BLACK,
    TRANSPARENT,
    SPACING,
    BUTTON_MARGIN,
    TRANSPARENCY,
    BLUR,
    SHADOW_INTENSITY,
    TEXT_INPUT_WIDTH,
};

mod transformation;

mod vertex_buffer;

mod blend_mode;
pub use blend_mode::{BlendMode, set_blend_mode};

mod shaders;

mod surface;
pub use surface::Surface;

mod blur;

mod shadow;

mod passthrough_shader;

mod renderer;
use crate::renderer::Renderer;

mod events;
pub use events::{Event, Key};

mod rect;
pub use rect::Rect;

mod texture;
pub use texture::Texture;

mod text;
pub use text::Font;

mod container;
pub use container::{Container, Orientation, TOP_LEFT, TOP, TOP_RIGHT, LEFT, CENTER, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT};

mod render_rect;
pub use render_rect::RenderRect;

mod button;
pub use button::Button;

mod sprite;
pub use sprite::Sprite;

mod texture_atlas;
pub use texture_atlas::TextureAtlas;

mod rect_array;
pub use rect_array::RectArray;

/*
A struct that will be passed all
around the functions that need drawing
*/
pub struct GraphicsContext {
    pub renderer: Renderer,
    pub font: Font,
}

pub fn init(window_width: i32, window_height: i32, window_title: String, default_font_data: Vec<u8>) -> GraphicsContext {
    GraphicsContext{
        renderer: Renderer::new(window_width, window_height, window_title),
        font: Font::new(default_font_data),
    }

}