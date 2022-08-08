#include "glfwAbstraction.hpp"
#include "orientedObject.hpp"

gfx::_OrientedObject::_OrientedObject(int x, int y, int w, int h, Orientation orientation) : orientation(orientation), x(x), y(y), w(w), h(h) {}

gfx::RectShape gfx::_OrientedObject::getTranslatedRect() const {
    return {int(x + getWindowWidth() * orientation.x - w * orientation.x), int(y + getWindowHeight() * orientation.y - h * orientation.y), w, h};
}
