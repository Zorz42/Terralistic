#include <set>
#include "chat.hpp"

static const std::set<char> allowed_chars = {'!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '"', '|', '~', '<', '>', '?', '-', '=', ',', '.', '/', '[', ']', ';', '\'', '\\', '`'};

void Chat::init() {
    chat_box.scale = 2;
    chat_box.setText("");
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
    chat_box.width = 100;
    
    text_inputs = {&chat_box};
}

void Chat::update() {
    int target_width = chat_box.active ? 300 : 100;
    chat_box.width += (target_width - (int)chat_box.width) / 3;
    
    for(ChatLine* i : chat_lines)
        i->text_sprite.y += (i->y_to_be - i->text_sprite.y) / 2;
}

void Chat::render() {
    chat_box.render(mouse_x, mouse_y);
    
    for(ChatLine* i : chat_lines) {
        if(!i->text.empty()) {
            i->text_sprite.loadFromText(i->text);
            i->text_sprite.scale = 2;
            i->text_sprite.y = chat_box.y;
            i->text_sprite.x = SPACING / 2;
            i->text_sprite.orientation = gfx::BOTTOM_LEFT;
            i->y_to_be = chat_box.y - chat_box.getHeight();
            i->text.clear();
            i->text.shrink_to_fit();
            
            for(ChatLine* line : chat_lines)
                if(line != i)
                    line->y_to_be -= i->text_sprite.getHeight();
        }
        
        if(i->time_created + 10500 > gfx::getTicks() || chat_box.active) {
            int alpha = i->time_created + 10500 - gfx::getTicks();
            if(alpha < 0 || alpha >= 500)
                alpha = 500;
            i->text_sprite.setColor({255, 255, 255, (unsigned char)((float)alpha / 500.f * 255)});
            i->text_sprite.render();
        }
    }
}

void Chat::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ENTER && chat_box.active) {
        if(!chat_box.getText().empty()) {
            sf::Packet chat_packet;
            chat_packet << PacketType::CHAT << chat_box.getText();
            manager->sendPacket(chat_packet);
            chat_box.setText("");
        }
        chat_box.active = false;
    } else if(key == gfx::Key::T && !chat_box.active) {
        chat_box.active = true;
        chat_box.ignore_one_input = true;
    } else if(key == gfx::Key::ESCAPE && chat_box.active) {
        chat_box.setText("");
        chat_box.active = false;
    }
}

void Chat::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::CHAT: {
            ChatLine* new_line = new ChatLine;
            event.packet >> new_line->text;
            new_line->time_created = gfx::getTicks();
            chat_lines.push_back(new_line);
            break;
        }
        default:;
    }
}

void Chat::stop() {
    for(ChatLine* i : chat_lines)
        delete i;
}
