#![allow(non_snake_case)]

mod menus;
mod game;

use graphics as gfx;
use shared::{blocks, resource_path};
use shared_mut;
use events;
use shared_mut::SharedMut;

/*
This struct is an event.
*/
pub struct CustomEvent {
    pub value: i32,
}

impl events::Event for CustomEvent {}

/*
This struct is an event sender.
 */
pub struct CustomEventSender {
    pub event_sender: events::Sender<CustomEvent>,
}

impl CustomEventSender {
    /*
    Creates a new event sender.
    */
    pub fn new() -> Self {
        CustomEventSender {
            event_sender: events::Sender::new(),
        }
    }
}

/*
This is another custom event sender.
*/
pub struct CustomEventSender2 {
    pub event_sender: events::Sender<CustomEvent>,
}

impl CustomEventSender2 {
    /*
    Creates a new event sender.
    */
    pub fn new() -> Self {
        CustomEventSender2 {
            event_sender: events::Sender::new(),
        }
    }
}

/*
This struct is an event listener.
*/
pub struct CustomEventListener {
    pub value: i32,
}

impl events::Listener<CustomEvent> for CustomEventListener {
    fn on_event(&mut self, event: &CustomEvent) {
        println!("Custom event received: {}", event.value);
        self.value = event.value;
    }
}


fn main() {
    println!("Hello, this is client!");

    let mut event_sender = CustomEventSender::new();
    let mut event_sender2 = CustomEventSender2::new();

    let event_listener = SharedMut::new(CustomEventListener { value: 10 });

    event_sender.event_sender.add_listener(&event_listener);
    let event = CustomEvent { value: 42 };
    event_sender.event_sender.send(&event);
    println!("event_listener.value = {}", event_listener.get().value);
    event_sender2.event_sender.send(&event);

    let mut graphics = gfx::init(1130, 700, String::from("Terralistic"));

    // create a texture that is loaded from surface which is loaded from /Users/jakob/Terralistic-rust/Build/Resources/background.opa
    let mut background = gfx::Texture::new();
    let mut surface = gfx::Surface::deserialize(include_bytes!("../Build/Resources/background.opa").to_vec());
    surface.set_pixel(0, 0, gfx::Color { r: 255, g: 0, b: 0, a: 255 });
    surface.set_pixel(1, 0, gfx::Color { r: 255, g: 0, b: 0, a: 255 });
    surface.set_pixel(0, 1, gfx::Color { r: 255, g: 0, b: 0, a: 255 });
    surface.set_pixel(1, 1, gfx::Color { r: 255, g: 0, b: 0, a: 255 });
    background.load_from_surface(&mut surface);


    while graphics.renderer.is_window_open() {
        /*for event in graphics.events.get_events() {

        }*///comment when it's not implemented, don't push uncompilable code

        graphics.renderer.pre_render();

        let rect = gfx::Rect{x: 10, y: 10, w: 100, h: 100};
        rect.render(&graphics.renderer, gfx::Color { r: 255, g: 255, b: 0, a: 255 });

        // draw background
        background.render(&graphics.renderer, 3.0, 10, 10, gfx::Rect{x: 0, y: 0, w: background.get_texture_width(), h: background.get_texture_height()}, false, gfx::Color{r: 255, g: 255, b: 255, a: 255});

        //background.render(&graphics.renderer, 1.0, 10 + background.get_texture_width(), 10, gfx::RectShape{x: 0, y: 0, w: background.get_texture_width(), h: background.get_texture_height()}, false, gfx::Color{r: 255, g: 255, b: 255, a: 255});
        //background.render(&graphics.renderer, 1.0, 1130 / 2, 10, gfx::RectShape{x: 0, y: 0, w: background.get_texture_width(), h: background.get_texture_height()}, false, gfx::Color{r: 255, g: 255, b: 255, a: 255});

        graphics.renderer.post_render();
    }
}
