#include "graphics-internal.hpp"

gfx::RectShape gfx::_CenteredObject::getTranslatedRect() const {
    return {getTranslatedX(), getTranslatedY(), getWidth(), getHeight()};
}

short gfx::_CenteredObject::getTranslatedX(unsigned short width) const {
    if(orientation % 3 == 0)
        return x;
    else if(orientation % 3 == 1)
        return x + getWindowWidth() / 2 - width / 2;
    else
         return x + getWindowWidth() - width;
}

short gfx::_CenteredObject::getTranslatedY(unsigned short height) const {
    if(orientation / 3 == 0)
        return y;
    else if(orientation / 3 == 1)
        return y + getWindowHeight() / 2 - height / 2;
    else
        return y + getWindowHeight() - height;
}

short gfx::_CenteredObject::getTranslatedX() const {
    return getTranslatedX(getWidth());
}

short gfx::_CenteredObject::getTranslatedY() const {
    return getTranslatedY(getHeight());
}



gfx::_CenteredObject::_CenteredObject(short x, short y, ObjectType orientation) : orientation(orientation), x(x), y(y) {}
