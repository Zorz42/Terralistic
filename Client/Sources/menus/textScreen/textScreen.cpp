#include "graphics.hpp"
#include "textScreen.hpp"

void renderTextScreen(const std::string &text) {
    gfx::Sprite text_image;
    text_image.renderText(text);
    text_image.orientation = gfx::CENTER;
    text_image.scale = 3;

    gfx::clearWindow();
    text_image.render();
    gfx::updateWindow();
}
