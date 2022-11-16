#![allow(non_snake_case)]

use graphics;
use shared;

fn main() {
    println!("Hello, this is client!");
    graphics::init(1130, 700, String::from("Terralistic"));
}
