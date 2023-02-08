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
// disable some Clippy lints
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::similar_names)]

use crate::libraries::graphics as gfx;

use crate::client::menus::{run_main_menu, MenuBack};

pub mod libraries {
    pub mod events;
    pub mod graphics;
}

pub mod shared;

pub mod server {
    pub mod server_impl;
}

pub mod client {
    pub mod game;
    pub mod menus;
}

fn main() {
    let graphics_result = gfx::init(
        1130,
        700,
        "Terralistic",
        include_bytes!("Build/Resources/font.opa"),
    );

    let mut graphics;

    match graphics_result {
        Ok(g) => graphics = g,
        Err(e) => {
            println!("Failed to initialize graphics: {e}");
            return;
        }
    }

    if graphics
        .renderer
        .set_min_window_size(
            graphics.renderer.get_window_width(),
            graphics.renderer.get_window_height(),
        )
        .is_err()
    {
        println!("Failed to set minimum window size");
    }

    let mut menu_back = MenuBack::new();

    run_main_menu(&mut graphics, &mut menu_back);
}
