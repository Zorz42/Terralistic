#include <stdexcept>
#include "textInput.hpp"
#include "font.hpp"

void gfx::TextInput::setText(const std::string& text_) {
    text = text_;
    loadFromText(text, text_color);
}

void gfx::TextInput::eraseSelected() {
    text.erase(cursor[0], cursor[1] - cursor[0]);
    cursor[1] = cursor[0];
    loadFromText(text, text_color);
}

int gfx::TextInput::findLeftMove(int curr_pos) {
    int new_pos = std::max(0, curr_pos - 1);
    if(key_states[(int)Key::CTRL])
        while(new_pos != 0 && cursor[0] != new_pos && text[new_pos - 1] != ' ' && text[new_pos - 1] != '-')
            new_pos--;
    return new_pos;
}

int gfx::TextInput::findRightMove(int curr_pos) {
    int new_pos = std::min((int)text.size(), curr_pos + 1);
    if(key_states[(int)Key::CTRL])
        while(new_pos != text.size() && new_pos != cursor[1] && text[new_pos] != ' ' && text[new_pos] != '-')
            new_pos++;
    return new_pos;
}

int gfx::TextInput::getWidth() const {
    return (width + 2 * margin) * scale;
}

gfx::TextInput::TextInput() {
    margin = 3;
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
    back_rect.setX(rect.x);
    back_rect.setY(rect.y);
    back_rect.setWidth(rect.w);
    back_rect.setHeight(rect.h);
    back_rect.fill_color = isHovered(mouse_x, mouse_y) ? hover_color : def_color;
    back_rect.render();
    
    rect.x += margin * scale;
    rect.y += margin * scale;
    rect.w = getTextureWidth() * scale;
    rect.h -= margin * 2 * scale;
    int x, w;
    if (rect.w > width * scale) {
        x = rect.w / scale - width;
        w = width;
    }
    else {
        x = 0;
        w = rect.w / scale;
    }
    
    if (active) {
        int width_before = 0;
        for(char i : text.substr(0, cursor[0]))
            width_before += font_rects[(int)(unsigned char)i].w + TEXT_SPACING;
        
        int width_of_selection = 0;
        for(char i : text.substr(cursor[0], cursor[1] - cursor[0]))
            width_of_selection += font_rects[(int)(unsigned char)i].w + TEXT_SPACING;


        //if(!width_before)
            //width_before = 1;
        
        if(!width_of_selection)
            width_of_selection = 1;
        
        Color box_color = width_of_selection == 1 ? Color{255, 255, 255} : Color{230, 230, 230, 150};

        RectShape(rect.x + width_before * scale, rect.y, scale * width_of_selection, rect.h).render(box_color);
    }

    Texture::render(scale, rect.x, rect.y, {x, 0, w, int(rect.h / scale)});
}

void gfx::TextInput::setBorderColor(Color color) {
    back_rect.border_color = color;
}
