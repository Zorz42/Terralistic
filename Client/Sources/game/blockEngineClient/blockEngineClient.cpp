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

static gfx::image breaking_texture;
static blockEngineClient::renderChunk* chunks;
static blockEngineClient::renderBlock* blocks;
static blockEngineClient::uniqueRenderBlock* unique_render_blocks;

blockEngineClient::renderBlock& getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < blockEngine::world_height && x >= 0 && x < blockEngine::world_width, "requested block is out of bounds");
    return blocks[y * blockEngine::world_width + x];
}

blockEngineClient::renderChunk& getChunk(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < (blockEngine::world_height >> 4) && x >= 0 && x < (blockEngine::world_width >> 4), "requested chunk is out of bounds");
    return chunks[y * (blockEngine::world_width >> 4) + x];
}

INIT_SCRIPT
    INIT_ASSERT(!blockEngine::unique_blocks.empty());
    unique_render_blocks = new blockEngineClient::uniqueRenderBlock[blockEngine::unique_blocks.size()];
    for(int i = 0; i < blockEngine::unique_blocks.size(); i++)
        unique_render_blocks[i].createTexture(&blockEngine::unique_blocks[i]);

    unique_render_blocks[blockEngine::GRASS_BLOCK].connects_to.push_back(blockEngine::DIRT);
    unique_render_blocks[blockEngine::DIRT].connects_to.push_back(blockEngine::GRASS_BLOCK);
    unique_render_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::GRASS_BLOCK);
    unique_render_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::LEAVES);

    breaking_texture.setTexture(gfx::loadImageFile("texturePack/misc/breaking.png"));
    breaking_texture.scale = 2;
INIT_SCRIPT_END

void blockEngineClient::renderBlock::updateOrientation() {
    if(!getUniqueRenderBlock().single_texture) {
        block_orientation = 0;
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        unsigned char c = 1;
        for(int i = 0; i < 4; i++) {
            if(
               getX() + x_[i] >= blockEngine::world_width || getX() + x_[i] < 0 || getY() + y_[i] >= blockEngine::world_height || getY() + y_[i] < 0 ||
               blockEngine::getBlock(getX() + x_[i], getY() + y_[i]).block_id == getRelatedBlock().block_id ||
               std::count(getUniqueRenderBlock().connects_to.begin(), getUniqueRenderBlock().connects_to.end(), blockEngine::getBlock(getX() + x_[i], getY() + y_[i]).block_id)
            )
                block_orientation += c;
            c += c;
        }
    }
}

void blockEngineClient::renderBlock::draw() {
    gfx::rect rect((getX() & 15) * BLOCK_WIDTH, (getY() & 15) * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getRelatedBlock().light_level)});
    
    if(getUniqueRenderBlock().texture.getTexture() && getRelatedBlock().light_level)
        gfx::render(getUniqueRenderBlock().texture, rect.x, rect.y, gfx::rectShape(0, short((BLOCK_WIDTH >> 1) * block_orientation), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
    
    if(getRelatedBlock().light_level != MAX_LIGHT)
        gfx::render(rect);

    if(getRelatedBlock().break_progress)
        gfx::render(breaking_texture, rect.x, rect.y, gfx::rectShape(0, short(BLOCK_WIDTH * (getRelatedBlock().break_progress - 1)), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
}

void blockEngineClient::renderChunk::updateTexture() {
    update = false;
    gfx::setRenderTarget(texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++)
            if(getBlock((getX() << 4) + x_, (getY() << 4) + y_).to_update) {
                getBlock((getX() << 4) + x_, (getY() << 4) + y_).updateOrientation();
                gfx::rect rect(short(x_ * BLOCK_WIDTH), short(y_ * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH, {135, 206, 235});
                gfx::render(rect);
                getBlock((getX() << 4) + x_, (getY() << 4) + y_).draw();
                getBlock((getX() << 4) + x_, (getY() << 4) + y_).to_update = false;
            }
    gfx::resetRenderTarget();
}

void blockEngineClient::renderChunk::render() const {
    gfx::render(texture, (getX() << 4) * BLOCK_WIDTH - playerHandler::view_x + (gfx::getWindowWidth() >> 1), (getY() << 4) * BLOCK_WIDTH - playerHandler::view_y + (gfx::getWindowHeight() >> 1));
}

void blockEngineClient::renderChunk::createTexture() {
    texture.setTexture(gfx::createBlankTexture(BLOCK_WIDTH << 4, BLOCK_WIDTH << 4));
}

void blockEngineClient::uniqueRenderBlock::createTexture(blockEngine::uniqueBlock* unique_block) {
    texture.setTexture(unique_block->name == "air" ? nullptr : gfx::loadImageFile("texturePack/blocks/" + unique_block->name + ".png"));
    single_texture = texture.getTextureHeight() == 8;
    texture.scale = 2;
}

void blockEngineClient::renderBlock::scheduleTextureUpdate() {
    to_update = true;
    getChunk(getX() >> 4, getY() >> 4).update = true;
}

unsigned short blockEngineClient::renderBlock::getX() const {
    return (unsigned int)(this - blocks) % blockEngine::world_width;
}

unsigned short blockEngineClient::renderBlock::getY() const {
    return (unsigned int)(this - blocks) / blockEngine::world_width;
}

unsigned short blockEngineClient::renderChunk::getX() const {
    return (unsigned int)(this - chunks) % (blockEngine::world_width >> 4);
}

unsigned short blockEngineClient::renderChunk::getY() const {
    return (unsigned int)(this - chunks) / (blockEngine::world_width >> 4);
}

blockEngine::block& blockEngineClient::renderBlock::getRelatedBlock() {
    return blockEngine::getBlock(getX(), getY());
}

blockEngineClient::uniqueRenderBlock& blockEngineClient::renderBlock::getUniqueRenderBlock() {
    return unique_render_blocks[getRelatedBlock().block_id];
}

blockEngine::uniqueBlock& blockEngineClient::renderBlock::getUniqueBlock() {
    return getRelatedBlock().getUniqueBlock();
}

void updateBlock(unsigned short x, unsigned short y) {
    getBlock(x, y).scheduleTextureUpdate();
    
    blockEngineClient::renderBlock* neighbors[4] = {nullptr, nullptr, nullptr, nullptr};
    if(x != 0)
        neighbors[0] = &getBlock(x - 1, y);
    if(x != blockEngine::world_width - 1)
        neighbors[1] = &getBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = &getBlock(x, y - 1);
    if(y != blockEngine::world_height - 1)
        neighbors[3] = &getBlock(x, y + 1);
    for(int i = 0; i < 4; i++)
        if(neighbors[i] != nullptr)
            neighbors[i]->scheduleTextureUpdate();
}

EVENT_LISTENER(blockEngine::block_change)
    updateBlock(data.x, data.y);
EVENT_LISTENER_END

EVENT_LISTENER(blockEngine::light_change)
    updateBlock(data.x, data.y);
EVENT_LISTENER_END

EVENT_LISTENER(blockEngine::break_progress_change)
    updateBlock(data.x, data.y);
EVENT_LISTENER_END

// Module

void blockEngineClient::module::init() {
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
                    getChunk(x, y).updateTexture();
                getChunk(x, y).render();
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
