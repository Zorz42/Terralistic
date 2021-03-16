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
    rectShape gfx_rect = x.getRect();
    SDL_Rect sdl_rect = {gfx_rect.x, gfx_rect.y, gfx_rect.w, gfx_rect.h};
    SDL_RenderFillRect(renderer, &sdl_rect);
}

void gfx::render(const texture& tex, rectShape dest_rect) {
    SDL_Rect rect = {dest_rect.x, dest_rect.y, dest_rect.w, dest_rect.h};
    SDL_RenderCopy(renderer, (SDL_Texture*)tex.tex, nullptr, &rect);
}

void gfx::render(const texture& tex, rectShape dest_rect, rectShape src_rect) {
    SDL_Rect dest_rect_sdl = {dest_rect.x, dest_rect.y, dest_rect.w, dest_rect.h}, src_rect_sdl = {src_rect.x, src_rect.y, src_rect.w, src_rect.h};
    SDL_RenderCopy(renderer, (SDL_Texture*)tex.tex, &src_rect_sdl, &dest_rect_sdl);
}

void gfx::render(const image& img, short x, short y) {
    render(img, rectShape(x, y, img.w * img.scale, img.h * img.scale));
}

void gfx::render(const sprite& spr) {
    render(spr, spr.getRect());
}

void gfx::render(const button& b) {
    rectShape rect = b.getRect();
    render(rect, b.isHovered() ? b.hover_color : b.def_color);
    rect.x += b.margin;
    rect.y += b.margin;
    rect.w -= b.margin * 2;
    rect.h -= b.margin * 2;
    render(b, rect);
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
