#include "graphics-internal.hpp"

gfx::_CenteredObject::_CenteredObject(short x, short y, Orientation orientation) : orientation(orientation), x(x), y(y) {}

short gfx::_CenteredObject::getTranslatedX() const {
    return x + getWindowWidth() * orientation.x - getWidth() * orientation.x;
}

short gfx::_CenteredObject::getTranslatedY() const {
    return y + getWindowHeight() * orientation.y - getHeight() * orientation.y;
}

gfx::RectShape gfx::_CenteredObject::getTranslatedRect() const {
    return {getTranslatedX(), getTranslatedY(), getWidth(), getHeight()};
}
