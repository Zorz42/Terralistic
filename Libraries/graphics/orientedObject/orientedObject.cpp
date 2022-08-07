#include "glfwAbstraction.hpp"
#include "orientedObject.hpp"

gfx::_OrientedObject::_OrientedObject(int x, int y, int w, int h, Orientation orientation) : orientation(orientation), RectShape(x, y, w, h) {}

int gfx::_OrientedObject::getTranslatedX() const {
    return x + getWindowWidth() * orientation.x - w * orientation.x;
}

int gfx::_OrientedObject::getTranslatedY() const {
    return y + getWindowHeight() * orientation.y - h * orientation.y;
}

gfx::RectShape gfx::_OrientedObject::getTranslatedRect() const {
    return {getTranslatedX(), getTranslatedY(), w, h};
}
