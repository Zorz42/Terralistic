//
//  blockRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#include "core.hpp"

#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "blockRenderer.hpp"

void blockRenderer::init() {
    unique_blocks = new uniqueBlock[blockEngine::unique_blocks.size()];
    for(int i = 0; i < blockEngine::unique_blocks.size(); i++)
        unique_blocks[i].createTexture(&blockEngine::unique_blocks[i]);

    unique_blocks[blockEngine::GRASS_BLOCK].connects_to.push_back(blockEngine::DIRT);
    unique_blocks[blockEngine::DIRT].connects_to.push_back(blockEngine::GRASS_BLOCK);
    unique_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::GRASS_BLOCK);
    unique_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::LEAVES);

    breaking_texture.setTexture(gfx::loadImageFile("texturePack/misc/breaking.png"));
    breaking_texture.scale = 2;
    
    blockEngine::prepareWorld();
    
    chunks = new chunk[(blockEngine::world_width >> 4) * (blockEngine::world_height >> 4)];
    blocks = new block[blockEngine::world_width * blockEngine::world_height];
    
    for(unsigned short x = 0; x < (blockEngine::world_width >> 4); x++)
        for(unsigned short y = 0; y < (blockEngine::world_height >> 4); y++)
            getChunk(x, y).createTexture();
    
    listening_to = {packets::BLOCK_CHANGE, packets::CHUNK, packets::BLOCK_BREAK_PROGRESS_CHANGE};
    events_listening_to = {blockEngine::block_change, blockEngine::light_change, blockEngine::break_progress_change};
}

void blockRenderer::onEvent(events::eventType type, void* data) {
    if(type == blockEngine::block_change) {
        auto& data_ = *(blockEngine::block_change_data*)data;
        updateBlock(data_.x, data_.y);
    } else if(type == blockEngine::light_change) {
        auto& data_ = *(blockEngine::light_change_data*)data;
        updateBlock(data_.x, data_.y);
    } else if(type == blockEngine::break_progress_change) {
        auto& data_ = *(blockEngine::break_progress_change_data*)data;
        updateBlock(data_.x, data_.y);
    }
}

void blockRenderer::onPacket(packets::packet packet) {
    switch (packet.type) {
        case packets::BLOCK_CHANGE: {
            auto type = (blockEngine::blockType)packet.getUChar();
            unsigned short y = packet.getUShort(), x = packet.getUShort();
            blockEngine::removeNaturalLight(x);
            blockEngine::getBlock(x, y).setBlockType(type);
            blockEngine::setNaturalLight(x);
            blockEngine::getBlock(x, y).light_update();
            break;
        }
        case packets::CHUNK: {
            unsigned short x = packet.getUShort(), y = packet.getUShort();
            blockEngine::chunkState& chunk_state = blockEngine::getChunkState(x, y);
            for(unsigned short x_ = x << 4; x_ < (x << 4) + 16; x_++)
                blockEngine::removeNaturalLight(x_);
            for(unsigned short y_ = 0; y_ < 16; y_++)
                for(unsigned short x_ = 0; x_ < 16; x_++) {
                    blockEngine::blockType type = (blockEngine::blockType)packet.getChar();
                    blockEngine::getBlock((x << 4) + x_, (y << 4) + y_).setBlockType(type);
                }
            for(unsigned short x_ = x << 4; x_ < (x << 4) + 16; x_++)
                blockEngine::setNaturalLight(x_);
            chunk_state = blockEngine::loaded;
            break;
        }
        case packets::BLOCK_BREAK_PROGRESS_CHANGE: {
            unsigned short progress = packet.getUShort(), x = packet.getUShort(), y = packet.getUShort();
            blockEngine::getBlock(x, y).setBreakProgress(progress);
            break;
        }
        default:;
    }
}

void blockRenderer::render() {
    // figure out, what the window is covering and only render that
    short begin_x = playerHandler::view_x / (BLOCK_WIDTH << 4) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH << 4) - 1;
    short end_x = playerHandler::view_x / (BLOCK_WIDTH << 4) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH << 4) + 2;

    short begin_y = playerHandler::view_y / (BLOCK_WIDTH << 4) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH << 4) - 1;
    short end_y = playerHandler::view_y / (BLOCK_WIDTH << 4) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH << 4) + 2;
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > blockEngine::world_width >> 4)
        end_x = blockEngine::world_width >> 4;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > blockEngine::world_height >> 4)
        end_y = blockEngine::world_height >> 4;
    
    // only request one chunk per frame from server
    bool has_requested = false;
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++) {
            if(blockEngine::getChunkState(x, y) == blockEngine::unloaded && !has_requested) {
                packets::packet packet(packets::CHUNK);
                packet << y << x;
                scene->networking_manager.sendPacket(packet);
                blockEngine::getChunkState(x, y) = blockEngine::pending_load;
                has_requested = true;
            } else if(blockEngine::getChunkState(x, y) == blockEngine::loaded) {
                if(getChunk(x, y).update)
                    updateChunkTexture(x, y);
                renderChunk(x, y);
            }
        }
    begin_x <<= 4;
    begin_y <<= 4;
    end_x <<= 4;
    end_y <<= 4;
    for(unsigned short x = begin_x > MAX_LIGHT ? begin_x - MAX_LIGHT : 0; x < end_x + MAX_LIGHT && x < blockEngine::world_width; x++)
        for(unsigned short y = begin_y > MAX_LIGHT ? begin_y - MAX_LIGHT : 0; y < end_y + MAX_LIGHT && y < blockEngine::world_height; y++)
            if(blockEngine::getBlock(x, y).to_update_light && blockEngine::getChunkState(x >> 4, y >> 4) == blockEngine::loaded)
                blockEngine::getBlock(x, y).light_update();
}

void blockRenderer::stop() {
    blockEngine::close();
    delete[] chunks;
    delete[] blocks;
}
