#include "graphics-internal.hpp"

gfx::PixelGrid::PixelGrid(unsigned short width, unsigned short height) : width(width), height(height) {
    array = new unsigned char[width * height * 4];
}

unsigned short gfx::PixelGrid::getWidth() const {
    return width;
}

unsigned short gfx::PixelGrid::getHeight() const {
    return height;
}

void gfx::PixelGrid::setPixel(unsigned short x, unsigned short y, Color color) {
    unsigned short index = (y * width + x) * 4;
    array[index] = color.r;
    array[index + 1] = color.g;
    array[index + 2] = color.b;
    array[index + 3] = color.a;
}

unsigned char* gfx::PixelGrid::getArray() const {
    return array;
}
