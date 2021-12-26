#include "walls.hpp"
#include "compress.hpp"

Walls::Wall* Walls::getWall(int x, int y) {
    if(x < 0 || x >= getWidth() || y < 0 || y >= getHeight())
        throw Exception("Wall is accessed out of the bounds! (" + std::to_string(x) + ", " + std::to_string(y) + ")");
    return &walls[y * getWidth() + x];
}

void Walls::create() {
    walls = new Wall[getWidth() * getHeight()];
}

WallType* Walls::getWallType(int x, int y) {
    return getWallTypeById(getWall(x, y)->id);
}

void Walls::setWallTypeSilently(int x, int y, WallType* type) {
    getWall(x, y)->id = type->id;
}

int Walls::getWidth() const {
    return blocks->getWidth();
}

int Walls::getHeight() const {
    return blocks->getHeight();
}

std::vector<char> Walls::toSerial() {
    std::vector<char> serial;
    unsigned long iter = 0;
    serial.resize(serial.size() + getWidth() * getHeight());
    Wall* wall = walls;
    for(int i = 0; i < getWidth() * getHeight(); i++) {
        serial[iter++] = (char)wall->id;
        wall++;
    }
    return compress(serial);
}

void Walls::fromSerial(const std::vector<char>& serial) {
    std::vector<char> decompressed = decompress(serial);
    const char* iter = &decompressed[0];
    create();
    Wall* wall = walls;
    for(int i = 0; i < getWidth() * getHeight(); i++) {
        wall->id = *iter++;
        wall++;
    }
}

void Walls::registerNewWallType(WallType* wall_type) {
    wall_type->id = curr_id++;
    wall_types.push_back(wall_type);
}

WallType* Walls::getWallTypeById(int wall_id) {
    return wall_types[wall_id];
}

WallType* Walls::getWallTypeByName(const std::string& name) {
    for(int i = 0; i < wall_types.size(); i++)
        if(wall_types[i]->name == name)
            return wall_types[i];
    return nullptr;
}

int Walls::getNumWallTypes() {
    return (int)wall_types.size();
}
