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
    
    for(chatLine* i : chat_lines)
        i->text_sprite.y += (i->y_to_be - i->text_sprite.y) / 2;
}

void chat::render() {
    for(chatLine* i : chat_lines)
        if(i->time_created + 10500 > gfx::getTicks() || chat_box.active) {
            int alpha = i->time_created + 10500 - gfx::getTicks();
            if(alpha < 0 || alpha >= 500)
                alpha = 500;
            i->text_sprite.setAlpha((float)alpha / 500.0f * 255);
            gfx::render(i->text_sprite);
        }
            
    gfx::render(chat_box);
}

void chat::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_ENTER && chat_box.active) {
        if(!chat_box.getText().empty()) {
            packets::packet chat_packet(packets::CHAT);
            chat_packet << chat_box.getText();
            manager->sendPacket(chat_packet);
            chat_box.setText("");
        }
        chat_box.active = false;
    } else if(key == gfx::KEY_T && !chat_box.active) {
        chat_box.active = true;
        chat_box.ignore_one_input = true;
    } else if(key == gfx::KEY_ESCAPE && chat_box.active) {
        chat_box.setText("");
        chat_box.active = false;
    }
}

void chat::onPacket(packets::packet packet) {
    switch(packet.type) {
        case packets::CHAT: {
            chatLine* new_line = new chatLine;
            new_line->text_sprite.setTexture(gfx::renderText(packet.getString(), {255, 255, 255}));
            new_line->text_sprite.scale = 2;
            new_line->y_to_be = chat_box.getTranslatedY() - new_line->text_sprite.getHeight();
            new_line->text_sprite.y = chat_box.getTranslatedY();
            new_line->time_created = gfx::getTicks();
            
            for(chatLine* i : chat_lines)
                i->y_to_be -= new_line->text_sprite.getHeight();
            chat_lines.push_back(new_line);
            break;
        }
        default:;
    }
}

void chat::stop() {
    for(chatLine* i : chat_lines)
        delete i;
}
