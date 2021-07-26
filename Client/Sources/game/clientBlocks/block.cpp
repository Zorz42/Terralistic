//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include <algorithm>
#include <utility>
#include "clientBlocks.hpp"
#include "properties.hpp"
#include "resourcePack.hpp"
#include <cassert>

ClientBlock ClientBlocks::getBlock(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < getWorldHeight() && x >= 0 && x < getWorldWidth());
    return ClientBlock(x, y, &blocks[y * getWorldWidth() + x], this);
}

void ClientBlock::setType(BlockType block_id, LiquidType liquid_id) {
    block_data->block_id = block_id;
    block_data->liquid_id = liquid_id;
    update();
}

void ClientBlock::setLightLevel(unsigned char level) {
    block_data->light_level = level;
    update();
}

const BlockInfo& ClientMapBlock::getUniqueBlock() const {
    return ::getBlockInfo(block_id);
}

const LiquidInfo& ClientMapBlock::getUniqueLiquid() const {
    return ::getLiquidInfo(liquid_id);
}

void ClientBlocks::renderBlocksBack() {
    // figure out, what the window is covering and only render that
    short begin_x = view_x / (BLOCK_WIDTH << 4) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH << 4) - 1;
    short end_x = view_x / (BLOCK_WIDTH << 4) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH << 4) + 2;

    short begin_y = view_y / (BLOCK_WIDTH << 4) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH << 4) - 1;
    short end_y = view_y / (BLOCK_WIDTH << 4) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH << 4) + 2;

    if(begin_x < 0)
        begin_x = 0;
    if(end_x > getWorldWidth() >> 4)
        end_x = getWorldWidth() >> 4;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > getWorldHeight() >> 4)
        end_y = getWorldHeight() >> 4;


    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++) {
            if(getChunk(x, y).getState() == ChunkState::unloaded) {
                sf::Packet packet;
                packet << PacketType::CHUNK << x << y;
                networking_manager->sendPacket(packet);
                getChunk(x, y).setState(ChunkState::pending_load);
            } else if(getChunk(x, y).getState() == ChunkState::loaded) {
                if(getChunk(x, y).hasToUpdate())
                    getChunk(x, y).updateTexture();
                getChunk(x, y).drawBack();
            }
        }
}

void ClientBlocks::renderBlocksFront() {
    // figure out, what the window is covering and only render that
    short begin_x = view_x / (BLOCK_WIDTH << 4) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH << 4) - 1;
    short end_x = view_x / (BLOCK_WIDTH << 4) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH << 4) + 2;

    short begin_y = view_y / (BLOCK_WIDTH << 4) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH << 4) - 1;
    short end_y = view_y / (BLOCK_WIDTH << 4) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH << 4) + 2;

    if(begin_x < 0)
        begin_x = 0;
    if(end_x > getWorldWidth() >> 4)
        end_x = getWorldWidth() >> 4;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > getWorldHeight() >> 4)
        end_y = getWorldHeight() >> 4;

    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++)
            if(getChunk(x, y).getState() == ChunkState::loaded) {
                if(getChunk(x, y).hasToUpdate())
                    getChunk(x, y).updateTexture();
                getChunk(x, y).drawFront();
            }
}

void ClientBlock::updateOrientation() {
    if(parent_map->resource_pack->getBlockTexture(getType()).getTextureHeight() != 8) {
        block_data->orientation = 0;
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        unsigned char c = 1;
        for(int i = 0; i < 4; i++) {
            if(
               x + x_[i] >= parent_map->getWorldWidth() || x + x_[i] < 0 || y + y_[i] >= parent_map->getWorldHeight() || y + y_[i] < 0 ||
               parent_map->getBlock(x + x_[i], y + y_[i]).getType() == getType() ||
               std::count(block_data->getUniqueBlock().connects_to.begin(), block_data->getUniqueBlock().connects_to.end(), parent_map->getBlock(x + x_[i], y + y_[i]).getType())
            )
                block_data->orientation += c;
            c += c;
        }
    }
    block_data->update = false;
}

void ClientBlock::drawBack() {
    gfx::Rect rect((x & 15) * BLOCK_WIDTH, (y & 15) * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH, { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel()) });
    
    if(getLightLevel())
        parent_map->resource_pack->getBlockTexture(getType()).render(2, rect.x, rect.y, gfx::RectShape(0, short((BLOCK_WIDTH >> 1) * block_data->orientation), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));

    if(getBreakStage())
        parent_map->resource_pack->getBreakingTexture().render(2, rect.x, rect.y, gfx::RectShape(0, short(BLOCK_WIDTH / 2 * (getBreakStage() - 1)), BLOCK_WIDTH / 2, BLOCK_WIDTH / 2));
}

void ClientBlock::drawFront() {
    gfx::Rect rect((x & 15) * BLOCK_WIDTH, (y & 15) * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH, { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel()) });

    if(getLiquidType() != LiquidType::EMPTY) {
        int level = ((int)getLiquidLevel() + 1) / 16;
        parent_map->resource_pack->getLiquidTexture(getLiquidType()).render(2, rect.x, rect.y + BLOCK_WIDTH - level * 2, gfx::RectShape(0, 0, BLOCK_WIDTH / 2, level));
    }
    
    if(getLightLevel() != MAX_LIGHT)
        rect.render();
}

void ClientBlock::scheduleTextureUpdate() {
    block_data->update = true;
    parent_map->getChunk(x >> 4, y >> 4).scheduleUpdate();
}

void ClientBlock::update() {
    scheduleTextureUpdate();

    // also update neighbors
    if(x != 0)
        parent_map->getBlock(x - 1, y).scheduleTextureUpdate();
    if(x != parent_map->getWorldWidth() - 1)
        parent_map->getBlock(x + 1, y).scheduleTextureUpdate();
    if(y != 0)
        parent_map->getBlock(x, y - 1).scheduleTextureUpdate();
    if(y != parent_map->getWorldHeight() - 1)
        parent_map->getBlock(x, y + 1).scheduleTextureUpdate();
}

void ClientBlock::setBreakStage(unsigned char stage) {
    block_data->break_stage = stage;
    update();
}
