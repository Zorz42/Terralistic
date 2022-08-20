#include "button.hpp"

bool gfx::Button::isHovered(int mouse_x, int mouse_y, int mouse_vel) const {
    RectShape rect = getTranslatedRect();
    return !disabled && mouse_vel < 1500 && mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

void gfx::Button::render(int mouse_x, int mouse_y, int mouse_vel, bool button_pressed) {
    RectShape rect = getTranslatedRect();
    
    int hover_progress_target = isHovered(mouse_x, mouse_y, mouse_vel) ? (button_pressed ? 0.8 : 1) : 0;
    while(timer_counter < timer.getTimeElapsed()) {
        hover_progress += float(hover_progress_target - hover_progress) / 40;
        if(std::abs(hover_progress_target - hover_progress) <= 0.01)
            hover_progress = hover_progress_target;
        timer_counter++;
    }
    
    Color button_color{
        (unsigned char)(hover_color.r * hover_progress + def_color.r * (1 - hover_progress)),
        (unsigned char)(hover_color.g * hover_progress + def_color.g * (1 - hover_progress)),
        (unsigned char)(hover_color.b * hover_progress + def_color.b * (1 - hover_progress)),
        (unsigned char)(hover_color.a * hover_progress + def_color.a * (1 - hover_progress)),
    };
    
    Color button_border_color{
        (unsigned char)(border_hover_color.r * hover_progress + def_border_color.r * (1 - hover_progress)),
        (unsigned char)(border_hover_color.g * hover_progress + def_border_color.g * (1 - hover_progress)),
        (unsigned char)(border_hover_color.b * hover_progress + def_border_color.b * (1 - hover_progress)),
        (unsigned char)(border_hover_color.a * hover_progress + def_border_color.a * (1 - hover_progress)),
    };
    
    int padding = (1 - hover_progress) * 30;
    RectShape hover_rect(rect.x + padding, rect.y + padding, std::max(0, rect.w - 2 * padding), std::max(0, rect.h - 2 * padding));
    rect.render(def_color);
    rect.renderOutline(def_border_color);
    hover_rect.render(button_color);
    hover_rect.renderOutline(button_border_color);
    
    float texture_scale = scale + hover_progress * 0.4;
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
