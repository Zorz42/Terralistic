use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use crate::libraries::graphics as gfx;

#[must_use]
pub fn png_file_to_opa_bytes(png_path: PathBuf) -> Vec<u8> {
    // png 0.18 wants a BufRead rather than a bare Read
    let file = File::open(&png_path).unwrap_or_else(|e| panic!("could not open png {}: {e}", png_path.display()));
    let decoder = png::Decoder::new(BufReader::new(file));
    let mut reader = decoder.read_info().unwrap();
    // png 0.18 returns None here when the required buffer would not fit in a usize
    let buffer_size = reader.output_buffer_size().unwrap_or_else(|| panic!("png {} is too large to decode", png_path.display()));
    let mut buf = vec![0; buffer_size];
    let info = reader.next_frame(&mut buf).unwrap_or_else(|e| panic!("could not decode png {}: {e}", png_path.display()));
    let bytes = buf.get(..info.buffer_size()).unwrap_or_else(|| panic!("png {} decoded to an unexpected size", png_path.display()));
    // create surface from pixels
    let mut surface = gfx::Surface::new(gfx::IntSize(info.width, info.height));
    for y in 0..info.height {
        for x in 0..info.width {
            let index = (y * info.width + x) as usize * 4;
            let color = gfx::Color {
                r: bytes[index],
                g: bytes[index + 1],
                b: bytes[index + 2],
                a: bytes[index + 3],
            };
            *surface.get_pixel_mut(gfx::IntPos(x as i32, y as i32)).unwrap() = color;
        }
    }
    // serialize surface and write to file
    surface.serialize_to_bytes().unwrap()
}

pub fn png_file_to_opa_file(input_file: PathBuf, output_file: PathBuf) {
    let serialized = png_file_to_opa_bytes(input_file);

    let mut file = File::create(&output_file).unwrap_or_else(|e| panic!("could not create {}: {e}", output_file.display()));
    file.write_all(&serialized).unwrap();
}
