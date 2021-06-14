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

void gfx::render(const rect& x, bool fill) {
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
    SDL_Rect rect = {x, y, int(tex.getTextureWidth() * tex.scale), int(tex.getTextureHeight() * tex.scale)};
    SDL_RenderCopyEx(renderer, (SDL_Texture*)tex.getTexture(), nullptr, &rect, 0, nullptr, tex.flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void gfx::render(const image& tex, short x, short y, rectShape src_rect) {
    SDL_Rect dest_rect_sdl = {x, y, int(src_rect.w * tex.scale), int(src_rect.h * tex.scale)}, src_rect_sdl = {src_rect.x, src_rect.y, src_rect.w, src_rect.h};
    SDL_RenderCopyEx(renderer, (SDL_Texture*)tex.getTexture(), &src_rect_sdl, &dest_rect_sdl, 0, nullptr, tex.flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void gfx::render(const sprite& spr) {
    render(spr, spr.getTranslatedX(), spr.getTranslatedY());
}

void gfx::render(button& b) {
    rectShape rect = b.getTranslatedRect();
    int hover_progress_target = b.isHovered() ? 255 : 0;
    b.hover_progress += (hover_progress_target - (int)b.hover_progress) / 2;
    color button_color{
        (unsigned char)((int)b.hover_color.r * (int)b.hover_progress / 255 + (int)b.def_color.r * (int)(255 - b.hover_progress) / 255),
        (unsigned char)((int)b.hover_color.g * (int)b.hover_progress / 255 + (int)b.def_color.g * (int)(255 - b.hover_progress) / 255),
        (unsigned char)((int)b.hover_color.b * (int)b.hover_progress / 255 + (int)b.def_color.b * (int)(255 - b.hover_progress) / 255),
    };
    render(rect, button_color);
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

void* gfx::loadImageFile(const std::string& path) {
    // load picture and return texture
    SDL_Texture *loaded_texture = IMG_LoadTexture(gfx::renderer, (resource_path + path).c_str());;
    SDL_assert(loaded_texture);
    return (void*)loaded_texture;
}

void* gfx::renderText(const std::string& text, color text_color) {
    // render text to texture
    SDL_assert(font);
    SDL_Surface *rendered_surface = TTF_RenderText_Solid(font, text.c_str(), {text_color.r, text_color.g, text_color.b, text_color.a});
    SDL_assert(rendered_surface);
    
    SDL_Texture* result = SDL_CreateTextureFromSurface(gfx::renderer, rendered_surface);
    SDL_FreeSurface(rendered_surface);
    
    return (void*)result;
}

void* gfx::createBlankTexture(unsigned short w, unsigned short h) {
    SDL_Texture* result = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGBA8888, SDL_TEXTUREACCESS_TARGET, w, h);
    SDL_assert(result);
    SDL_SetTextureBlendMode(result, SDL_BLENDMODE_BLEND);
    return result;
}
