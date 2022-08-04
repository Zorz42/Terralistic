#include "console.hpp"
#include <set>
#include <chrono>
#include <ctime>

static const std::set<char> allowed_chars = {'!', ':', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '"', '|', '~', '<', '>', '?', '-', '=', ',', '.', '/', '[', ']', ';', '\'', '\\', '`', ' '};

Console::Console(float x_, float y_, float w_, float h_) {
    target_x = x_;
    target_y = y_;
    target_w = w_;
    target_h = h_;
    min_width = 620;
    min_height = 90;
    texture.createBlankImage(width, height);
}

void Console::init() {
    input_box.scale = 2;
    input_box.orientation = gfx::BOTTOM_LEFT;
    input_box.y = -SPACING >> 1;
    input_box.x = SPACING >> 1;
    input_box.textProcessing = [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || allowed_chars.find(c) != allowed_chars.end())
            return c;
        return '\0';
    };

    input_box.def_color.a = TRANSPARENCY;
    input_box.setBlurIntensity(BLUR);
    input_box.setBorderColor(BORDER_COLOR);
    input_box.width = 100;
    input_box.setPassthroughKeys({gfx::Key::ARROW_UP, gfx::Key::ARROW_DOWN});
}



void Console::update(float frame_length) {
    if(width != texture.getTextureWidth() || height != texture.getTextureHeight())
        texture.createBlankImage(width, height);
    input_box.width = width - 10;

    texture.setRenderTarget();

    gfx::RectShape(2, 2, width - 4, height - 4).render(GREY);

    gfx::resetRenderTarget();
}
