//
//  render.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/03/2021.
//

#include "graphics-internal.hpp"

void gfx::render(rectShape x, color c) {
    SDL_SetRenderDrawColor(renderer, c.r, c.g, c.b, c.a);
    SDL_Rect sdl_rect = {x.x, x.y, x.w, x.h};
    SDL_RenderFillRect(renderer, &sdl_rect);
}

void gfx::render(rect x) {
    SDL_SetRenderDrawColor(renderer, x.c.r, x.c.g, x.c.b, x.c.a);
    SDL_Rect sdl_rect = {x.x, x.y, x.w, x.h};
    SDL_RenderFillRect(renderer, &sdl_rect);
}

void gfx::render(const gfx::texture& tex, short x, short y, unsigned short w, unsigned short h) {
    SDL_Rect rect = {x, y, w, h};
    SDL_RenderCopy(renderer, (SDL_Texture*)tex.tex, nullptr, &rect);
}

void gfx::render(const gfx::image& img, short x, short y) {
    render(img.tex, x, y, img.w * img.scale, img.h * img.scale);
}

void gfx::render(const gfx::sprite& spr) {
    gfx::rectShape rect = spr.getRect();
    render(spr.tex, rect.x, rect.y, rect.w, rect.h);
}
