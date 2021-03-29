//
//  render.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/03/2021.
//

#include "graphics-internal.hpp"

void gfx::render(rectShape x, color c, bool fill) {
    SDL_SetRenderDrawColor(renderer, c.r, c.g, c.b, c.a);
    SDL_Rect sdl_rect = {x.x, x.y, x.w, x.h};
    if(fill)
        SDL_RenderFillRect(renderer, &sdl_rect);
    else
        SDL_RenderDrawRect(renderer, &sdl_rect);
}

void gfx::render(rect x, bool fill) {
    SDL_SetRenderDrawColor(renderer, x.c.r, x.c.g, x.c.b, x.c.a);
    rectShape gfx_rect = x.getTranslatedRect();
    SDL_Rect sdl_rect = {gfx_rect.x, gfx_rect.y, gfx_rect.w, gfx_rect.h};
    if(fill)
        SDL_RenderFillRect(renderer, &sdl_rect);
    else
        SDL_RenderDrawRect(renderer, &sdl_rect);
}

void gfx::render(const image& tex, rectShape rect) {
    SDL_Rect sdl_rect = {rect.x, rect.y, rect.w, rect.h};
    SDL_RenderCopyEx(renderer, (SDL_Texture*)tex.getTexture(), nullptr, &sdl_rect, 0, nullptr, tex.flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void gfx::render(const image& tex, short x, short y) {
    SDL_Rect rect = {x, y, tex.getTextureWidth() * tex.scale, tex.getTextureHeight() * tex.scale};
    SDL_RenderCopyEx(renderer, (SDL_Texture*)tex.getTexture(), nullptr, &rect, 0, nullptr, tex.flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void gfx::render(const image& tex, short x, short y, rectShape src_rect) {
    SDL_Rect dest_rect_sdl = {x, y, src_rect.w * tex.scale, src_rect.h * tex.scale}, src_rect_sdl = {src_rect.x, src_rect.y, src_rect.w, src_rect.h};
    SDL_RenderCopyEx(renderer, (SDL_Texture*)tex.getTexture(), &src_rect_sdl, &dest_rect_sdl, 0, nullptr, tex.flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void gfx::render(const sprite& spr) {
    render(spr, spr.getTranslatedX(), spr.getTranslatedY());
}

void gfx::render(const button& b) {
    rectShape rect = b.getTranslatedRect();
    render(rect, b.isHovered() ? b.hover_color : b.def_color);
    render(b, rect.x + b.margin * b.scale, rect.y + b.margin * b.scale);
}

void gfx::render(const textInput& t) {
    rectShape rect = t.getTranslatedRect();
    render(rect, t.border_color);
    render(gfx::rectShape(rect.x + t.scale, rect.y + t.scale, rect.w - t.scale * 2, rect.h - t.scale * 2), t.isHovered() ? t.hover_color : t.def_color);
    rect.x += t.margin * t.scale;
    rect.y += t.margin * t.scale;
    rect.w = t.getTextureWidth() * t.scale;
    rect.h -= t.margin * 2 * t.scale;
    render(t, rect.x, rect.y, rectShape(rect.w > t.width * t.scale ? rect.w / t.scale - t.width : 0, 0, rect.w > t.width * t.scale ? t.width : rect.w / t.scale, rect.h / t.scale));
    if(t.active)
        render(gfx::rect(rect.x + (t.getText().empty() ? 0 : rect.w > t.width * t.scale ? t.width * t.scale : rect.w), rect.y, t.scale, rect.h, t.text_color));
}

SDL_Texture* createTextureFromSurface(SDL_Surface* surface) {
    SDL_Texture* result = SDL_CreateTextureFromSurface(gfx::renderer, surface);
    SDL_FreeSurface(surface);
    return result;
}

void* gfx::loadImageFile(const std::string& path) {
    // load picture and return texture
    SDL_Surface *loaded_surface = IMG_Load((resource_path + path).c_str());
    SDL_assert(loaded_surface);

    // green screen -> change rgb(0, 255, 0) to transparent
    SDL_SetColorKey(loaded_surface, SDL_TRUE, SDL_MapRGB(loaded_surface->format, 0, 255, 0));
    
    return (void*)createTextureFromSurface(loaded_surface);
}

#include <iostream>

void* gfx::renderText(const std::string& text, color text_color) {
    // render text to texture
    SDL_assert(font);
    SDL_Surface *rendered_surface = TTF_RenderText_Solid(font, text.c_str(), {text_color.r, text_color.g, text_color.b, text_color.a});
    SDL_assert(rendered_surface);

    return (void*)createTextureFromSurface(rendered_surface);
}

void* gfx::createBlankTexture(unsigned short w, unsigned short h) {
    SDL_Texture* result = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGBA8888, SDL_TEXTUREACCESS_TARGET, w, h);
    SDL_assert(result);
    return result;
}
