#include <fstream>
#include <vector>
#include "readOpa.hpp"
#include "exception.hpp"

void loadOpa(gfx::Texture& texture, const std::string& path) {
    std::ifstream opa_file(path);
    if(!opa_file.is_open())
        throw Exception("Could not open file: " + path);
    std::vector<unsigned char> data = std::vector<unsigned char>((std::istreambuf_iterator<char>(opa_file)), std::istreambuf_iterator<char>());
    int width = *(int*)&data[0];
    int height = *(int*)&data[4];
    data.erase(data.begin(), data.begin() + 8);
    texture.loadFromData(&data[0], width, height);
}
