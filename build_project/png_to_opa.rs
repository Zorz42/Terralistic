use crate::libraries::graphics as gfx;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[must_use]
pub fn png_file_to_opa_bytes(png_path: PathBuf) -> Vec<u8> {
    let decoder = png::Decoder::new(File::open(png_path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    // create surface from pixels
    let mut surface = gfx::Surface::new(info.width, info.height);
    for y in 0..info.height {
        for x in 0..info.width {
            let index = (y * info.width + x) as usize * 4;
            let color = gfx::Color {
                r: bytes[index],
                g: bytes[index + 1],
                b: bytes[index + 2],
                a: bytes[index + 3],
            };
            surface.set_pixel(x as i32, y as i32, color).unwrap();
        }
    }
    // serialize surface and write to file
    surface.serialize_to_bytes().unwrap()
}

pub fn png_file_to_opa_file(input_file: PathBuf, output_file: PathBuf) {
    let serialized = png_file_to_opa_bytes(input_file);

    let mut file = File::create(output_file).unwrap();
    file.write_all(&serialized).unwrap();
}
