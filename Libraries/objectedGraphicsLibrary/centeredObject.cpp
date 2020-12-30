//
//  centeredObject.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"

short ogl_private::centeredObject::getX() {
    if(orientation_x == 1)
        return short(swl::window_width / 2 - getWidth() / 2 + x);
    else if(orientation_x == 2)
        return short(swl::window_width - getWidth() + x);
    else
        return x;
}

short ogl_private::centeredObject::getY() {
    if(orientation_y == 1)
        return short(swl::window_height / 2 - getHeight() / 2 + y);
    else if(orientation_y == 2)
        return short(swl::window_height - getHeight() + y);
    else
        return y;
}

void ogl_private::centeredObject::setOrientation(Uint8 objectType) {
    orientation_x = Uint8(objectType % 3);
    orientation_y = Uint8(objectType / 3);
}

[[maybe_unused]] void ogl_private::centeredObject::setX(short x_) {
    x = x_;
}

void ogl_private::centeredObject::setY(short y_) {
    y = y_;
}

SDL_Rect ogl_private::centeredObject::getRect() {
    return {
        getX(),
        getY(),
        getWidth(),
        getHeight(),
    };
}
