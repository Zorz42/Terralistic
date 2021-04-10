//
//  blockSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#include "blockSelector.hpp"
#include "playerHandler.hpp"

// this is a rectangle with which you select which block to break or where to place selected block

struct clickEvents {
    void (*rightClickEvent)(map::block*, game*) = nullptr;
    void (*leftClickEvent)(map::block*, game*) = nullptr;
};

static std::vector<clickEvents> click_events;

// you can register special click events to blocks for custom behaviour
void grass_block_leftClickEvent(map::block* block, game* scene) {
    block->setType(map::blockType::DIRT);
}

void air_rightClickEvent(map::block* block, game* scene) {
    map::blockType type = scene->player_inventory.getSelectedSlot()->getUniqueItem().places;
    if(type != map::blockType::AIR && scene->player_inventory.getSelectedSlot()->decreaseStack(1)) {
        block->setType(type);
        block->lightUpdate();
    }
}

void air_leftClickEvent(map::block* block, game* scene) {}

void blockSelector::initEvents() {
    click_events = std::vector<clickEvents>(map::unique_blocks.size());

    click_events[(int)map::blockType::GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    click_events[(int)map::blockType::AIR].rightClickEvent = &air_rightClickEvent;
    click_events[(int)map::blockType::AIR].leftClickEvent = &air_leftClickEvent;
}

void blockSelector::render() {
    if((prev_selected_y != selected_block_y || prev_selected_x != selected_block_x) && is_left_button_pressed) {
        if(scene->multiplayer) {
            packets::packet packet(packets::STARTED_BREAKING);
            packet << selected_block_x << selected_block_y;
            scene->networking_manager.sendPacket(packet);
        }
        prev_selected_x = selected_block_x;
        prev_selected_y = selected_block_y;
    }
    
    if(is_left_button_pressed && !scene->multiplayer) {
        map::block block = scene->world_map->getBlock(selected_block_x, selected_block_y);
        if(click_events[(int)block.getType()].leftClickEvent)
            click_events[(int)block.getType()].leftClickEvent(&block, scene);
        else {
            block.setBreakProgress(block.getBreakProgress() + gfx::getDeltaTime());
            if(!scene->multiplayer && block.getBreakProgress() >= block.getBreakTime())
                block.breakBlock();
        }
    }
    
    if(!playerHandler::hovered) {
        selected_block_x = (unsigned short)(gfx::getMouseX() + playerHandler::view_x - gfx::getWindowWidth() / 2) / BLOCK_WIDTH;
        selected_block_y = (unsigned short)(gfx::getMouseY() + playerHandler::view_y - gfx::getWindowHeight() / 2) / BLOCK_WIDTH;
        select_rect.x = -playerHandler::view_x + gfx::getWindowWidth() / 2 + selected_block_x * BLOCK_WIDTH;
        select_rect.y = -playerHandler::view_y + gfx::getWindowHeight() / 2 + selected_block_y * BLOCK_WIDTH;
        gfx::render(select_rect, false);
    }
}

void blockSelector::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && !playerHandler::hovered) {
        is_left_button_pressed = true;
        prev_selected_x = scene->world_map->getWorldWidth();
        prev_selected_y = scene->world_map->getWorldHeight();
    } else if(key == gfx::KEY_MOUSE_RIGHT && !gfx::colliding(playerHandler::player.getTranslatedRect(), select_rect.getTranslatedRect()) && !playerHandler::hovered) {
        if(scene->multiplayer) {
            packets::packet packet(packets::RIGHT_CLICK);
            packet << selected_block_x << selected_block_y;
            scene->networking_manager.sendPacket(packet);
        } else {
            map::block block = scene->world_map->getBlock(selected_block_x, selected_block_y);
            if(click_events[(int)block.getType()].rightClickEvent)
                click_events[(int)block.getType()].rightClickEvent(&block, scene);
        }
    }
}

void blockSelector::onKeyUp(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && !playerHandler::hovered) {
        is_left_button_pressed = false;
        if(scene->multiplayer) {
            packets::packet packet(packets::STOPPED_BREAKING);
            scene->networking_manager.sendPacket(packet);
        }
    }
}
