//
//  drawables.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/03/2021.
//

#include "graphics-internal.hpp"

void gfx::texture::setSurface(void *surface) {
    tex = SDL_CreateTextureFromSurface(renderer, (SDL_Surface*)surface);
    SDL_FreeSurface((SDL_Surface*)surface);
}

void gfx::image::setSurface(void *surface) {
    tex = SDL_CreateTextureFromSurface(renderer, (SDL_Surface*)surface);
    w = ((SDL_Surface*)surface)->w;
    h = ((SDL_Surface*)surface)->h;
    SDL_FreeSurface((SDL_Surface*)surface);
}

void gfx::sprite::setSurface(void *surface) {
    tex = SDL_CreateTextureFromSurface(renderer, (SDL_Surface*)surface);
    w = ((SDL_Surface*)surface)->w;
    h = ((SDL_Surface*)surface)->h;
    SDL_FreeSurface((SDL_Surface*)surface);
}

gfx::rectShape gfx::_centeredObject::getRect() const {
    return rectShape(orientation % 3 == 1 ? (window_width >> 1) - (w >> 1) + x : (orientation % 3 == 2 ? window_width - w + x : x), orientation / 3 == 1 ? (window_height >> 1) - (h >> 1) + y : (orientation / 3 == 2 ? window_height - h + y : y), w * scale, h * scale);
}

void gfx::button::setText(const std::string &text, color text_color) {
    setSurface(renderText(text, text_color));
}

gfx::rectShape gfx::button::getRect() const {
    rectShape rect = this->_centeredObject::getRect();
    rect.w += margin * 2;
    rect.h += margin * 2;
    return rect;
}

bool gfx::button::isHovered() const {
    rectShape rect = getRect();
    return mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}
