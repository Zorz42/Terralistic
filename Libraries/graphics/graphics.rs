// enable a bunch of clippy lints
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::assertions_on_result_states)]
#![warn(clippy::create_dir)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::deref_by_slicing)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exit)]
#![warn(clippy::expect_used)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::mem_forget)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::single_char_lifetime_names)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::todo)]
#![warn(clippy::try_err)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unreachable)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::verbose_file_reads)]

mod color;
pub use color::Color;

mod theme;
pub use theme::{
    BLACK, BLUR, BORDER_COLOR, BUTTON_PADDING, DARK_GREY, GREY, LIGHT_GREY, SHADOW_INTENSITY,
    SPACING, TEXT_INPUT_WIDTH, TRANSPARENCY, TRANSPARENT, WHITE,
};

mod transformation;

mod vertex_buffer;

mod blend_mode;
pub use blend_mode::{set_blend_mode, BlendMode};

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
pub use container::{
    Container, Orientation, BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, CENTER, LEFT, RIGHT, TOP, TOP_LEFT,
    TOP_RIGHT,
};

mod render_rect;
pub use render_rect::RenderRect;

mod button;
pub use button::Button;

mod text_input;
pub use text_input::TextInput;

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

pub fn init(
    window_width: i32,
    window_height: i32,
    window_title: &str,
    default_font_data: &[u8],
) -> GraphicsContext {
    GraphicsContext {
        renderer: Renderer::new(window_width, window_height, window_title),
        font: Font::new(default_font_data),
    }
}
