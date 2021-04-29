//
//  renderMap.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/04/2021.
//

#include "renderMap.hpp"

gfx::image renderMap::breaking_texture;
renderMap::uniqueRenderBlock* renderMap::unique_render_blocks;

void renderMap::loadBlocks() {
    unique_render_blocks = new uniqueRenderBlock[map::unique_blocks.size()];
    for(int i = 0; i < map::unique_blocks.size(); i++)
        unique_render_blocks[i].loadFromUniqueBlock(&map::unique_blocks[i]);

    unique_render_blocks[(int)map::blockType::GRASS_BLOCK].connects_to.push_back(map::blockType::DIRT);
    unique_render_blocks[(int)map::blockType::DIRT].connects_to.push_back(map::blockType::GRASS_BLOCK);
    unique_render_blocks[(int)map::blockType::WOOD].connects_to.push_back(map::blockType::GRASS_BLOCK);
    unique_render_blocks[(int)map::blockType::WOOD].connects_to.push_back(map::blockType::LEAVES);

    breaking_texture.setTexture(gfx::loadImageFile("texturePack/misc/breaking.png"));
    breaking_texture.scale = 2;
}

void renderMap::init() {
    render_chunks = new renderChunk[(getWorldWidth() >> 4) * (getWorldHeight() >> 4)];
    render_blocks = new renderBlock[getWorldWidth() * getWorldHeight()];
    
    for(unsigned short x = 0; x < (getWorldWidth() >> 4); x++)
        for(unsigned short y = 0; y < (getWorldHeight() >> 4); y++)
            getRenderChunk(x, y).createTexture();
}

void renderMap::renderBlocks() {
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
    
    // only request one chunk per frame from server
    bool has_requested = false;
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++) {
            if(getChunkState(x, y) == map::chunkState::unloaded && !has_requested) {
                packets::packet packet(packets::CHUNK);
                packet << y << x;
                networking_manager->sendPacket(packet);
                getChunkState(x, y) = map::chunkState::pending_load;
                has_requested = true;
            } else if(getChunkState(x, y) == map::chunkState::loaded) {
                if(getRenderChunk(x, y).update)
                    updateChunkTexture(x, y);
                drawChunk(x, y);
            }
        }
    begin_x <<= 4;
    begin_y <<= 4;
    end_x <<= 4;
    end_y <<= 4;
    for(unsigned short x = begin_x > MAX_LIGHT ? begin_x - MAX_LIGHT : 0; x < end_x + MAX_LIGHT && x < getWorldWidth(); x++)
        for(unsigned short y = begin_y > MAX_LIGHT ? begin_y - MAX_LIGHT : 0; y < end_y + MAX_LIGHT && y < getWorldHeight(); y++)
            if(getBlock(x, y).hasScheduledLightUpdate() && getChunkState(x >> 4, y >> 4) == map::chunkState::loaded)
                getBlock(x, y).lightUpdate();
}

void renderMap::stop() {
    delete[] render_chunks;
    delete[] render_blocks;
}
