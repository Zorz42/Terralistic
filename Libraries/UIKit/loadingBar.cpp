//
//  loadingBar.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 08/07/2020.
//

#include "singleWindowLibrary.hpp"
#include "UIKit.hpp"

ui::loadingBar::loadingBar(unsigned short total_progress_, ogl::objectType type) : total_progress(total_progress_) {
    setOrientation(type);
}

void ui::loadingBar::setBackColor(Uint8 r_, Uint8 g_, Uint8 b_) {
    back_r = r_;
    back_g = g_;
    back_b = b_;
}

void ui::loadingBar::render() {
    unsigned short temp_width = getWidth(); // back up width
    SDL_Rect render_rect = getRect();
    swl::setDrawColor(back_r, back_g, back_b);
    swl::render(render_rect);
    swl::setDrawColor(r, g, b);
    progress_width += (temp_width / total_progress * *current_progress - progress_width) / 3;
    render_rect.w = progress_width;
    swl::render(render_rect);
    setWidth(temp_width);
}

void ui::loadingBar::bind(void* progress_variable) {
    current_progress = (unsigned short*)progress_variable;
}
