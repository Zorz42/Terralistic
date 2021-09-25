#include "graphics-internal.hpp"

gfx::Sprite::Sprite() : _CenteredObject(0, 0) {}

void gfx::Sprite::render() const {
    Texture::render(scale, getTranslatedX(), getTranslatedY(), flipped);
}
