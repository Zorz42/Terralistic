//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 05/04/2021.
//

#include "clientMap.hpp"
#include "assert.hpp"
#include "SimplexNoise.h"

std::vector<map::uniqueBlock> map::unique_blocks;
gfx::image breaking_texture;

void map::initBlocks() {
    unique_blocks = {
        {"air",               /*ghost*/true,  /*connects_to*/{                                                               }},
        {"dirt",              /*ghost*/false, /*connects_to*/{blockType::GRASS_BLOCK, blockType::SNOWY_GRASS_BLOCK           }},
        {"stone_block",       /*ghost*/false, /*connects_to*/{blockType::SNOWY_GRASS_BLOCK                                   }},
        {"grass_block",       /*ghost*/false, /*connects_to*/{blockType::DIRT, blockType::SNOWY_GRASS_BLOCK                  }},
        {"stone",             /*ghost*/true,  /*connects_to*/{                                                               }},
        {"wood",              /*ghost*/true,  /*connects_to*/{blockType::GRASS_BLOCK, blockType::LEAVES                      }},
        {"leaves",            /*ghost*/true,  /*connects_to*/{                                                               }},
        {"sand",              /*ghost*/false, /*connects_to*/{blockType::DIRT, blockType::GRASS_BLOCK, blockType::STONE_BLOCK}},
        {"snowy_grass_block", /*ghost*/false, /*connects_to*/{blockType::DIRT, blockType::GRASS_BLOCK, blockType::STONE_BLOCK}},
        {"snow_block",        /*ghost*/false, /*connects_to*/{blockType::SNOWY_GRASS_BLOCK, blockType::ICE                   }},
        {"ice_block",         /*ghost*/false, /*connects_to*/{blockType::SNOW_BLOCK                                          }},
    };
    
    breaking_texture.setTexture(gfx::loadImageFile("texturePack/misc/breaking.png"));
    breaking_texture.scale = 2;
}

map::uniqueBlock::uniqueBlock(const std::string& name, bool ghost, std::vector<map::blockType> connects_to) : ghost(ghost), name(name), connects_to(connects_to) {
    texture.setTexture(gfx::loadImageFile("texturePack/blocks/" + name + ".png"));
    single_texture = texture.getTextureHeight() == 8;
    texture.scale = 2;
    texture.free_texture = false;
}

map::block map::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < getWorldHeight() && x >= 0 && x < getWorldWidth(), "requested block is out of bounds");
    return block(x, y, &blocks[y * getWorldWidth() + x], this);
}

void map::block::setType(blockType block_id, liquidType liquid_id) {
    block_data->block_id = block_id;
    block_data->liquid_id = liquid_id;
    update();
}

void map::block::setLightLevel(unsigned char level) {
    block_data->light_level = level;
    update();
}

map::uniqueBlock& map::blockData::getUniqueBlock() const {
    return unique_blocks[(int)block_id];
}

map::uniqueLiquid& map::blockData::getUniqueLiquid() const {
    return unique_liquids[(int)liquid_id];
}

void map::renderBlocks() {
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
    
    // only request finite number of chunks per frame from server
#define REQUEST_LIMIT 5
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++) {
            if(getChunk(x, y).getState() == chunkState::unloaded && chunks_pending < REQUEST_LIMIT) {
                packets::packet packet(packets::CHUNK);
                packet << y << x;
                networking_manager->sendPacket(packet);
                getChunk(x, y).setState(chunkState::pending_load);
                chunks_pending++;
            } else if(getChunk(x, y).getState() == chunkState::loaded) {
                if(getChunk(x, y).hasToUpdate())
                    getChunk(x, y).updateTexture();
                getChunk(x, y).draw();
            }
        }
}

void map::block::updateOrientation() {
    if(!block_data->getUniqueBlock().single_texture) {
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

void map::block::draw() {
    gfx::rect rect((x & 15) * BLOCK_WIDTH, (y & 15) * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH, { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel()) });

    if(block_data->getUniqueBlock().texture.getTexture() && getLightLevel())
        gfx::render(block_data->getUniqueBlock().texture, rect.x, rect.y, gfx::rectShape(0, short((BLOCK_WIDTH >> 1) * block_data->orientation), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
    
    if(getLightLevel() != MAX_LIGHT)
        gfx::render(rect);

    if(getBreakStage())
        gfx::render(breaking_texture, rect.x, rect.y, gfx::rectShape(0, short(BLOCK_WIDTH / 2 * (getBreakStage() - 1)), BLOCK_WIDTH / 2, BLOCK_WIDTH / 2));

    if(getLiquidType() != liquidType::EMPTY) {
        int level = ((int)getLiquidLevel() + 1) / 16;
        gfx::render(block_data->getUniqueLiquid().texture, rect.x, rect.y + BLOCK_WIDTH - level * 2, gfx::rectShape(0, 0, BLOCK_WIDTH / 2, level));
    }
}

void map::block::scheduleTextureUpdate() {
    block_data->update = true;
    parent_map->getChunk(x >> 4, y >> 4).scheduleUpdate();
}

void map::block::update() {
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

void map::block::setBreakStage(unsigned char stage) {
    block_data->break_stage = stage;
    update();
}
