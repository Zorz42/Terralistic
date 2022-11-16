#![allow(non_snake_case)]

use graphics as gfx;
use shared;

fn main() {
    println!("Hello, this is client!");

    let mut graphics = gfx::init(1130, 700, String::from("Terralistic"));

    while graphics.renderer.is_window_open() {
        for event in graphics.events.get_events() {

        }

        graphics.renderer.pre_render();

        graphics.renderer.post_render();
    }
}
