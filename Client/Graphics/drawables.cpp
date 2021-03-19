//
//  drawables.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 11/03/2021.
//

#include "graphics-internal.hpp"

void gfx::texture::setTexture(void *texture) {
    freeTexture();
    tex = texture;
}

gfx::rectShape gfx::_centeredObject::getTranslatedRect() const {
    return rectShape(getTranslatedX(), getTranslatedY(), getWidth(), getHeight());
}

short gfx::_centeredObject::getTranslatedX() const {
    return orientation % 3 == 1 ? (window_width >> 1) - (getWidth() >> 1) + x : (orientation % 3 == 2 ? window_width - getWidth() + x : x);
}

short gfx::_centeredObject::getTranslatedY() const {
    return orientation / 3 == 1 ? (window_height >> 1) - (getHeight() >> 1) + y : (orientation / 3 == 2 ? window_height - getHeight() + y : y);
}

unsigned short gfx::button::getWidth() const {
    return (getTextureWidth() + (margin << 1)) * scale;
}

unsigned short gfx::button::getHeight() const {
    return (getTextureHeight() + (margin << 1)) * scale;
}

bool gfx::button::isHovered() const {
    rectShape rect = getTranslatedRect();
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
    setTexture(gfx::renderText(this->text.empty() ? " " : this->text, text_color));
}

unsigned short gfx::texture::getTextureWidth() const {
    int width = 0;
    SDL_QueryTexture((SDL_Texture*)tex, nullptr, nullptr, &width, nullptr);
    return width;
}

unsigned short gfx::texture::getTextureHeight() const {
    int height = 0;
    SDL_QueryTexture((SDL_Texture*)tex, nullptr, nullptr, nullptr, &height);
    return height;
}

unsigned short gfx::textInput::getWidth() const {
    return (width + 2 * margin) * scale;
}
