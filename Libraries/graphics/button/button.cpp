#include "button.hpp"

bool gfx::Button::isHovered(int mouse_x, int mouse_y) const {
    RectShape rect = getTranslatedRect();
    return !disabled && mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

void gfx::Button::render(int mouse_x, int mouse_y, bool button_pressed) {
    RectShape rect = getTranslatedRect();
    
    while(timer_counter < timer.getTimeElapsed()) {
        int hover_progress_target = isHovered(mouse_x, mouse_y) ? (button_pressed ? 200 : 255) : 0;
        hover_progress += float(hover_progress_target - hover_progress) / 40;
        if(std::abs(hover_progress_target - hover_progress) <= 5)
            hover_progress = hover_progress_target;
        timer_counter++;
    }
    
    Color button_color{
        (unsigned char)(hover_color.r * hover_progress / 255 + def_color.r * float(255 - hover_progress) / 255),
        (unsigned char)(hover_color.g * hover_progress / 255 + def_color.g * float(255 - hover_progress) / 255),
        (unsigned char)(hover_color.b * hover_progress / 255 + def_color.b * float(255 - hover_progress) / 255),
        (unsigned char)(hover_color.a * hover_progress / 255 + def_color.a * float(255 - hover_progress) / 255),
    };
    
    Color button_border_color{
        (unsigned char)(border_hover_color.r * hover_progress / 255 + def_border_color.r * float(255 - hover_progress) / 255),
        (unsigned char)(border_hover_color.g * hover_progress / 255 + def_border_color.g * float(255 - hover_progress) / 255),
        (unsigned char)(border_hover_color.b * hover_progress / 255 + def_border_color.b * float(255 - hover_progress) / 255),
        (unsigned char)(border_hover_color.a * hover_progress / 255 + def_border_color.a * float(255 - hover_progress) / 255),
    };
    
    int padding = (255 - hover_progress) / 255 * 30;
    RectShape hover_rect(rect.x + padding, rect.y + padding, std::max(0, rect.w - 2 * padding), std::max(0, rect.h - 2 * padding));
    rect.render(def_color);
    rect.renderOutline(def_border_color);
    hover_rect.render(button_color);
    hover_rect.renderOutline(button_border_color);
    
    float texture_scale = scale + hover_progress / 255.f * 0.2;
    Texture::render(texture_scale, rect.x + rect.w / 2 - getTextureWidth() * texture_scale / 2, rect.y + rect.h / 2 - getTextureHeight() * texture_scale / 2);
}

void gfx::Button::loadFromSurface(const Surface& surface) {
    Texture::loadFromSurface(surface);
    updateSize();
}

void gfx::Button::setScale(float scale_) {
    scale = scale_;
    updateSize();
}

float gfx::Button::getScale() const {
    return scale;
}

void gfx::Button::setMargin(int margin_) {
    margin = margin_;
    updateSize();
}

int gfx::Button::getMargin() const {
    return margin;
}

int gfx::Button::getWidth() const {
    return w;
}

int gfx::Button::getHeight() const {
    return h;
}

void gfx::Button::updateSize() {
    w = (getTextureWidth() + margin * 2) * scale;
    h = (getTextureHeight() + margin * 2) * scale;
}
