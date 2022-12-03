use crate::{Color};
use serde_derive::{Serialize, Deserialize};

/*
Surface is an image stored in ram.
*/
#[derive(Serialize, Deserialize)]
pub struct Surface {
    pub(crate) pixels: Vec<u8>,
    width: i32,
    height: i32,
}


impl Surface {
    /*
    Creates a new surface with all transparent pixels.
    */
    pub fn new(width: i32, height: i32) -> Self {
        if width < 0 || height < 0 {
            panic!("Dimensions negative");
        }

        Surface {
            pixels: std::vec![0; (width * height * 4).try_into().unwrap()],
            width,
            height,
        }
    }

    /*
    Serializes the surface into a vector of bytes.
    It is serialized with bincode and compressed with snap.
    */
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        bincode::serialize_into(&mut buffer, &self).unwrap();
        snap::raw::Encoder::new().compress_vec(&buffer).unwrap()
    }

    /*
    Deserializes a surface from a vector of bytes the same way it was serialized.
    */
    pub fn deserialize(buffer: Vec<u8>) -> Self {
        let decompressed = snap::raw::Decoder::new().decompress_vec(&buffer).unwrap();
        bincode::deserialize(&decompressed).unwrap()
    }


    /*
    Converts 2D location to a linear location in color array.
    The index points to the red bit of the color and the next
    three to green, blue, alpha.
    */
    fn get_index(&self, x: i32, y: i32) -> usize {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            panic!("Pixel out of bounds");
        }

        (y * self.width + x) as usize * 4
    }

    /*
    Retrieves the pixel color on a specified location.
    */
    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        let index = self.get_index(x, y);
        Color{
            r: self.pixels[index],
            g: self.pixels[index + 1],
            b: self.pixels[index + 2],
            a: self.pixels[index + 3],
        }
    }

    /*
    Sets the pixel color on a specified location.
    */
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        let index = self.get_index(x, y);
        self.pixels[index] = color.r;
        self.pixels[index + 1] = color.g;
        self.pixels[index + 2] = color.b;
        self.pixels[index + 3] = color.a;
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    /*
        Copies another surface to the specified location.
    */
    pub fn draw(&mut self, x: i32, y: i32, surface: &Self, color: Color) {
        for xpos in 0..surface.get_width() {
            for ypos in 0..surface.get_height() {
                let surface_color = surface.get_pixel(xpos, ypos);
                self.set_pixel(xpos + x, ypos + y, Color {
                    r: (surface_color.r as f32 * (color.r as f32 / 255.0)) as u8,
                    g: (surface_color.g as f32 * (color.g as f32 / 255.0)) as u8,
                    b: (surface_color.b as f32 * (color.b as f32 / 255.0)) as u8,
                    a: (surface_color.a as f32 * (color.a as f32 / 255.0)) as u8,
                });
            }
        }
    }
}