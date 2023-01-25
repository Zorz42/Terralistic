#![allow(non_snake_case)]

mod menus;
mod game;

use std::sync::Mutex;
use crate::menus::MenuBack;
use graphics as gfx;

fn main() {
    let mut graphics = gfx::init(
        1130,
        700,
        String::from("Terralistic"),
        include_bytes!("../Build/Resources/font.opa").to_vec(),
    );
    graphics.renderer.set_min_window_size(
        graphics.renderer.get_window_width(),
        graphics.renderer.get_window_height(),
    );

    let mut menu_back = MenuBack::new();

    menus::run_main_menu(&mut graphics, &mut menu_back);
}
