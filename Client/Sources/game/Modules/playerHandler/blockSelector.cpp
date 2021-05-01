//
//  blockSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#include "playerHandler.hpp"
#include "playerRenderer.hpp"

// this is a rectangle with which you select which block to break or where to place selected block

void playerHandler::renderBlockSelector() {
    if((prev_selected_y != selected_block_y || prev_selected_x != selected_block_x) && is_left_button_pressed) {
        packets::packet packet(packets::STARTED_BREAKING);
        packet << selected_block_x << selected_block_y;
        manager->sendPacket(packet);

        prev_selected_x = selected_block_x;
        prev_selected_y = selected_block_y;
    }
    
    if(!playerHandler::hovered) {
        selected_block_x = (unsigned short)(gfx::getMouseX() + world_map->view_x - gfx::getWindowWidth() / 2) / BLOCK_WIDTH;
        selected_block_y = (unsigned short)(gfx::getMouseY() + world_map->view_y - gfx::getWindowHeight() / 2) / BLOCK_WIDTH;
        select_rect.x = -world_map->view_x + gfx::getWindowWidth() / 2 + selected_block_x * BLOCK_WIDTH;
        select_rect.y = -world_map->view_y + gfx::getWindowHeight() / 2 + selected_block_y * BLOCK_WIDTH;
        gfx::render(select_rect, false);
    }
}

void playerHandler::onKeyDownSelector(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && !playerHandler::hovered) {
        is_left_button_pressed = true;
        prev_selected_x = world_map->getWorldWidth();
        prev_selected_y = world_map->getWorldHeight();
    } else if(key == gfx::KEY_MOUSE_RIGHT && !playerHandler::hovered) {
        gfx::rectShape rect = gfx::rectShape(gfx::getWindowWidth() / 2 - playerRenderer::getPlayerWidth() / 2, gfx::getWindowHeight() / 2 - playerRenderer::getPlayerHeight() / 2, playerRenderer::getPlayerWidth(), playerRenderer::getPlayerHeight());
        if(!gfx::colliding(rect, select_rect.getTranslatedRect())) {
            packets::packet packet(packets::RIGHT_CLICK);
            packet << selected_block_x << selected_block_y;
            manager->sendPacket(packet);
        }
    }
}

void playerHandler::onKeyUpSelector(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && !playerHandler::hovered) {
        is_left_button_pressed = false;
        packets::packet packet(packets::STOPPED_BREAKING);
        manager->sendPacket(packet);
    }
}
