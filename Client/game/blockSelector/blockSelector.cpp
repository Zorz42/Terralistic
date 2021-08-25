#include "blockSelector.hpp"
#include "packetType.hpp"

void BlockSelector::init() {
    select_rect.smooth_factor = 2;
}

void BlockSelector::render() {
    if(getKeyState(gfx::Key::MOUSE_LEFT) != is_left_button_pressed) {
        is_left_button_pressed = getKeyState(gfx::Key::MOUSE_LEFT);
        if(is_left_button_pressed) {
            prev_selected_x = blocks->getWorldWidth();
            prev_selected_y = blocks->getWorldHeight();
        } else {
            sf::Packet packet;
            packet << PacketType::STOPPED_BREAKING;
            manager->sendPacket(packet);
        }
    }
    
    if(!inventory_handler->isHovered()) {
        if((prev_selected_y != selected_block_y || prev_selected_x != selected_block_x) && is_left_button_pressed) {
            sf::Packet packet;
            packet << PacketType::STARTED_BREAKING << selected_block_x << selected_block_y;
            manager->sendPacket(packet);

            prev_selected_x = selected_block_x;
            prev_selected_y = selected_block_y;
        }
        
        selected_block_x = (unsigned short)((gfx::getMouseX() + blocks->view_x - gfx::getWindowWidth() / 2) / (BLOCK_WIDTH * 2));
        selected_block_y = (unsigned short)((gfx::getMouseY() + blocks->view_y - gfx::getWindowHeight() / 2) / (BLOCK_WIDTH * 2));
        select_rect.setX(-blocks->view_x + gfx::getWindowWidth() / 2 + selected_block_x * BLOCK_WIDTH * 2);
        select_rect.setY(-blocks->view_y + gfx::getWindowHeight() / 2 + selected_block_y * BLOCK_WIDTH * 2);
        select_rect.render(false);
    }
}

void BlockSelector::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_RIGHT && !inventory_handler->isHovered()) {
        unsigned short starting_x = (player_handler->getMainPlayer().x) / (BLOCK_WIDTH * 2);
        unsigned short starting_y = (player_handler->getMainPlayer().y) / (BLOCK_WIDTH * 2);
        unsigned short ending_x = (player_handler->getMainPlayer().x + player_handler->getPlayerWidth() * 2 - 1) / (BLOCK_WIDTH * 2);
        unsigned short ending_y = (player_handler->getMainPlayer().y + player_handler->getPlayerHeight() * 2 - 1) / (BLOCK_WIDTH * 2);

        if(selected_block_x < starting_x || selected_block_x > ending_x || selected_block_y < starting_y || selected_block_y > ending_y) {
            sf::Packet packet;
            packet << PacketType::RIGHT_CLICK << selected_block_x << selected_block_y;
            manager->sendPacket(packet);
        }
    }
}
