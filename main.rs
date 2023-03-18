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
#![allow(clippy::ptr_as_ptr)]
#![allow(clippy::shadow_unrelated)]

use crate::libraries::graphics as gfx;
use core::sync::atomic::AtomicBool;
use std::sync::Mutex;
extern crate alloc;
use alloc::sync::Arc;
use std::thread::sleep;
use core::time::Duration;

use crate::client::menus::{run_main_menu, MenuBack};
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
    pub mod menus;
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
            .set_min_window_size(graphics.renderer.get_window_size())
            .is_err()
        {
            println!("Failed to set minimum window size");
        }
        Some(graphics)
    };



    let server_running = Arc::new(AtomicBool::new(true));
    let server_running_clone = server_running.clone();

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

    let (event_sender, event_receiver) = std::sync::mpsc::channel();
    let gfx_is_some = server_graphics_context.is_some();

    let server_thread = std::thread::spawn(move || {
        let mut server = if gfx_is_some {
            Server::new(MULTIPLAYER_PORT, Some(event_sender))
        } else {
            Server::new(MULTIPLAYER_PORT, None)
        };
        let result = server.start(
            &server_running_clone,
            &loading_text,
            vec![include_bytes!("base_game/base_game.mod").to_vec()],
            &path.join("server.world"),
        );

        if let Err(e) = result {
            println!("Server stopped with an error: {e}");
        }
    });

    if let Some(graphics) = server_graphics_context {
        let mut manager = UiManager::new(graphics, event_receiver);
        manager.run(server_thread, &server_running);
    } else {
        loop {
            if server_thread.is_finished() {
                break;
            }
            sleep(Duration::from_millis(50));
        }
        let _thread_result = server_thread.join();
    }
}

fn client_main() {
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
        .set_min_window_size(graphics.renderer.get_window_size())
        .is_err()
    {
        println!("Failed to set minimum window size");
    }

    let mut menu_back = MenuBack::new(&mut graphics);

    run_main_menu(&mut graphics, &mut menu_back);
}
