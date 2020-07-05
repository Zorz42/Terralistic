//
//  rect.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "objectedGraphicsLibrary.hpp"

ogl::rect::rect(ogl::objectType type) {
    centered_x = type == ogl::centered;
    centered_y = type == ogl::centered;
}

void ogl::rect::setColor(Uint8 r_, Uint8 g_, Uint8 b_) {
    r = r_;
    g = g_;
    b = b_;
}

void ogl::rect::render() {
    SDL_Rect render_rect = {
        getX(width),
        getY(height),
        width,
        height,
    };
    swl::setDrawColor(r, g, b);
    swl::render(render_rect, fill);
}
