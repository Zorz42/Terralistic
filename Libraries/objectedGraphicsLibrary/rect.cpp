//
//  rect.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"

ogl::rect::rect(ogl::objectType type) {
    setOrientation(type);
}

void ogl::rect::setColor(Uint8 r_, Uint8 g_, Uint8 b_) {
    r = r_;
    g = g_;
    b = b_;
}

void ogl::rect::render() {
    swl::setDrawColor(r, g, b);
    SDL_Rect render_rect = getRect();
    swl::render(render_rect, fill);
}

bool ogl::rect::touchesPoint(short x, short y) {
    SDL_Rect temp_rect = getRect();
    return x > temp_rect.x && y > temp_rect.y && x < temp_rect.x + temp_rect.w && y < temp_rect.y + temp_rect.h;
}
