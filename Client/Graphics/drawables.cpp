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
    return rectShape(
                     orientation % 3 == 1 ? (window_width >> 1) - (w >> 1) * scale + x : (orientation % 3 == 2 ? window_width - w * scale + x : x),
                     orientation / 3 == 1 ? (window_height >> 1) - (h >> 1) * scale + y : (orientation / 3 == 2 ? window_height - h * scale + y : y),
                     w * scale, h * scale);
}

gfx::rectShape gfx::button::getRect() const {
    return rectShape(
                     orientation % 3 == 1 ? (window_width >> 1) - ((w >> 1) + margin) * scale + x : (orientation % 3 == 2 ? window_width - (w + margin * 2) * scale + x : x),
                     orientation / 3 == 1 ? (window_height >> 1) - ((h >> 1) + margin) * scale + y : (orientation / 3 == 2 ? window_height - (h + margin * 2) * scale + y : y),
                     (w + margin * 2) * scale, (h + margin * 2) * scale);
}

bool gfx::button::isHovered() const {
    rectShape rect = getRect();
    return mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

gfx::texture::~texture() {
    freeTexture();
}

void gfx::texture::freeTexture() {
    if(tex && free_texture) {
        SDL_DestroyTexture((SDL_Texture*)tex);
        tex = nullptr;
    }
}

void gfx::textInput::setText(const std::string& text) {
    this->text.clear();
    for(char c : text) {
        if(textProcessing)
            c = textProcessing(c);
        if(c != 0)
            this->text.push_back(c);
    }
    setSurface(gfx::renderText(this->text.empty() ? " " : this->text, text_color));
}

gfx::rectShape gfx::textInput::getRect() const {
    return rectShape(
                     orientation % 3 == 1 ? (window_width >> 1) - ((w >> 1) + margin) * scale + x : (orientation % 3 == 2 ? window_width - (w + margin * 2) * scale + x : x),
                     orientation / 3 == 1 ? (window_height >> 1) - ((h >> 1) + margin) * scale + y : (orientation / 3 == 2 ? window_height - (h + margin * 2) * scale + y : y),
                     (width + margin * 2) * scale, (h + margin * 2) * scale);
}
