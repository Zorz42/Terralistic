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

        let rect = gfx::RectShape{x: 0, y: 0, w: 1, h: 1};
        rect.render(&graphics.renderer, gfx::Color { r: 255, g: 0, b: 0, a: 255 });

        graphics.renderer.post_render();
    }
}
