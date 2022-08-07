#include <set>
#include "chat.hpp"

static const std::set<char> allowed_chars = {'!', ':', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '"', '|', '~', '<', '>', '?', '-', '=', ',', '.', '/', '[', ']', ';', '\'', '\\', '`', ' '};

void Chat::init() {
    networking->packet_event.addListener(this);
    chat_box.setScale(2);
    chat_box.orientation = gfx::BOTTOM_LEFT;
    chat_box.y = -SPACING / 2;
    chat_box.x = SPACING / 2;
    chat_box.textProcessing = [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || allowed_chars.find(c) != allowed_chars.end())
            return c;
        return '\0';
    };
    
    chat_box.def_color.a = TRANSPARENCY;
    chat_box.setBlurIntensity(BLUR);
    chat_box.setBorderColor(BORDER_COLOR);
    chat_box.width = chat_width;
    chat_box.setPassthroughKeys({gfx::Key::ARROW_UP, gfx::Key::ARROW_DOWN});

    text_inputs = {&chat_box};
}

void Chat::update(float frame_length) {
    enabled = players->getMainPlayer() != nullptr;
    
    if(enabled) {
        int target_width = chat_box.active ? 300 : 100;
        while(timer_counter < timer.getTimeElapsed()) {
            timer_counter++;
            chat_width += (target_width - chat_width) / 40;
        }
        chat_box.width = chat_width;
        
        for(auto & chat_line : chat_lines)
            chat_line->text_sprite.y += (chat_line->y_to_be - chat_line->text_sprite.y) / 2;
    }
}

void Chat::render() {
    chat_box.render(getMouseX(), getMouseY());
    
    for(auto & chat_line : chat_lines) {
        if(!chat_line->text.empty()) {
            chat_line->text_sprite.loadFromSurface(gfx::textToSurface(chat_line->text));
            chat_line->text_sprite.setScale(2);
            chat_line->text_sprite.y = chat_box.y;
            chat_line->text_sprite.x = SPACING / 2;
            chat_line->text_sprite.orientation = gfx::BOTTOM_LEFT;
            chat_line->y_to_be = chat_box.y - chat_box.getHeight();
            chat_line->text.clear();
            chat_line->text.shrink_to_fit();
            
            for(auto & i2 : chat_lines)
                if(i2 != chat_line)
                    i2->y_to_be -= chat_line->text_sprite.h;
        }
        
        if(chat_line->timer.getTimeElapsed() < 10000 || chat_box.active) {
            float alpha = 10000 - chat_line->timer.getTimeElapsed();
            if(alpha >= 500 || chat_box.active)
                alpha = 500;
            
            if(alpha > 0) {
                chat_line->text_sprite.setColor({255, 255, 255, (unsigned char)(alpha / 500 * 255)});
                chat_line->text_sprite.render();
            }
        }
    }
}

bool Chat::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ENTER && chat_box.active) {
        if(!chat_box.getText().empty()) {
            Packet chat_packet;
            chat_packet << ClientPacketType::CHAT << chat_box.getText();
            networking->sendPacket(chat_packet);
            saved_lines.insert(saved_lines.begin() + 1, chat_box.getText());
            selected_saved_line = 0;
            if(saved_lines.size() > 20)
                saved_lines.erase(saved_lines.end());
            chat_box.setText("");
        }
        chat_box.active = false;
        return true;
    } else if(key == gfx::Key::T && !chat_box.active) {
        chat_box.active = true;
        chat_box.ignore_next_input = true;
        return true;
    } else if(key == gfx::Key::ESCAPE && chat_box.active) {
        chat_box.setText("");
        selected_saved_line = 0;
        chat_box.active = false;
        return true;
    }
    if(key == gfx::Key::ARROW_UP && chat_box.active) {
        if(selected_saved_line < saved_lines.size() - 1) {
            selected_saved_line++;
            chat_box.setText(saved_lines[selected_saved_line]);
        }
        return true;
    } else if(key == gfx::Key::ARROW_DOWN && chat_box.active) {
        if(selected_saved_line > 0){
            selected_saved_line--;
            chat_box.setText(saved_lines[selected_saved_line]);
        }
        return true;
    }
    return false;
}

void Chat::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case ServerPacketType::CHAT: {
            std::string curr_line, whole_message;
            event.packet >> whole_message;
            whole_message.push_back('\n');
            while(!whole_message.empty()) {
                curr_line.push_back(whole_message[0]);
                whole_message.erase(whole_message.begin());
                if(whole_message[0] == '\n') {
                    auto* new_line = new ChatLine;
                    new_line->text = curr_line;
                        chat_lines.push_back(new_line);
                    whole_message.erase(whole_message.begin());
                    curr_line = "";
                }
            }
            break;
        }
        default:;
    }
}

void Chat::stop() {
    for(auto & chat_line : chat_lines)
        delete chat_line;
    
    networking->packet_event.removeListener(this);
}
