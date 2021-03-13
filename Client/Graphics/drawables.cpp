//
//  drawables.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/03/2021.
//

#include "graphics-internal.hpp"

SDL_Texture* transformSurfaceToTexture(SDL_Surface* surf, unsigned short* w, unsigned short* h) {
    SDL_Texture* result = SDL_CreateTextureFromSurface(gfx::renderer, surf);
    SDL_assert(result);
    
    // get dimensions of surface, because you cant get it in texture
    if(w)
        *w = (unsigned short)surf->w;
    if(h)
        *h = (unsigned short)surf->h;
    
    SDL_FreeSurface(surf);
    
    return result;
}

void gfx::texture::loadFromFile(const std::string& path, unsigned short* w, unsigned short* h) {
    // load picture and return texture
    SDL_Surface *loaded_surface = IMG_Load((resource_path + path).c_str());
    SDL_assert(loaded_surface);

    // green screen -> remove rgb(0, 255, 0) which is green to transparent
    SDL_SetColorKey(loaded_surface, SDL_TRUE, SDL_MapRGB(loaded_surface->format, 0, 255, 0));
    
    tex = (void*)transformSurfaceToTexture(loaded_surface, w, h);
}

void gfx::texture::loadFromText(const std::string& text, color text_color, unsigned short* w, unsigned short* h) {
    // render text to texture
    SDL_Surface *rendered_surface = TTF_RenderText_Solid(font, text.c_str(), {text_color.r, text_color.g, text_color.b, text_color.a});
    SDL_assert(rendered_surface);

    tex = (void*)transformSurfaceToTexture(rendered_surface, w, h);
}

void gfx::image::loadFromFile(const std::string& path) {
    tex.loadFromFile(path, &w, &h);
}

void gfx::image::loadFromText(const std::string& text, color text_color) {
    tex.loadFromText(text, text_color, &w, &h);
}

gfx::rectShape gfx::_centeredObject::getRect() const {
    return rectShape(
                     orientation % 3 == 1 ? window_width / 2 - w / 2 + x : (orientation % 3 == 2 ? window_width - w + x : x),
                     orientation / 3 == 1 ? window_height / 2 - h / 2 + y : (orientation / 3 == 2 ? window_height - h + y : y),
                     w * scale,
                     h * scale
                     );
}

void gfx::sprite::loadFromFile(const std::string &path) {
    tex.loadFromFile(path, &w, &h);
}

void gfx::sprite::loadFromText(const std::string &text, color text_color) {
    tex.loadFromText(text, text_color, &w, &h);
}
