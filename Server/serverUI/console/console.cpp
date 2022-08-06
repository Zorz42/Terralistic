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

    input_box.def_color.a = 255;
    input_box.setBlurIntensity(1);
    input_box.setBorderColor(BORDER_COLOR);
    input_box.width = 100;
    input_box.setPassthroughKeys({gfx::Key::ARROW_UP, gfx::Key::ARROW_DOWN});
    text_inputs = {&input_box};
}



void Console::update(float frame_length) {
    if(width != texture.getTextureWidth() || height != texture.getTextureHeight())
        texture.createBlankImage(width, height);
    input_box.width = width / 2 - 15;
    input_box.x = 10;
    input_box.y = (float)height - 10 - (float)input_box.getHeight();

    texture.setRenderTarget();

    gfx::RectShape(0, 0, width, height).render(DARK_GREY);//can be removed once setMinimumWindoeSize works
    gfx::RectShape(2, 2, width - 4, height - 4).render(GREY);
    input_box.render(getMouseX(), getMouseY());//you are not able to click directly onto the text input if this module is not in the upper right position. will be fixed with the implementation of UI containers

    gfx::resetRenderTarget();
}

bool Console::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ENTER && input_box.active) {
        if(!input_box.getText().empty()) {
            Packet chat_packet;
            chat_packet << ServerPacketType::CHAT << ("[Server] "+ input_box.getText());
            server->getNetworking()->sendToEveryone(chat_packet);
            saved_lines.insert(saved_lines.begin() + 1, input_box.getText());
            selected_saved_line = 0;
            if(saved_lines.size() > 20)
                saved_lines.erase(saved_lines.end());
            input_box.setText("");
        }
        input_box.active = false;
        return true;
    } else if(key == gfx::Key::ESCAPE && input_box.active) {
        input_box.setText("");
        selected_saved_line = 0;
        input_box.active = false;
        return true;
    }
    if(key == gfx::Key::ARROW_UP && input_box.active) {
        if(selected_saved_line < saved_lines.size() - 1) {
            selected_saved_line++;
            input_box.setText(saved_lines[selected_saved_line]);
            input_box.setCursor(input_box.getText().size());
        }
        return true;
    } else if(key == gfx::Key::ARROW_DOWN && input_box.active) {
        if(selected_saved_line > 0){
            selected_saved_line--;
            input_box.setText(saved_lines[selected_saved_line]);
            input_box.setCursor(input_box.getText().size());
        }
        return true;
    }
    return false;
}
