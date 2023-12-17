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
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::if_not_else)]
#![warn(clippy::from_iter_instead_of_collect)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::ignored_unit_patterns)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::impl_trait_in_params)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::large_digit_groups)]
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::manual_map)]
#![warn(clippy::manual_non_exhaustive)]
#![warn(clippy::implicit_clone)]
#![warn(clippy::inconsistent_struct_constructor)]
#![warn(clippy::index_refutable_slice)]
#![warn(clippy::manual_assert)]
#![warn(clippy::manual_clamp)]
#![warn(clippy::manual_instant_elapsed)]
#![warn(clippy::manual_let_else)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::manual_string_new)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_on_vec_items)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::match_wild_err_arm)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::mut_mut)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::naive_bytecount)]
#![warn(clippy::needless_bitwise_bool)]
#![warn(clippy::needless_collect)]
#![warn(clippy::needless_continue)]
#![warn(clippy::needless_for_each)]
#![warn(clippy::needless_pass_by_ref_mut)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::needless_raw_string_hashes)]
#![warn(clippy::needless_raw_strings)]
#![warn(clippy::no_effect_underscore_binding)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::option_option)]
#![warn(clippy::pub_without_shorthand)]
#![warn(clippy::range_minus_one)]
#![warn(clippy::range_plus_one)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::redundant_clone)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::redundant_else)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::redundant_type_annotations)]
#![warn(clippy::ref_binding_to_reference)]
#![warn(clippy::ref_option_ref)]
#![warn(clippy::ref_patterns)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::return_self_not_must_use)]
#![warn(clippy::same_functions_in_if_condition)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::semicolon_inside_block)]
#![warn(clippy::stable_sort_primitive)]
#![warn(clippy::unnecessary_box_returns)]
#![warn(clippy::unnecessary_join)]
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unnecessary_self_imports)]
#![warn(clippy::unnecessary_struct_initialization)]
#![warn(clippy::unnecessary_wraps)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unnested_or_patterns)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::unused_async)]
#![warn(clippy::unused_peekable)]
#![warn(clippy::unused_rounding)]
#![warn(clippy::unused_self)]
#![warn(clippy::verbose_file_reads)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::zero_sized_map_values)]
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
#![allow(clippy::ptr_as_ptr)]
#![allow(clippy::shadow_unrelated)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::significant_drop_tightening)]
#![allow(clippy::new_without_default)]
#![allow(clippy::module_inception)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::todo)]
#![allow(clippy::unimplemented)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![windows_subsystem = "windows"]

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

use directories::BaseDirs;

use crate::client::global_settings::GlobalSettings;
use crate::client::menus::{run_main_menu, MenuBack};
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use crate::server::server_core::{Server, MULTIPLAYER_PORT};
use crate::server::server_ui::UiManager;

pub mod libraries {
    pub mod events;
    pub mod graphics;
}

pub mod shared;

pub mod server {
    pub mod server_core;
    pub mod server_ui;
}

pub mod client {
    pub mod game;
    pub mod global_settings;
    pub mod menus;
    pub mod settings;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    args.get(1).map_or_else(
        || {
            client_main();
        },
        |arg| {
            if arg == "server" {
                server_main(args.as_slice());
            } else if arg == "client" {
                client_main();
            } else if arg == "version" {
                println!("{}", shared::versions::VERSION);
            } else {
                println!("Invalid argument: {arg}");
            }
        },
    );
}

fn server_main(args: &[String]) {
    let server_graphics_context = if args.contains(&"nogui".to_owned()) {
        None
    } else {
        let graphics_result = gfx::init(
            1130,
            700,
            "Terralistic Server",
            include_bytes!("Build/Resources/font.opa"),
            Some(include_bytes!("Build/Resources/font_mono.opa")),
        );

        let mut graphics;

        match graphics_result {
            Ok(g) => graphics = g,
            Err(e) => {
                println!("Failed to initialize graphics: {e}");
                return;
            }
        }

        if graphics.renderer.set_min_window_size(graphics.renderer.get_window_size()).is_err() {
            println!("Failed to set minimum window size");
        }
        Some(graphics)
    };

    let server_running = Arc::new(AtomicBool::new(true));

    let loading_text = Arc::new(Mutex::new("Loading".to_owned()));

    let curr_dir = std::env::current_dir();
    let curr_dir = match curr_dir {
        Ok(path) => path,
        Err(e) => {
            println!("Couldn't get current directory: {e}");
            return;
        }
    };

    let path = curr_dir.join("server_data");
    let path_clone = path.clone();

    let (srv_to_ui_event_sender, srv_to_ui_event_receiver) = std::sync::mpsc::channel();
    let (ui_to_srv_event_sender, ui_to_srv_event_receiver) = std::sync::mpsc::channel();

    let mut server = if server_graphics_context.is_some() {
        Server::new(MULTIPLAYER_PORT, Some(ui_to_srv_event_receiver), Some(srv_to_ui_event_sender))
    } else {
        Server::new(MULTIPLAYER_PORT, None, None)
    };

    if let Some(graphics) = server_graphics_context {
        let mut manager = UiManager::new(server, graphics, srv_to_ui_event_receiver, ui_to_srv_event_sender, path_clone);
        let res = manager.run(&server_running, &loading_text, vec![include_bytes!("base_game/base_game.mod").to_vec()], &path.join("server.world"));
        if let Err(e) = res {
            println!("Server stopped with an error: {e}");
        }
    } else {
        let res = server.run(&server_running, &loading_text, vec![include_bytes!("base_game/base_game.mod").to_vec()], &path.join("server.world"));
        if let Err(e) = res {
            println!("Server stopped with an error: {e}");
        }
    }
}

fn client_main() {
    let graphics_result = gfx::init(1670, 1050, "Terralistic", include_bytes!("Build/Resources/font.opa"), None);

    let mut graphics;

    match graphics_result {
        Ok(g) => graphics = g,
        Err(e) => {
            println!("Failed to initialize graphics: {e}");
            return;
        }
    }

    if graphics.renderer.set_min_window_size(gfx::FloatSize(1130.0, 700.0)).is_err() {
        println!("Failed to set minimum window size");
    }

    let mut menu_back = MenuBack::new(&graphics);

    let base_dirs;
    if let Some(base_dirs_) = BaseDirs::new() {
        base_dirs = base_dirs_;
    } else {
        return;
    }

    let mut settings = Settings::new(base_dirs.data_dir().join("Terralistic").join("settings.txt"));

    let mut global_settings = GlobalSettings::new();
    global_settings.init(&mut settings);
    global_settings.update(&mut graphics, &settings);

    run_main_menu(&mut graphics, &mut menu_back, &mut settings, &mut global_settings);

    global_settings.stop(&mut settings);

    if let Err(error) = settings.save_config() {
        println!("Failed to save settings to config file: {error}");
    }
}
