//
//  button.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 07/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "UIKit.hpp"

ui::button::button(ogl::objectType type) {
    setOrientation(type);
    text.setOrientation(ogl::top_left);
    setMargin(10);
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

void ui::button::render(bool display_hover) {
    SDL_Rect render_rect = getRect();
    if(hovered() && display_hover)
        swl::setDrawColor(hover_r, hover_g, hover_b);
    else
        swl::setDrawColor(r, g, b);
    swl::render(render_rect, fill);
    
    text.setX(getX() + margin * text.scale);
    text.setY(getY() + margin * text.scale);
    
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
