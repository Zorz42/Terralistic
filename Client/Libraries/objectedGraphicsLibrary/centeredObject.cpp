//
//  centeredObject.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"

// centered object can have 9 positions, top left, top, top right... they are relative to window size, x and y are offsets to those positions

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

swl::rect ogl_private::centeredObject::getRect() {
    return {
        getX(),
        getY(),
        getWidth(),
        getHeight(),
    };
}
