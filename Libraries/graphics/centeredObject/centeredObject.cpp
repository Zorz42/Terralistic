#include "glfwAbstraction.hpp"
#include "centeredObject.hpp"

gfx::_CenteredObject::_CenteredObject(int x, int y, Orientation orientation) : orientation(orientation), x(x), y(y) {}

int gfx::_CenteredObject::getTranslatedX() const {
    return x + getWindowWidth() * orientation.x - getWidth() * orientation.x;
}

int gfx::_CenteredObject::getTranslatedY() const {
    return y + getWindowHeight() * orientation.y - getHeight() * orientation.y;
}

gfx::RectShape gfx::_CenteredObject::getTranslatedRect() const {
    return {getTranslatedX(), getTranslatedY(), getWidth(), getHeight()};
}
