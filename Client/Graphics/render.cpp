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

void gfx::render(const texture& tex, short x, short y, unsigned short w, unsigned short h) {
    SDL_Rect rect = {x, y, w, h};
    SDL_RenderCopy(renderer, (SDL_Texture*)tex.tex, nullptr, &rect);
}

void gfx::render(const image& img, short x, short y) {
    render(img, x, y, img.w * img.scale, img.h * img.scale);
}

void gfx::render(const sprite& spr) {
    rectShape rect = spr.getRect();
    render(spr, rect.x, rect.y, rect.w, rect.h);
}

void gfx::render(const button& b) {
    rectShape rect = b.getRect();
    render(rect, b.isHovered() ? b.hover_color : b.def_color);
    render(b, rect.x + b.margin, rect.y + b.margin, rect.w - b.margin * 2, rect.h - b.margin * 2);
}

void* gfx::loadImageFile(const std::string& path) {
    // load picture and return texture
    SDL_Surface *loaded_surface = IMG_Load((resource_path + path).c_str());
    SDL_assert(loaded_surface);

    // green screen -> remove rgb(0, 255, 0) which is green to transparent
    SDL_SetColorKey(loaded_surface, SDL_TRUE, SDL_MapRGB(loaded_surface->format, 0, 255, 0));
    
    return loaded_surface;
}

void* gfx::renderText(const std::string& text, color text_color) {
    // render text to texture
    SDL_assert(font);
    SDL_Surface *rendered_surface = TTF_RenderText_Solid(font, text.c_str(), {text_color.r, text_color.g, text_color.b, text_color.a});
    SDL_assert(rendered_surface);

    return rendered_surface;
}
