#include <fstream>
#include <vector>
#include "readOpa.hpp"
#include "exception.hpp"
#include "compress.hpp"

gfx::Surface readOpa(const std::string& path) {
    std::ifstream opa_file(path, std::ios::binary);
    
    if(!opa_file.is_open() || (opa_file.eof() && opa_file.fail()))
        throw OpaFileError("Could not open file: " + path);

    std::vector<char> data = std::vector<char>((std::istreambuf_iterator<char>(opa_file)), std::istreambuf_iterator<char>());
    data = decompress(data);
    
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
    return surface;
}
