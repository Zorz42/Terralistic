#include "graphics-internal.hpp"
#include "exception.hpp"

gfx::PixelGrid::PixelGrid(int width, int height) : width(width), height(height) {
    if(width <= 0 || height <= 0)
        throw Exception("PixelGrid width and height must be positive.");
    
    array = new unsigned char[(int)width * height * 4];
}

int gfx::PixelGrid::getWidth() const {
    return width;
}

int gfx::PixelGrid::getHeight() const {
    return height;
}

void gfx::PixelGrid::setPixel(int x, int y, Color color) {
    if(x < 0 || x >= width || y < 0 || y >= height)
        throw Exception("Pixel position must be in the range of width and height.");
    
    int index = ((int)y * width + x) * 4;
    *(int*)&array[index] = (int)color.r + ((int)color.g << 8) + ((int)color.b << 16) + ((int)color.a << 24);
}

unsigned char* gfx::PixelGrid::getArray() const {
    return array;
}

gfx::PixelGrid::~PixelGrid() {
    delete[] array;
}
