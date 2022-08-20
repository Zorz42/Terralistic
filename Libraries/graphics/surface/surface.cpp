#include "surface.hpp"

void gfx::Surface::loadFromBuffer(const std::vector<unsigned char>& buffer, int w, int h) {
    width = w;
    height = h;
    
    if(width * height * 4 != buffer.size())
        throw BufferSizeError("Surface buffer size mismatch");
    
    data = buffer;
}

void gfx::Surface::createEmpty(int w, int h) {
    width = w;
    height = h;
    data = std::vector<unsigned char>(width * height * 4, 0);
}

int gfx::Surface::getIndex(int x, int y) const {
    if(x < 0 || x >= width || y < 0 || y >= height)
        throw PixelOutOfBoundsError("Pixel out of bounds");
    return ((height - y - 1) * width + x) * 4;
}

gfx::Color gfx::Surface::getPixel(int x, int y) const {
    Color result;
    int index = getIndex(x, y);
    result.r = data[index + 0];
    result.g = data[index + 1];
    result.b = data[index + 2];
    result.a = data[index + 3];
    return result;
}

void gfx::Surface::setPixel(int x, int y, gfx::Color color) {
    if(x < 0 || x >= width || y < 0 || y >= height)
        throw PixelOutOfBoundsError("Pixel out of bounds");
    
    int index = getIndex(x, y);
    data[index + 0] = color.r;
    data[index + 1] = color.g;
    data[index + 2] = color.b;
    data[index + 3] = color.a;
}

const std::vector<unsigned char>& gfx::Surface::getData() const {
    return data;
}

int gfx::Surface::getWidth() const {
    return width;
}

int gfx::Surface::getHeight() const {
    return height;
}

void gfx::Surface::draw(int x, int y, const Surface &surface, Color color) {
    for(int y_ = 0; y_ < surface.getHeight(); y_++)
        for(int x_ = 0; x_ < surface.getWidth(); x_++) {
            Color c = surface.getPixel(x_, y_);
            c.r *= (float)color.r / 255;
            c.g *= (float)color.g / 255;
            c.b *= (float)color.b / 255;
            c.a *= (float)color.a / 255;
            setPixel(x + x_, y + y_, c);
        }
}
