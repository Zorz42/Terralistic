//
//  textScreen.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 26/03/2021.
//

#include "graphics.hpp"
#include "textScreen.hpp"

void renderTextScreen(const std::string &text) {
    gfx::Sprite text_image;
    text_image.setTexture(gfx::renderText(text, {255, 255, 255}));
    text_image.orientation = gfx::CENTER;
    text_image.scale = 3;

    gfx::clearWindow();
    text_image.render();
    gfx::updateWindow();
}
