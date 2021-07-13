//
//  drawables.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/03/2021.
//
/*
#include "graphics-internal.hpp"

// _centeredObject

gfx::rectShape gfx::_centeredObject::getTranslatedRect() const {
    return rectShape(getTranslatedX(), getTranslatedY(), getWidth(), getHeight());
}

short gfx::_centeredObject::getTranslatedX() const {
    return orientation % 3 == 1 ? (window_width >> 1) - (getWidth() >> 1) + x : (orientation % 3 == 2 ? window_width - getWidth() + x : x);
}

short gfx::_centeredObject::getTranslatedY() const {
    return orientation / 3 == 1 ? (window_height >> 1) - (getHeight() >> 1) + y : (orientation / 3 == 2 ? window_height - getHeight() + y : y);
}

// image

void gfx::image::setTexture(void* texture_) {
    freeTexture();
    texture = texture_;
}

gfx::image::~image() {
    freeTexture();
}

void gfx::image::freeTexture() {
    if (texture && free_texture) {
        SDL_DestroyTexture((SDL_Texture*)texture);
        texture = nullptr;
    }
}

unsigned short gfx::image::getTextureWidth() const {
    int width = 0;
    SDL_QueryTexture((SDL_Texture*)texture, nullptr, nullptr, &width, nullptr);
    return width;
}

unsigned short gfx::image::getTextureHeight() const {
    int height = 0;
    SDL_QueryTexture((SDL_Texture*)texture, nullptr, nullptr, nullptr, &height);
    return height;
}

void gfx::image::clear() {
    SDL_Texture* prev_target = SDL_GetRenderTarget(gfx::renderer);
    setRenderTarget(*this);
    SDL_SetRenderDrawColor(gfx::renderer, 0, 0, 0, 0);
    SDL_RenderClear(gfx::renderer);
    SDL_SetRenderTarget(gfx::renderer, prev_target);
}

void gfx::image::setAlpha(unsigned char alpha) {
    SDL_SetTextureAlphaMod((SDL_Texture*)texture, alpha);
}

// button

unsigned short gfx::button::getWidth() const {
    return (getTextureWidth() + (margin << 1)) * scale;
}

unsigned short gfx::button::getHeight() const {
    return (getTextureHeight() + (margin << 1)) * scale;
}

bool gfx::button::isHovered() const {
    if (disabled)
        return false;
    rectShape rect = getTranslatedRect();
    return mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

// textInput

void gfx::textInput::setText(const std::string& text_) {
    text = text_;
    setTexture(gfx::renderText((std::string)"|g" + text, text_color));
}

unsigned short gfx::textInput::getWidth() const {
    return (width + 2 * margin) * scale;
}

gfx::textInput::textInput() {
    margin = 3;
    gfx::image temp;
    temp.setTexture(gfx::renderText("|g", { 0, 0, 0 }));
    cut_length = temp.getTextureWidth() - 1;
}*/