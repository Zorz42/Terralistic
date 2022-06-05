#include <fstream>
#include <vector>
#include "readOpa.hpp"
#include "exception.hpp"

void loadOpa(gfx::Texture& texture, const std::string& path) {
    std::ifstream opa_file(path);
    if(!opa_file.is_open() || opa_file.eof() && opa_file.fail())
        throw Exception("Could not open file: " + path);

    opa_file.seekg(0, std::ios_base::end);
    std::streampos file_size = opa_file.tellg();
    std::vector<unsigned char> data(file_size);

    opa_file.seekg(0, std::ios_base::beg);
    opa_file.read((char*)&data[0], file_size);

    int width = *(int*)&data[0];
    int height = *(int*)&data[4];
    data.erase(data.begin(), data.begin() + 8);
    if(data.size() != width * height * 4)
        throw Exception("OPA data size error file size is " + std::to_string(data.size()) + " but should be " + std::to_string(width * height * 4));
    
    texture.loadFromData(&data[0], width, height);
}
