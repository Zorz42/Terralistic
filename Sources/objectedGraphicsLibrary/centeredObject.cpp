//
//  centeredObject.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"

short __ogl_private::centeredObject::getX(short width_) {
    if(centered_x)
        return swl::window_width / 2 - width_ / 2 + x;
    else
        return x;
}

short __ogl_private::centeredObject::getY(short height_) {
    if(centered_y)
        return swl::window_height / 2 - height_ / 2 + y;
    else
        return y;
}
