//
//  blockRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#include "core.hpp"

#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "blockEngineClient.hpp"

blockEngineClient::module::renderBlock& blockEngineClient::module::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < blockEngine::world_height && x >= 0 && x < blockEngine::world_width, "requested block is out of bounds");
    return blocks[y * blockEngine::world_width + x];
}

blockEngineClient::module::renderChunk& blockEngineClient::module::getChunk(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < (blockEngine::world_height >> 4) && x >= 0 && x < (blockEngine::world_width >> 4), "requested chunk is out of bounds");
    return chunks[y * (blockEngine::world_width >> 4) + x];
}

void blockEngineClient::module::updateRenderBlockOrientation(unsigned short x, unsigned short y) {
    renderBlock& block = getBlock(x, y);
    if(!getUniqueRenderBlock(x, y).single_texture) {
        block.block_orientation = 0;
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        unsigned char c = 1;
        for(int i = 0; i < 4; i++) {
            if(
               x + x_[i] >= blockEngine::world_width || x + x_[i] < 0 || y + y_[i] >= blockEngine::world_height || y + y_[i] < 0 ||
               blockEngine::getBlock(x + x_[i], y + y_[i]).block_id == blockEngine::getBlock(x, y).block_id ||
               std::count(getUniqueRenderBlock(x, y).connects_to.begin(), getUniqueRenderBlock(x, y).connects_to.end(), blockEngine::getBlock(x + x_[i], y + y_[i]).block_id)
            )
                block.block_orientation += c;
            c += c;
        }
    }
}

void blockEngineClient::module::drawRenderBlock(unsigned short x, unsigned short y) {
    renderBlock& block = getBlock(x, y);
    gfx::rect rect((x & 15) * BLOCK_WIDTH, (y & 15) * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * blockEngine::getBlock(x, y).light_level)});
    
    if(getUniqueRenderBlock(x, y).texture.getTexture() && blockEngine::getBlock(x, y).light_level)
        gfx::render(getUniqueRenderBlock(x, y).texture, rect.x, rect.y, gfx::rectShape(0, short((BLOCK_WIDTH >> 1) * block.block_orientation), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
    
    if(blockEngine::getBlock(x, y).light_level != MAX_LIGHT)
        gfx::render(rect);

    if(blockEngine::getBlock(x, y).break_progress)
        gfx::render(breaking_texture, rect.x, rect.y, gfx::rectShape(0, short(BLOCK_WIDTH * (blockEngine::getBlock(x, y).break_progress - 1)), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
}

void blockEngineClient::module::updateChunkTexture(unsigned short x, unsigned short y) {
    renderChunk& chunk = getChunk(x, y);
    chunk.update = false;
    gfx::setRenderTarget(chunk.texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            renderBlock& block = getBlock((x << 4) + x_, (y << 4) + y_);
            if(block.to_update) {
                updateRenderBlockOrientation((x << 4) + x_, (y << 4) + y_);
                gfx::render(gfx::rect(short(x_ * BLOCK_WIDTH), short(y_ * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH, {135, 206, 235}));
                drawRenderBlock((x << 4) + x_, (y << 4) + y_);
                block.to_update = false;
            }
        }
    gfx::resetRenderTarget();
}

void blockEngineClient::module::drawRenderChunk(unsigned short x, unsigned short y) {
    gfx::render(getChunk(x, y).texture, (x << 4) * BLOCK_WIDTH - playerHandler::view_x + (gfx::getWindowWidth() >> 1), (y << 4) * BLOCK_WIDTH - playerHandler::view_y + (gfx::getWindowHeight() >> 1));
}

void blockEngineClient::module::renderChunk::createTexture() {
    texture.setTexture(gfx::createBlankTexture(BLOCK_WIDTH << 4, BLOCK_WIDTH << 4));
}

void blockEngineClient::module::uniqueRenderBlock::createTexture(blockEngine::uniqueBlock* unique_block) {
    texture.setTexture(unique_block->name == "air" ? nullptr : gfx::loadImageFile("texturePack/blocks/" + unique_block->name + ".png"));
    single_texture = texture.getTextureHeight() == 8;
    texture.scale = 2;
}

void blockEngineClient::module::scheduleTextureUpdate(unsigned short x, unsigned short y) {
    getBlock(x, y).to_update = true;
    getChunk(x >> 4, y >> 4).update = true;
}

blockEngineClient::module::uniqueRenderBlock& blockEngineClient::module::getUniqueRenderBlock(unsigned short x, unsigned short y) {
    return unique_render_blocks[blockEngine::getBlock(x, y).block_id];
}

void blockEngineClient::module::updateBlock(unsigned short x, unsigned short y) {
    scheduleTextureUpdate(x, y);
    
    std::pair<short, short> neighbors[4] = {{-1, 0}, {-1, 0}, {-1, 0}, {-1, 0}};
    if(x != 0)
        neighbors[0] = {x - 1, y};
    if(x != blockEngine::world_width - 1)
        neighbors[1] = {x + 1, y};
    if(y != 0)
        neighbors[2] = {x, y - 1};
    if(y != blockEngine::world_height - 1)
        neighbors[3] = {x, y + 1};
    for(int i = 0; i < 4; i++)
        if(neighbors[i].first != -1)
            scheduleTextureUpdate(neighbors[i].first, neighbors[i].second);
}

/*EVENT_LISTENER(blockEngine::block_change)
    updateBlock(data.x, data.y);
EVENT_LISTENER_END

EVENT_LISTENER(blockEngine::light_change)
    updateBlock(data.x, data.y);
EVENT_LISTENER_END

EVENT_LISTENER(blockEngine::break_progress_change)
    updateBlock(data.x, data.y);
EVENT_LISTENER_END*/

// Module

void blockEngineClient::module::init() {
    unique_render_blocks = new uniqueRenderBlock[blockEngine::unique_blocks.size()];
    for(int i = 0; i < blockEngine::unique_blocks.size(); i++)
        unique_render_blocks[i].createTexture(&blockEngine::unique_blocks[i]);

    unique_render_blocks[blockEngine::GRASS_BLOCK].connects_to.push_back(blockEngine::DIRT);
    unique_render_blocks[blockEngine::DIRT].connects_to.push_back(blockEngine::GRASS_BLOCK);
    unique_render_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::GRASS_BLOCK);
    unique_render_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::LEAVES);

    breaking_texture.setTexture(gfx::loadImageFile("texturePack/misc/breaking.png"));
    breaking_texture.scale = 2;
    
    blockEngine::prepareWorld();
    
    chunks = new renderChunk[(blockEngine::world_width >> 4) * (blockEngine::world_height >> 4)];
    blocks = new renderBlock[blockEngine::world_width * blockEngine::world_height];
    
    for(unsigned short x = 0; x < (blockEngine::world_width >> 4); x++)
        for(unsigned short y = 0; y < (blockEngine::world_height >> 4); y++)
            getChunk(x, y).createTexture();
    
    listening_to = {};
}

void blockEngineClient::module::stop() {
    blockEngine::close();
    delete[] chunks;
    delete[] blocks;
}

void blockEngineClient::module::render() {
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
                drawRenderChunk(x, y);
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

void blockEngineClient::module::onPacket(packets::packet packet) {
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
