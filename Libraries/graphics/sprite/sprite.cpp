#include "sprite.hpp"

gfx::Sprite::Sprite() : _OrientedObject(0, 0) {}

void gfx::Sprite::render() const {
    Texture::render(scale, getTranslatedRect().x, getTranslatedRect().y, {0, 0, getTextureWidth(), getTextureHeight()}, flipped, color);
}

void gfx::Sprite::setColor(Color color_) {
    color = color_;
}

void gfx::Sprite::loadFromSurface(const Surface& surface) {
    Texture::loadFromSurface(surface);
    setScale(scale);
}

void gfx::Sprite::setScale(float scale_) {
    scale = scale_;
    w = getTextureWidth() * scale;
    h = getTextureHeight() * scale;
}

float gfx::Sprite::getScale() const {
    return scale;
}

int gfx::Sprite::getWidth() const {
    return w;
}

int gfx::Sprite::getHeight() const {
    return h;
}
