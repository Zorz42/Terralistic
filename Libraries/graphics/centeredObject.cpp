#include "graphics-internal.hpp"

gfx::_CenteredObject::_CenteredObject(short x, short y, ObjectType orientation) : orientation(orientation), x(x), y(y) {}

short gfx::_CenteredObject::getTranslatedX() const {
    if(orientation % 3 == 0)
        return x;
    else if(orientation % 3 == 1)
        return x + getWindowWidth() / 2 - getWidth() / 2;
    else
         return x + getWindowWidth() - getWidth();
}

short gfx::_CenteredObject::getTranslatedY() const {
    if(orientation / 3 == 0)
        return y;
    else if(orientation / 3 == 1)
        return y + getWindowHeight() / 2 - getHeight() / 2;
    else
        return y + getWindowHeight() - getHeight();
}

gfx::RectShape gfx::_CenteredObject::getTranslatedRect() const {
    return {getTranslatedX(), getTranslatedY(), getWidth(), getHeight()};
}
