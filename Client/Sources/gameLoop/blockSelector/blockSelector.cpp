//
//  blockSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#define FILENAME blockSelector
#define NAMESPACE blockSelector
#include "core.hpp"

#include "blockSelector.hpp"
#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "gameLoop.hpp"
#include "blockRenderer.hpp"

// this is a rectangle with which you select which block to break or where to place selected block

ogl::rect selectRect(ogl::top_left);
bool left_button_pressed = false;
unsigned short prev_selected_x = 0, prev_selected_y = 0;

INIT_SCRIPT
    selectRect.fill = false;
    selectRect.setColor(255, 0, 0);
    selectRect.setWidth(BLOCK_WIDTH);
    selectRect.setHeight(BLOCK_WIDTH);
INIT_SCRIPT_END

void rightClickEvent(unsigned short x, unsigned short y) {
    if(gameLoop::online) {
        packets::packet packet(packets::RIGHT_CLICK);
        packet << x << y;
        networking::sendPacket(packet);
    } else {
        blockEngine::block* block = &blockEngine::getBlock(x, y);
        if(block->getUniqueBlock().rightClickEvent)
            block->getUniqueBlock().rightClickEvent(block);
    }
}

void leftClickEvent(unsigned short x, unsigned short y) {
    blockEngine::block* block = &blockEngine::getBlock(x, y);
    if(block->getUniqueBlock().leftClickEvent)
        block->getUniqueBlock().leftClickEvent(block);
    else {
        block->setBreakProgress(block->break_progress_ms + gameLoop::frame_length);
        if(!gameLoop::online && block->break_progress_ms >= block->getUniqueBlock().break_time)
            block->break_block();
    }
}

void blockSelector::render() {
    if((prev_selected_y != blockSelector::selected_block_y || prev_selected_x != blockSelector::selected_block_x) && left_button_pressed) {
        packets::packet packet(packets::STARTED_BREAKING);
        packet << blockSelector::selected_block_x << blockSelector::selected_block_y;
        networking::sendPacket(packet);
        prev_selected_x = blockSelector::selected_block_x;
        prev_selected_y = blockSelector::selected_block_y;
    }
    
    if(left_button_pressed && !gameLoop::online)
        leftClickEvent(blockSelector::selected_block_x, blockSelector::selected_block_y);
    
    if(!playerHandler::hovered) {
        selected_block_x = (unsigned short)(swl::mouse_x + playerHandler::view_x - swl::window_width / 2) / BLOCK_WIDTH;
        selected_block_y = (unsigned short)(swl::mouse_y + playerHandler::view_y - swl::window_height / 2) / BLOCK_WIDTH;
        selectRect.setX(short(-playerHandler::view_x + swl::window_width / 2 + selected_block_x * BLOCK_WIDTH));
        selectRect.setY(short(-playerHandler::view_y + swl::window_height / 2 + selected_block_y * BLOCK_WIDTH));
        selectRect.render();
    }
}

bool blockSelector::collidingWithPlayer() {
    return swl::colliding(playerHandler::player.getRect(), selectRect.getRect());
}

void blockSelector::handleEvents(SDL_Event& event) {
    if(event.type == SDL_MOUSEBUTTONDOWN) {
        if(event.button.button == SDL_BUTTON_LEFT && !playerHandler::hovered) {
            left_button_pressed = true;
            prev_selected_x = blockEngine::world_width;
            prev_selected_y = blockEngine::world_height;
        }
        else if(event.button.button == SDL_BUTTON_RIGHT && !blockSelector::collidingWithPlayer() && !playerHandler::hovered)
            rightClickEvent(blockSelector::selected_block_x, blockSelector::selected_block_y);
    }
    else if(event.type == SDL_MOUSEBUTTONUP && event.button.button == SDL_BUTTON_LEFT && !playerHandler::hovered) {
        left_button_pressed = false;
        packets::packet packet(packets::STOPPED_BREAKING);
        networking::sendPacket(packet);
    }
}
