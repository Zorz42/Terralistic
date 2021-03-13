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
    return rectShape(orientation % 3 == 1 ? window_width / 2 - w / 2 + x : (orientation % 3 == 2 ? window_width - w + x : x), orientation / 3 == 1 ? window_height / 2 - h / 2 + y : (orientation / 3 == 2 ? window_height - h + y : y), w * scale, h * scale);
}
