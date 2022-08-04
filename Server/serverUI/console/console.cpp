#include "console.hpp"
#include <set>
#include <chrono>
#include <ctime>

static const std::set<char> allowed_chars = {'!', ':', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '"', '|', '~', '<', '>', '?', '-', '=', ',', '.', '/', '[', ']', ';', '\'', '\\', '`', ' '};

Console::Console(float x_, float y_, float w_, float h_): LauncherModule("console") {
    target_x = x_;
    target_y = y_;
    target_w = w_;
    target_h = h_;
    min_width = 300;
    min_height = 90;
    texture.createBlankImage(width, height);
}

void Console::init() {
    input_box.scale = 2;
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
    input_box.x = 10;
    input_box.y = 10;//(float)height - 10 - (float)input_box.getHeight();

    texture.setRenderTarget();

    gfx::RectShape(0, 0, width, height).render(DARK_GREY);//can be removed once setMinimumWindoeSize works
    gfx::RectShape(2, 2, width - 4, height - 4).render(GREY);
    input_box.render(getMouseX(), getMouseY());

    gfx::resetRenderTarget();
}
