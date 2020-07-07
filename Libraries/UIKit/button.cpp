//
//  button.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 07/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "UIKit.hpp"

ui::button::button(ogl::objectType type) {
    setCenteredX(type == ogl::centered);
    setCenteredY(type == ogl::centered);
    setMargin(10);
    setText(" ");
}

void ui::button::setHoverColor(Uint8 r, Uint8 g, Uint8 b) {
    hover_r = r;
    hover_g = g;
    hover_b = b;
}

void ui::button::setTextColor(Uint8 r, Uint8 g, Uint8 b) {
    text_r = r;
    text_g = g;
    text_b = b;
}

void ui::button::render() {
    SDL_Rect render_rect = getRect();
    if(hovered())
        swl::setDrawColor(hover_r, hover_g, hover_b);
    else
        swl::setDrawColor(r, g, b);
    swl::render(render_rect, fill);
    if(centered_x)
        text.setX(x);
    else
        text.setX(x + margin * text.scale);
    
    if(centered_y)
        text.setY(y);
    else
        text.setY(y + margin * text.scale);
    text.render();
}

bool ui::button::hovered() {
    return touchesPoint(swl::mouse_x, swl::mouse_y);
}

void ui::button::setText(std::string text_) {
    text.loadFromText(text_, SDL_Color{text_r, text_g, text_b});
    setMargin(margin);
}

void ui::button::setScale(Uint8 scale) {
    text.scale = scale;
    setMargin(margin);
}

bool ui::button::isPressed(SDL_Event &event) {
    return event.type == SDL_MOUSEBUTTONDOWN && event.button.button == SDL_BUTTON_LEFT && hovered();
}

void ui::button::setMargin(unsigned short margin_) {
    margin = margin_;
    setWidth(text.getWidth() + 2 * margin * text.scale);
    setHeight(text.getHeight() + 2 * margin * text.scale);
}

void ui::button::setCenteredX(bool input) {
    centered_x = input;
    text.centered_x = input;
}

void ui::button::setCenteredY(bool input) {
    centered_y = input;
    text.centered_y = input;
}
