#![allow(non_snake_case)]

use graphics as gfx;
use shared; //not needed, leave it in to get autocomplete in shared


fn main() {
    println!("Hello, this is client!");

    let mut graphics = gfx::init(1130, 700, String::from("Terralistic"));

    // create a texture that is loaded from surface which is loaded from /Users/jakob/Terralistic-rust/Build/Resources/background.opa
    //let mut background = gfx::Texture::new();
    //let mut surface = gfx::read_opa(String::from("/Users/jakob/Terralistic-rust/Build/Resources/background.opa"));
    //surface.set_pixel(0, 0, gfx::Color { r: 0, g: 255, b: 0, a: 255 });
    //background.load_from_surface(&mut surface);


    while graphics.renderer.is_window_open() {
        for event in graphics.events.get_events() {

        }

        graphics.renderer.pre_render();

        let rect = gfx::RectShape{x: 10, y: 10, w: 100, h: 100};
        rect.render(&graphics.renderer, gfx::Color { r: 255, g: 255, b: 0, a: 255 });

        // draw background
        //background.render(&graphics.renderer, 1.0, 0, 0, gfx::RectShape{x: 0, y: 0, w: background.get_texture_width(), h: background.get_texture_height()}, false, gfx::Color{r: 255, g: 255, b: 255, a: 255});

        graphics.renderer.post_render();
    }
}
