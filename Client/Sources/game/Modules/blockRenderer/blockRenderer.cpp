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
    unique_blocks = new uniqueBlock[map::unique_blocks.size()];
    for(int i = 0; i < map::unique_blocks.size(); i++)
        unique_blocks[i].loadFromUniqueBlock(&map::unique_blocks[i]);

    unique_blocks[(int)map::blockType::GRASS_BLOCK].connects_to.push_back(map::blockType::DIRT);
    unique_blocks[(int)map::blockType::DIRT].connects_to.push_back(map::blockType::GRASS_BLOCK);
    unique_blocks[(int)map::blockType::WOOD].connects_to.push_back(map::blockType::GRASS_BLOCK);
    unique_blocks[(int)map::blockType::WOOD].connects_to.push_back(map::blockType::LEAVES);

    breaking_texture.setTexture(gfx::loadImageFile("texturePack/misc/breaking.png"));
    breaking_texture.scale = 2;
    
    chunks = new chunk[(scene->world_map.getWorldWidth() >> 4) * (scene->world_map.getWorldHeight() >> 4)];
    blocks = new block[scene->world_map.getWorldWidth() * scene->world_map.getWorldHeight()];
    
    for(unsigned short x = 0; x < (scene->world_map.getWorldWidth() >> 4); x++)
        for(unsigned short y = 0; y < (scene->world_map.getWorldHeight() >> 4); y++)
            getChunk(x, y).createTexture();
    
    listening_to = {packets::BLOCK_CHANGE, packets::CHUNK, packets::BLOCK_PROGRESS_CHANGE};
    //events_listening_to = {map::block_change, scene->world_map.light_change, scene->world_map.getBreakStage()_change};
}

void blockRenderer::onEvent(events::eventType type, void* data) {
    /*if(type == map::block_change) {
        auto& data_ = *(map::block_change_data*)data;
        updateBlock(data_.x, data_.y);
    } else if(type == scene->world_map.light_change) {
        auto& data_ = *(scene->world_map.light_change_data*)data;
        updateBlock(data_.x, data_.y);
    } else if(type == scene->world_map.getBreakStage()_change) {
        auto& data_ = *(scene->world_map.getBreakStage()_change_data*)data;
        updateBlock(data_.x, data_.y);
    }*/
}

void blockRenderer::onPacket(packets::packet packet) {
    switch (packet.type) {
        case packets::BLOCK_CHANGE: {
            auto type = (map::blockType)packet.getUChar();
            unsigned short y = packet.getUShort(), x = packet.getUShort();
            scene->world_map.removeNaturalLight(x);
            scene->world_map.getBlock(x, y).setType(type);
            scene->world_map.setNaturalLight(x);
            scene->world_map.getBlock(x, y).lightUpdate();
            break;
        }
        case packets::CHUNK: {
            unsigned short x = packet.getUShort(), y = packet.getUShort();
            map::chunkState chunk_state = scene->world_map.getChunkState(x, y);
            for(unsigned short x_ = x << 4; x_ < (x << 4) + 16; x_++)
                scene->world_map.removeNaturalLight(x_);
            for(unsigned short y_ = 0; y_ < 16; y_++)
                for(unsigned short x_ = 0; x_ < 16; x_++) {
                    map::blockType type = (map::blockType)packet.getChar();
                    scene->world_map.getBlock((x << 4) + x_, (y << 4) + y_).setType(type);
                }
            for(unsigned short x_ = x << 4; x_ < (x << 4) + 16; x_++)
                scene->world_map.setNaturalLight(x_);
            chunk_state = map::chunkState::loaded;
            break;
        }
        case packets::BLOCK_PROGRESS_CHANGE: {
            unsigned short progress = packet.getUShort(), x = packet.getUShort(), y = packet.getUShort();
            scene->world_map.getBlock(x, y).setBreakProgress(progress);
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
    if(end_x > scene->world_map.getWorldWidth() >> 4)
        end_x = scene->world_map.getWorldWidth() >> 4;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > scene->world_map.getWorldHeight() >> 4)
        end_y = scene->world_map.getWorldHeight() >> 4;
    
    // only request one chunk per frame from server
    bool has_requested = false;
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++) {
            if(scene->world_map.getChunkState(x, y) == map::chunkState::unloaded && !has_requested) {
                packets::packet packet(packets::CHUNK);
                packet << y << x;
                scene->networking_manager.sendPacket(packet);
                scene->world_map.getChunkState(x, y) = map::chunkState::pending_load;
                has_requested = true;
            } else if(scene->world_map.getChunkState(x, y) == map::chunkState::loaded) {
                if(getChunk(x, y).update)
                    updateChunkTexture(x, y);
                renderChunk(x, y);
            }
        }
    begin_x <<= 4;
    begin_y <<= 4;
    end_x <<= 4;
    end_y <<= 4;
    for(unsigned short x = begin_x > MAX_LIGHT ? begin_x - MAX_LIGHT : 0; x < end_x + MAX_LIGHT && x < scene->world_map.getWorldWidth(); x++)
        for(unsigned short y = begin_y > MAX_LIGHT ? begin_y - MAX_LIGHT : 0; y < end_y + MAX_LIGHT && y < scene->world_map.getWorldHeight(); y++)
            if(scene->world_map.getBlock(x, y).hasScheduledLightUpdate() && scene->world_map.getChunkState(x >> 4, y >> 4) == map::chunkState::loaded)
                scene->world_map.getBlock(x, y).lightUpdate();
}

void blockRenderer::stop() {
    delete[] chunks;
    delete[] blocks;
}
