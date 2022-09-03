#include <fstream>
#include <vector>
#include "readOpa.hpp"
#include "exception.hpp"

gfx::Surface readOpa(const std::string& path) {
    std::ifstream opa_file(path, std::ios::binary);
    
    if(!opa_file.is_open() || (opa_file.eof() && opa_file.fail()))
        throw OpaFileError("Could not open file: " + path);

    std::vector<unsigned char> data = std::vector<unsigned char>((std::istreambuf_iterator<char>(opa_file)), std::istreambuf_iterator<char>());

    int width = *(int*)&data[0];
    int height = *(int*)&data[sizeof(int)];
    
    data.erase(data.begin(), data.begin() + sizeof(int) * 2);
    
    if(data.size() != width * height * 4)
        throw OpaFileError("OPA data size error file size is " + std::to_string(data.size()) + " but should be " + std::to_string(width * height * 4));
    
    gfx::Surface surface;
    surface.loadFromBuffer(data, width, height);
    return surface;
}
