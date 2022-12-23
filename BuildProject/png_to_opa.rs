use std::fs::File;
use std::io::Write;
use graphics as gfx;

pub fn png_to_opa(input_file: String, output_file: String) {
    // read png file and store pixels in a vector
    let decoder = png::Decoder::new(File::open(input_file).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    // create surface from pixels
    let mut surface = gfx::Surface::new(info.width as i32, info.height as i32);
    for y in 0..info.height {
        for x in 0..info.width {
            let index = (y * info.width + x) as usize * 4;
            let color = gfx::Color {
                r: bytes[index],
                g: bytes[index + 1],
                b: bytes[index + 2],
                a: bytes[index + 3],
            };
            surface.set_pixel(x as i32, y as i32, color);
        }
    }
    // serialize surface and write to file
    let serialized = surface.serialize();
    let mut file = File::create(output_file).unwrap();
    file.write_all(&serialized).unwrap();
}