//
//  blockSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#include "core.hpp"

#include "blockSelector.hpp"
#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "game.hpp"

// this is a rectangle with which you select which block to break or where to place selected block

struct clickEvents {
    void (*rightClickEvent)(scene->world_map.block*, game*) = nullptr;
    void (*leftClickEvent)(scene->world_map.block*, game*) = nullptr;
};

static std::vector<clickEvents> click_events;

// you can register special click events to blocks for custom behaviour
void grass_block_leftClickEvent(scene->world_map.block* block, game* scene) {
    block->setBlockType(scene->world_map.DIRT);
}

void air_rightClickEvent(scene->world_map.block* block, game* scene) {
    map::blockType type = scene->player_inventory.getSelectedSlot()->getUniqueItem().places;
    if(type != scene->world_map.AIR && scene->player_inventory.getSelectedSlot()->decreaseStack(1)) {
        block->setBlockType(type);
        block->light_update();
    }
}

void air_leftClickEvent(scene->world_map.block* block, game* scene) {}

INIT_SCRIPT
    INIT_ASSERT(!scene->world_map.unique_blocks.empty());
    click_events = std::vector<clickEvents>(scene->world_map.unique_blocks.size());

    click_events[scene->world_map.GRASS_BLOCK].leftClickEvent = &grass_block_leftClickEvent;
    click_events[scene->world_map.AIR].rightClickEvent = &air_rightClickEvent;
    click_events[scene->world_map.AIR].leftClickEvent = &air_leftClickEvent;
INIT_SCRIPT_END

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
        scene->world_map.block* block = &scene->world_map.getBlock(selected_block_x, selected_block_y);
        if(click_events[block->block_id].leftClickEvent)
            click_events[block->block_id].leftClickEvent(block, scene);
        else {
            block->setBreakProgress(block->break_progress_ms + gfx::getDeltaTime());
            if(!scene->multiplayer && block->break_progress_ms >= block->getUniqueBlock().break_time)
                block->break_block();
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
        prev_selected_x = scene->world_map.getWorldWidth();
        prev_selected_y = scene->world_map.getWorldHeight();
    } else if(key == gfx::KEY_MOUSE_RIGHT && !gfx::colliding(playerHandler::player.getTranslatedRect(), select_rect.getTranslatedRect()) && !playerHandler::hovered) {
        if(scene->multiplayer) {
            packets::packet packet(packets::RIGHT_CLICK);
            packet << selected_block_x << selected_block_y;
            scene->networking_manager.sendPacket(packet);
        } else {
            scene->world_map.block& block = scene->world_map.getBlock(selected_block_x, selected_block_y);
            if(click_events[block.block_id].rightClickEvent)
                click_events[block.block_id].rightClickEvent(&block, scene);
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
