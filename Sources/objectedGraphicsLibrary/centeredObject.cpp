//
//  centeredObject.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"

short __ogl_private::centeredObject::getX(short width) {
    if(centered)
        return (swl::window_width + center_rect_left_offset + center_rect_right_offset) / 2 + center_rect_left_offset - width / 2 + x_offset;
    else
        return x_offset;
}

short __ogl_private::centeredObject::getY(short height) {
    if(centered)
        return (swl::window_height + center_rect_up_offset + center_rect_down_offset) / 2 + center_rect_up_offset - height / 2 + y_offset;
    else
        return y_offset;
}


void __ogl_private::centeredObject::setOffset(short x, short y) {
    x_offset = x;
    y_offset = y;
    centered = true;
}

void __ogl_private::centeredObject::setCenterRectBoundsOffset(short up, short down, short left, short right) {
    center_rect_up_offset = up;
    center_rect_down_offset = down;
    center_rect_left_offset = left;
    center_rect_right_offset = right;
    centered = true;
}

void __ogl_private::centeredObject::setPos(short x, short y) {
    x_offset = x;
    y_offset = y;
    centered = false;
}
