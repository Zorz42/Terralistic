#include "graphics-internal.hpp"

int gfx::Button::getWidth() const {
    return (getTextureWidth() + margin * 2) * scale;
}

int gfx::Button::getHeight() const {
    return (getTextureHeight() + margin * 2) * scale;
}

bool gfx::Button::isHovered(int mouse_x, int mouse_y) const {
    RectShape rect = getTranslatedRect();
    return !disabled && mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

void gfx::Button::render(int mouse_x, int mouse_y) {
    RectShape rect = getTranslatedRect();
    int hover_progress_target = isHovered(mouse_x, mouse_y) ? (key_states[(int)Key::MOUSE_LEFT] ? 200 : 255) : 0;
    hover_progress += float(hover_progress_target - hover_progress) / 3;
    Color button_color{
        (unsigned char)((int)hover_color.r * hover_progress / 255 + (int)def_color.r * float(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.g * hover_progress / 255 + (int)def_color.g * float(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.b * hover_progress / 255 + (int)def_color.b * float(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.a * hover_progress / 255 + (int)def_color.a * float(255 - hover_progress) / 255),
    };
    Color button_border_color{
        (unsigned char)((int)border_hover_color.r * hover_progress / 255 + (int)def_border_color.r * float(255 - hover_progress) / 255),
        (unsigned char)((int)border_hover_color.g * hover_progress / 255 + (int)def_border_color.g * float(255 - hover_progress) / 255),
        (unsigned char)((int)border_hover_color.b * hover_progress / 255 + (int)def_border_color.b * float(255 - hover_progress) / 255),
        (unsigned char)((int)border_hover_color.a * hover_progress / 255 + (int)def_border_color.a * float(255 - hover_progress) / 255),
    };
    
    int padding = (255 - hover_progress) / 255 * 30;
    RectShape hover_rect(rect.x + padding, rect.y + padding, std::max(0, rect.w - 2 * padding), std::max(0, rect.h - 2 * padding));
    rect.render(def_color);
    rect.renderOutline(def_border_color);
    hover_rect.render(button_color);
    hover_rect.renderOutline(button_border_color);
    
    Texture::render(scale, rect.x + margin * scale, rect.y + margin * scale);
}
