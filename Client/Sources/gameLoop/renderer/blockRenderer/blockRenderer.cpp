//
//  blockRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#define FILENAME blockRenderer
#define NAMESPACE renderer
#include "core.hpp"

#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "blockRenderer.hpp"

SDL_Texture* breaking_texture = nullptr;

INIT_SCRIPT
    INIT_ASSERT(blockEngine::unique_blocks.size());
    for(blockEngine::uniqueBlock& unique_block : blockEngine::unique_blocks)
        blockRenderer::unique_render_blocks.push_back(&unique_block);

    blockRenderer::unique_render_blocks[blockEngine::GRASS_BLOCK].connects_to.push_back(blockEngine::DIRT);
    blockRenderer::unique_render_blocks[blockEngine::DIRT].connects_to.push_back(blockEngine::GRASS_BLOCK);
    blockRenderer::unique_render_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::GRASS_BLOCK);
    blockRenderer::unique_render_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::LEAVES);

    breaking_texture = swl::loadTextureFromFile("texturePack/misc/breaking.png");
INIT_SCRIPT_END

void blockRenderer::prepare() {
    chunks = new renderChunk[(blockEngine::world_width >> 4) * (blockEngine::world_height >> 4)];
    blocks = new renderBlock[blockEngine::world_width * blockEngine::world_height];
    
    for(unsigned short x = 0; x < (blockEngine::world_width >> 4); x++)
        for(unsigned short y = 0; y < (blockEngine::world_height >> 4); y++)
            getChunk(x, y).createTexture();
}

void blockRenderer::close() {
    delete[] chunks;
    delete[] blocks;
}

void blockRenderer::renderBlock::updateOrientation(unsigned short x, unsigned short y) {
    if(!unique_render_blocks[blockEngine::getBlock(x, y).block_id].single_texture) {
        block_orientation = 0;
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        Uint8 c = 1;
        for(int i = 0; i < 4; i++) {
            if(x + x_[i] >= blockEngine::world_width || x + x_[i] < 0) {
                block_orientation += c;
                continue;
            }
            if(y + y_[i] >= blockEngine::world_height || y + y_[i] < 0) {
                block_orientation += c;
                continue;
            }
            if(blockEngine::getBlock(x + x_[i], y + y_[i]).block_id == blockEngine::getBlock(x, y).block_id || std::count(unique_render_blocks[blockEngine::getBlock(x, y).block_id].connects_to.begin(), unique_render_blocks[blockEngine::getBlock(x, y).block_id].connects_to.end(), blockEngine::getBlock(x + x_[i], y + y_[i]).block_id))
                block_orientation += c;
            c += c;
        }
    }
}

void blockRenderer::renderBlock::draw(unsigned short x, unsigned short y) {
    swl::rect rect = {short((x & 15) * BLOCK_WIDTH), short((y & 15) * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH};
    if(unique_render_blocks[blockEngine::getBlock(x, y).block_id].texture && blockEngine::getBlock(x, y).light_level)
        swl::render(unique_render_blocks[blockEngine::getBlock(x, y).block_id].texture, rect, {0, short(8 * block_orientation), 8, 8});
    
    if(blockEngine::getBlock(x, y).light_level != MAX_LIGHT) {
        swl::setDrawColor(0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * blockEngine::getBlock(x, y).light_level));
        swl::render(rect);
    }
    unsigned char progress = (unsigned char)((float)blockEngine::getBlock(x, y).break_progress / (float)blockEngine::getBlock(x, y).getUniqueBlock().break_time * 9.0f);
    if(progress)
        swl::render(breaking_texture, rect, {0, short(8 * (progress - 1)), 8, 8});
}

