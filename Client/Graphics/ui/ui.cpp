#include "graphics-internal.hpp"

void gfx::Image::setTexture(void* texture_) {
    freeTexture();
    texture = texture_;
}

gfx::Image::~Image() {
    freeTexture();
}

void gfx::Image::freeTexture() {
    if (texture && free_texture) {
        SDL_DestroyTexture((SDL_Texture*)texture);
        texture = nullptr;
    }
}

unsigned short gfx::Image::getTextureWidth() const {
    int width = 0;
    SDL_QueryTexture((SDL_Texture*)texture, nullptr, nullptr, &width, nullptr);
    return width;
}

unsigned short gfx::Image::getTextureHeight() const {
    int height = 0;
    SDL_QueryTexture((SDL_Texture*)texture, nullptr, nullptr, nullptr, &height);
    return height;
}

void gfx::Image::clear() {
    SDL_Texture* prev_target = SDL_GetRenderTarget(gfx::renderer);
    setRenderTarget(*this);
    SDL_SetRenderDrawColor(gfx::renderer, 0, 0, 0, 0);
    SDL_RenderClear(gfx::renderer);
    SDL_SetRenderTarget(gfx::renderer, prev_target);
}
void gfx::Image::render(float scale, short x, short y, RectShape src_rect) const {
    SDL_Rect dest_rect_sdl = { x, y, int(src_rect.w * scale), int(src_rect.h * scale) }, src_rect_sdl = { src_rect.x, src_rect.y, src_rect.w, src_rect.h };
    SDL_RenderCopyEx(gfx::renderer, (SDL_Texture*)this->getTexture(), &src_rect_sdl, &dest_rect_sdl, 0, nullptr, this->flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void gfx::Image::setAlpha(unsigned char alpha) {
    SDL_SetTextureAlphaMod((SDL_Texture*)texture, alpha);
}

void gfx::Image::render(RectShape rect) const {
    SDL_Rect sdl_rect = { rect.x, rect.y, rect.w, rect.h };
    SDL_RenderCopyEx(gfx::renderer, (SDL_Texture*)this->getTexture(), nullptr, &sdl_rect, 0, nullptr, this->flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

void gfx::Image::render(float scale, short x, short y) const {
    SDL_Rect rect = { x, y, int(this->getTextureWidth() * scale), int(this->getTextureHeight() * scale) };
    SDL_RenderCopyEx(gfx::renderer, (SDL_Texture*)this->getTexture(), nullptr, &rect, 0, nullptr, this->flipped ? SDL_FLIP_HORIZONTAL : SDL_FLIP_NONE);
}

gfx::Sprite::Sprite() : _CenteredObject(0, 0) {};

void gfx::Sprite::render() const {
    ((Image)(*this)).render(scale, getTranslatedX(), getTranslatedY());
}

unsigned short gfx::Button::getWidth() const {
    return (getTextureWidth() + (margin << 1)) * scale;
}

unsigned short gfx::Button::getHeight() const {
    return (getTextureHeight() + (margin << 1)) * scale;
}

bool gfx::Button::isHovered() const {
    if (disabled)
        return false;
    RectShape rect = getTranslatedRect();
    return mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

void gfx::Button::render() {
    RectShape rect = this->getTranslatedRect();
    int hover_progress_target = this->isHovered() ? 255 : 0;
    this->hover_progress += (hover_progress_target - (int)this->hover_progress) / 2;
    Color button_color{
        (unsigned char)((int)this->hover_color.r * (int)this->hover_progress / 255 + (int)this->def_color.r * (int)(255 - this->hover_progress) / 255),
        (unsigned char)((int)this->hover_color.g * (int)this->hover_progress / 255 + (int)this->def_color.g * (int)(255 - this->hover_progress) / 255),
        (unsigned char)((int)this->hover_color.b * (int)this->hover_progress / 255 + (int)this->def_color.b * (int)(255 - this->hover_progress) / 255),
    };
    rect.render(button_color);
    ((Image)(*this)).render(scale, rect.x + margin * scale, rect.y + margin * scale);
}

void gfx::TextInput::setText(const std::string& text_) {
    text = text_;
    setTexture(gfx::renderText((std::string)"|g" + text, text_color));
}

unsigned short gfx::TextInput::getWidth() const {
    return (width + 2 * margin) * scale;
}

gfx::TextInput::TextInput() {
    margin = 3;
    Image temp;
    temp.setTexture(gfx::renderText("|g", { 0, 0, 0 }));
    cut_length = temp.getTextureWidth() - 1;
}
void gfx::TextInput::render() const {
    RectShape rect = this->getTranslatedRect();
    rect.render(border_color);
    RectShape(rect.x + this->scale, rect.y + this->scale, rect.w - this->scale * 2, rect.h - this->scale * 2).render(isHovered() ? hover_color : def_color);
    rect.x += this->margin * this->scale;
    rect.y += this->margin * this->scale;
    rect.w = this->getTextureWidth() * this->scale;
    rect.h -= this->margin * 2 * this->scale;
    ((Image)(*this)).render(scale, rect.x, rect.y, RectShape(rect.w - this->cut_length > this->width * this->scale ? rect.w / this->scale - this->width : this->cut_length, 0, rect.w - this->cut_length > this->width * this->scale ? this->width : rect.w / this->scale - this->cut_length, rect.h / this->scale));
    //((Image)(*this)).render(scale, rect.x, rect.y, RectShape(rect.w - this->cut_length > this->width * this->scale ? rect.w / this->scale - this->width : this->cut_length, 0, rect.w - this->cut_length > this->width * this->scale ? this->width : rect.w / this->scale - this->cut_length, rect.h / this->scale));
    if (active)
        Rect(rect.x + (rect.w > width * scale ? width * scale : rect.w - cut_length * scale), rect.y, scale, rect.h, text_color).render();
        
}
