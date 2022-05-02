#include "graphics-internal.hpp"

gfx::Sprite::Sprite() : _CenteredObject(0, 0) {}

void gfx::Sprite::render() const {
    Texture::render(scale, getTranslatedX(), getTranslatedY(), src_rect, flipped);
}

int gfx::Sprite::getWidth() const {
    return getTextureWidth() * scale;
}

int gfx::Sprite::getHeight() const {
    return getTextureHeight() * scale;
}

void gfx::Sprite::setColor(Color color_) {
    color = color_;
}

void gfx::Sprite::setSrcRect(RectShape src_rect_) {
    src_rect = src_rect_;
}

void gfx::Sprite::createBlankImage(int width, int height) {
    Texture::createBlankImage(width, height);
    setSrcRect({0, 0, getTextureWidth(), getTextureHeight()});
}

void gfx::Sprite::loadFromData(const unsigned char* data, int width, int height) {
    Texture::loadFromData(data, width, height);
    setSrcRect({0, 0, getTextureWidth(), getTextureHeight()});
}

void gfx::Sprite::loadFromText(const std::string &text, Color color) {
    Texture::loadFromText(text, color);
    setSrcRect({0, 0, getTextureWidth(), getTextureHeight()});
}
