//
//  rect.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#include "objectedGraphicsLibrary.hpp"

ogl::rect::rect(SDL_Rect rect) {
    setRect(rect);
}

void ogl::rect::setRect(SDL_Rect rect) {
    setPos(rect.x, rect.y);
    setWidth(rect.w);
    setHeight(rect.h);
}
