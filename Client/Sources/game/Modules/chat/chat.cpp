//
//  chat.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/06/2021.
//

#include "chat.hpp"

void chat::init() {
    chat_box.scale = 2;
    chat_box.setText("");
    chat_box.orientation = gfx::bottom_left;
    chat_box.y = -5;
    
    text_inputs = {&chat_box};
}

void chat::update() {
    int target_width = chat_box.active ? 300 : 150;
    chat_box.width += (target_width - (int)chat_box.width) / 3;
    disable_events = chat_box.active;
}

void chat::render() {
    gfx::render(chat_box);
    for(gfx::sprite* i : chat_lines)
        gfx::render(*i);
}

void chat::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_ENTER && chat_box.active) {
        packets::packet chat_packet(packets::CHAT);
        chat_packet << chat_box.getText();
        manager->sendPacket(chat_packet);
        chat_box.setText("");
        chat_box.active = false;
    } else if(key == gfx::KEY_T && !chat_box.active) {
        chat_box.active = true;
        chat_box.ignore_one_input = true;
    }
}

void chat::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::CHAT: {
            gfx::sprite* new_line = new gfx::sprite;
            new_line->setTexture(gfx::renderText(packet.getString(), {255, 255, 255}));
            new_line->scale = 2;
            new_line->y = chat_box.getTranslatedY() - new_line->getHeight();
            
            for(gfx::sprite* i : chat_lines)
                i->y -= new_line->getHeight();
            chat_lines.push_back(new_line);
            break;
        }
        default:;
    }
}
