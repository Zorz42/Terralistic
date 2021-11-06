#include "graphics-internal.hpp"

gfx::PixelGrid::PixelGrid(unsigned short width, unsigned short height) : width(width), height(height) {
    array = new unsigned char[(int)width * height * 4];
}

unsigned short gfx::PixelGrid::getWidth() const {
    return width;
}

unsigned short gfx::PixelGrid::getHeight() const {
    return height;
}

void gfx::PixelGrid::setPixel(unsigned short x, unsigned short y, Color color) {
    int index = ((int)y * width + x) * 4;
    *(int*)&array[index] = (int)color.r + ((int)color.g << 8) + ((int)color.b << 16) + ((int)color.a << 24);
}

unsigned char* gfx::PixelGrid::getArray() const {
    return array;
}

gfx::PixelGrid::~PixelGrid() {
    delete[] array;
}
