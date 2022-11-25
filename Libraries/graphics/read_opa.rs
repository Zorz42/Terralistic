use std::fs::File;
use std::io::Read;
use crate::surface;
use crate::color;
use snap::raw::decompress_len;
use snap::raw::Decoder;
use snap;

pub fn read_opa(path: String) -> surface::Surface {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // decompress buffer into decompressed with snap
    let decompressed_size = decompress_len(&buffer).unwrap();
    let mut decompressed = vec![0; decompressed_size];
    // create new snap decoder
    let mut decoder = Decoder::new();
    decoder.decompress(&buffer, &mut decompressed).unwrap();

    // read with and height from first 8 bytes and then the rest is the data
    let width = u32::from_le_bytes([decompressed[0], decompressed[1], decompressed[2], decompressed[3]]) as i32;
    let height = u32::from_le_bytes([decompressed[4], decompressed[5], decompressed[6], decompressed[7]]) as i32;

    let mut surface = surface::Surface::new(width, height);

    // iterate through x and y
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let b = decompressed[index * 4 + 8];
            let g = decompressed[index * 4 + 9];
            let r = decompressed[index * 4 + 10];
            let a = decompressed[index * 4 + 11];
            surface.set_pixel(x, y, color::Color {r, g, b, a});
        }
    }

    surface
}