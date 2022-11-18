use crate::surface;
use snap;

pub fn read_opa(path: String) /*-> surface::Surface*/ {
    let compressed_data = std::fs::read(path.as_str()).unwrap();
    let mut decompressed_data = vec![];

    let mut frame_decoder = snap::read::FrameDecoder::new(compressed_data.as_slice());
    std::io::copy(&mut frame_decoder, &mut decompressed_data).expect("Decompress operation failed");

    /*data = decompress(data);

    int width = *(int*)&data[0];
    int height = *(int*)&data[sizeof(int)];

    data.erase(data.begin(), data.begin() + sizeof(int) * 2);

    if(data.size() != width * height * 4)
        throw OpaFileError("OPA data size error file size is " + std::to_string(data.size()) + " but should be " + std::to_string(width * height * 4));

    gfx::Surface surface;
    std::vector<unsigned char> unsigned_data(data.size());
    for(int i = 0; i < (int)data.size(); i++)
        unsigned_data[i] = data[i];
    surface.loadFromBuffer(unsigned_data, width, height);
    return surface;*/
}