void blockRenderer::renderChunk::updateTexture(unsigned short x, unsigned short y) {
    update = false;
    swl::setRenderTarget(texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++)
            if(getBlock(x + x_, y + y_).to_update) {
                getBlock(x + x_, y + y_).updateOrientation(x + x_, y + y_);
                swl::rect rect = {short(x_ * BLOCK_WIDTH), short(y_ * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH};
                swl::setDrawColor(135, 206, 235);
                swl::render(rect);
                getBlock(x + x_, y + y_).draw(x + x_, y + y_);
                getBlock(x + x_, y + y_).to_update = false;
            }
    swl::resetRenderTarget();
}

void blockRenderer::render() {
    // figure out, what the window is covering and only render that
    unsigned short begin_x = playerHandler::view_x / BLOCK_WIDTH - swl::window_width / 2 / BLOCK_WIDTH;
    unsigned short end_x = playerHandler::view_x / BLOCK_WIDTH + swl::window_width / 2 / BLOCK_WIDTH;

    unsigned short begin_y = playerHandler::view_y / BLOCK_WIDTH - swl::window_height / 2 / BLOCK_WIDTH;
    unsigned short end_y = playerHandler::view_y / BLOCK_WIDTH + swl::window_height / 2 / BLOCK_WIDTH;
    
    if(begin_x < 0)
        begin_x = 0;
    if(end_x > blockEngine::world_width)
        end_x = (int)blockEngine::world_width;
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > blockEngine::world_height)
        end_y = (int)blockEngine::world_height;
    
    // only request one chunk per frame from server
    bool has_requested = false;
    
    for(unsigned short x = (begin_x >> 4) - 1; x < (end_x >> 4) + 1; x++)
        for(unsigned short y = (begin_y >> 4) - 1; y < (end_y >> 4) + 2; y++) {
            if(!blockEngine::getChunk(x, y).pending_load && !has_requested) {
                packets::packet packet(packets::CHUNK);
                packet << y << x;
                networking::sendPacket(packet);
                blockEngine::getChunk(x, y).pending_load = true;
                has_requested = true;
            } else if(blockEngine::getChunk(x, y).loaded) {
                if(blockRenderer::getChunk(x, y).update)
                    blockRenderer::getChunk(x, y).updateTexture(x << 4, y << 4);
                blockRenderer::getChunk(x, y).render(x, y);
            }
        }
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++)
            if(blockEngine::getBlock(x, y).to_update_light && blockEngine::getChunk(x >> 4, y >> 4).loaded)
                blockEngine::getBlock(x, y).light_update(x, y);
}

void blockRenderer::renderChunk::render(unsigned short x, unsigned short y) const {
    swl::rect rect = {short((x << 4) * BLOCK_WIDTH - playerHandler::view_x + swl::window_width / 2), short((y << 4) * BLOCK_WIDTH - playerHandler::view_y + swl::window_height / 2), BLOCK_WIDTH << 4, BLOCK_WIDTH << 4};
    swl::render(texture, rect);
}

void blockRenderer::renderChunk::createTexture() {
    texture = swl::createBlankTexture(BLOCK_WIDTH << 4, BLOCK_WIDTH << 4);
}

blockRenderer::renderBlock& blockRenderer::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < blockEngine::world_height && x >= 0 && x < blockEngine::world_width, "requested block is out of bounds");
    return blocks[x + y * blockEngine::world_width];
}

blockRenderer::renderChunk& blockRenderer::getChunk(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < (blockEngine::world_height >> 4) && x >= 0 && x < (blockEngine::world_width >> 4), "requested chunk is out of bounds");
    return chunks[x + y * (blockEngine::world_width >> 4)];
}

blockRenderer::uniqueRenderBlock::uniqueRenderBlock(blockEngine::uniqueBlock* unique_block) {
    unsigned short h = 0;
    texture = unique_block->name == "air" ? nullptr : swl::loadTextureFromFile("texturePack/blocks/" + unique_block->name + ".png", nullptr, &h);
    single_texture = h == 8;
}

EVENT_LISTENER(blockEngine::block_change)
    blockRenderer::getBlock(data.x, data.y).to_update = true;
    blockRenderer::getChunk(data.x >> 4, data.y >> 4).update = true;
EVENT_LISTENER_END

EVENT_LISTENER(blockEngine::light_change)
    blockRenderer::getBlock(data.x, data.y).to_update = true;
    blockRenderer::getChunk(data.x >> 4, data.y >> 4).update = true;
EVENT_LISTENER_END
