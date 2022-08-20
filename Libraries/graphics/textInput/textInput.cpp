#include <stdexcept>
#include "textInput.hpp"
#include "font.hpp"
#include <math.h>

void gfx::TextInput::setText(const std::string& text_) {
    text = text_;
    loadFromSurface(textToSurface(text, text_color));
}

void gfx::TextInput::eraseSelected() {
    text.erase(cursor[0], cursor[1] - cursor[0]);
    cursor[1] = cursor[0];
    loadFromSurface(textToSurface(text, text_color));
}

int gfx::TextInput::findLeftMove(int curr_pos, bool is_ctrl_pressed) {
    int new_pos = std::max(0, curr_pos - 1);
    if(is_ctrl_pressed)
        while(new_pos != 0 && cursor[0] != new_pos && text[new_pos - 1] != ' ' && text[new_pos - 1] != '-')
            new_pos--;
    return new_pos;
}

int gfx::TextInput::findRightMove(int curr_pos, bool is_ctrl_pressed) {
    int new_pos = std::min((int)text.size(), curr_pos + 1);
    if(is_ctrl_pressed)
        while(new_pos != text.size() && new_pos != cursor[1] && text[new_pos] != ' ' && text[new_pos] != '-')
            new_pos++;
    return new_pos;
}

int gfx::TextInput::getWidth() const {
    return (Button::getWidth() + 2 * getMargin()) * getScale();
}

gfx::TextInput::TextInput() {
    setMargin(3);
    back_rect.shadow_intensity = GFX_DEFAULT_TEXT_BOX_SHADOW_INTENSITY;
    setText("");
}

void gfx::TextInput::setBlurIntensity(float blur_intensity) {
    if(blur_intensity <= 0)
        throw std::runtime_error("Blur intensity must be positive.");
    back_rect.blur_radius = blur_intensity;
}

#define TEXT_SPACING 1

void gfx::TextInput::render(int mouse_x, int mouse_y) {
    RectShape rect = getTranslatedRect();
    back_rect.x = rect.x;
    back_rect.y = rect.y;
    back_rect.w = rect.w;
    back_rect.h = rect.h;
    back_rect.fill_color = isHovered(mouse_x, mouse_y) ? hover_color : def_color;
    back_rect.render();
    
    rect.x += getMargin() * getScale();
    rect.y += getMargin() * getScale();
    rect.w = getTextureWidth() * getScale();
    rect.h -= getMargin() * 2 * getScale();
    int x, w;
    if (rect.w > width * getScale()) {
        x = rect.w / getScale() - width;
        w = width;
    }
    else {
        x = 0;
        w = rect.w / getScale();
    }
    
    if (active) {
        cursor[0] = std::min(cursor[0], (int)text.size());
        cursor[1] = std::min(cursor[1], (int)text.size());

        int width_before = 0;
        for(char i : text.substr(0, cursor[0]))
            width_before += getCharWidth(i) + TEXT_SPACING;
        
        int width_of_selection = 0;
        for(char i : text.substr(cursor[0], cursor[1] - cursor[0]))
            width_of_selection += getCharWidth(i) + TEXT_SPACING;
        
        if(!width_of_selection)
            width_of_selection = 1;
        
        Color box_color = width_of_selection == 1 ? Color{255, 255, 255} : Color{230, 230, 230, 150};

        RectShape(rect.x + width_before * getScale(), rect.y, getScale() * width_of_selection, rect.h).render(box_color);
    }

    Texture::render(getScale(), rect.x, rect.y, {x, 0, w, int(rect.h / getScale())});
}

void gfx::TextInput::setBorderColor(Color color) {
    back_rect.border_color = color;
}
