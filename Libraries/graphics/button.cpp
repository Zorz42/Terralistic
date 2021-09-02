#include "graphics-internal.hpp"

unsigned short gfx::Button::getWidth() const {
    return (getTextureWidth() + margin * 2) * scale;
}

unsigned short gfx::Button::getHeight() const {
    return (getTextureHeight() + margin * 2) * scale;
}

bool gfx::Button::isHovered(unsigned short mouse_x, unsigned short mouse_y) const {
    RectShape rect = getTranslatedRect();
    return !disabled && mouse_x >= rect.x && mouse_y >= rect.y && mouse_x <= rect.x + rect.w && mouse_y <= rect.y + rect.h;
}

void gfx::Button::render(unsigned short mouse_x, unsigned short mouse_y) {
    RectShape rect = getTranslatedRect();
    int hover_progress_target = isHovered(mouse_x, mouse_y) ? 255 : 0;
    hover_progress += (hover_progress_target - (int)hover_progress) / 2;
    Color button_color{
        (unsigned char)((int)hover_color.r * (int)hover_progress / 255 + (int)def_color.r * (int)(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.g * (int)hover_progress / 255 + (int)def_color.g * (int)(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.b * (int)hover_progress / 255 + (int)def_color.b * (int)(255 - hover_progress) / 255),
        (unsigned char)((int)hover_color.a * (int)hover_progress / 255 + (int)def_color.a * (int)(255 - hover_progress) / 255),
    };
    rect.render(button_color);
    float ms = margin * scale;
    Image::render(scale, rect.x + ms, rect.y + ms);
}
