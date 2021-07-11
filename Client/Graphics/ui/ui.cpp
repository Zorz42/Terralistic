#include "ui.hpp"

//Image
void Image::setTexture(void* texture_) {
    freeTexture();
    texture = texture_;
}

Image::~Image() {
    freeTexture();
}

void Image::freeTexture() {
    if (texture && free_texture) {
        SDL_DestroyTexture((SDL_Texture*)texture);
        texture = nullptr;
    }
}

unsigned short Image::getTextureWidth() const {
    int width = 0;
    SDL_QueryTexture((SDL_Texture*)texture, nullptr, nullptr, &width, nullptr);
    return width;
}

unsigned short Image::getTextureHeight() const {
    int height = 0;
    SDL_QueryTexture((SDL_Texture*)texture, nullptr, nullptr, nullptr, &height);
    return height;
}

void Image::clear() {
    SDL_Texture* prev_target = SDL_GetRenderTarget(gfx::renderer);
    setRenderTarget(*this);
    SDL_SetRenderDrawColor(gfx::renderer, 0, 0, 0, 0);
    SDL_RenderClear(gfx::renderer);
    SDL_SetRenderTarget(gfx::renderer, prev_target);
}

void Image::setAlpha(unsigned char alpha) {
    SDL_SetTextureAlphaMod((SDL_Texture*)texture, alpha);
}

//Sprite
Sprite::Sprite: _centeredObject(0, 0) {};

//Buttom
unsigned short Button::getWidth() const {
    return (getTextureWidth() + (margin << 1)) * scale;
}

unsigned short Button::getHeight() const {
    return (getTextureHeight() + (margin << 1)) * scale;
}

bool Button::isHovered() const {
    if (disabled)
        return false;
    rectShape rect = getTranslatedRect();
    return mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

//TextInput
void TextInput::setText(const std::string& text_) {
    text = text_;
    setTexture(gfx::renderText((std::string)"|g" + text, text_color));
}

unsigned short TextInput::getWidth() const {
    return (width + 2 * margin) * scale;
}

TextInput::TextInput() {
    margin = 3;
    gfx::image temp;
    temp.setTexture(gfx::renderText("|g", { 0, 0, 0 }));
    cut_length = temp.getTextureWidth() - 1;
}
