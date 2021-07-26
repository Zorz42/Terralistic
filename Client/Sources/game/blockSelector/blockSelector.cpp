//
//  blockSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#include "blockSelector.hpp"
#include "packetType.hpp"

// this is a rectangle with which you select which block to break or where to place selected block

void BlockSelector::render() {
    if((prev_selected_y != selected_block_y || prev_selected_x != selected_block_x) && is_left_button_pressed) {
        sf::Packet packet;
        packet << PacketType::STARTED_BREAKING << selected_block_x << selected_block_y;
        manager->sendPacket(packet);

        prev_selected_x = selected_block_x;
        prev_selected_y = selected_block_y;
    }
    
    if(!inventory_handler->isHovered()) {
        selected_block_x = (unsigned short)(gfx::getMouseX() + world_map->view_x - gfx::getWindowWidth() / 2) / BLOCK_WIDTH;
        selected_block_y = (unsigned short)(gfx::getMouseY() + world_map->view_y - gfx::getWindowHeight() / 2) / BLOCK_WIDTH;
        select_rect.x = -world_map->view_x + gfx::getWindowWidth() / 2 + selected_block_x * BLOCK_WIDTH;
        select_rect.y = -world_map->view_y + gfx::getWindowHeight() / 2 + selected_block_y * BLOCK_WIDTH;
        select_rect.render(false);
    }
}

void BlockSelector::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && !inventory_handler->isHovered()) {
        is_left_button_pressed = true;
        prev_selected_x = world_map->getWorldWidth();
        prev_selected_y = world_map->getWorldHeight();
    } else if(key == gfx::Key::MOUSE_RIGHT && !inventory_handler->isHovered()) {
        gfx::RectShape rect = gfx::RectShape(gfx::getWindowWidth() / 2 - player_handler->getPlayerWidth() / 2, gfx::getWindowHeight() / 2 - player_handler->getPlayerHeight() / 2, player_handler->getPlayerWidth(), player_handler->getPlayerHeight());
        if(!gfx::colliding(rect, select_rect.getTranslatedRect())) {
            sf::Packet packet;
            packet << PacketType::RIGHT_CLICK << selected_block_x << selected_block_y;
            manager->sendPacket(packet);
        }
    }
}

void BlockSelector::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && !inventory_handler->isHovered()) {
        is_left_button_pressed = false;
        sf::Packet packet;
        packet << PacketType::STOPPED_BREAKING;
        manager->sendPacket(packet);
    }
}
