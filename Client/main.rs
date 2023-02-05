#![allow(non_snake_case)]

use graphics as gfx;

use crate::menus::MenuBack;

mod game;
mod menus;

fn main() {
    let mut graphics = gfx::init(
        1130,
        700,
        "Terralistic",
        include_bytes!("../Build/Resources/font.opa"),
    );
    graphics.renderer.set_min_window_size(
        graphics.renderer.get_window_width(),
        graphics.renderer.get_window_height(),
    );

    let mut menu_back = MenuBack::new();

    menus::run_main_menu(&mut graphics, &mut menu_back);
}
