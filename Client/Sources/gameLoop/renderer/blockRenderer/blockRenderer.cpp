//
//  blockRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#define FILENAME blockRenderer
#define NAMESPACE renderer
#include "core.hpp"

#include "renderer.hpp"
#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "blockRenderer.hpp"

SDL_Texture* breaking_texture = nullptr;

void draw(blockEngine::block* self, unsigned short x, unsigned short y) {
    swl::rect rect = {short(x * BLOCK_WIDTH), short(y * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH};
    if(self->getUniqueBlock().texture && blockEngine::getBlock(x, y).light_level)
        swl::render(self->getUniqueBlock().texture, rect, {0, short(8 * self->block_orientation), 8, 8});
    
    if(self->light_level != MAX_LIGHT) {
        swl::setDrawColor(0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * self->light_level));
        swl::render(rect);
    }
    unsigned char progress = (unsigned char)((float)self->break_progress / (float)self->getUniqueBlock().break_time * 9.0f);
    if(progress)
        swl::render(breaking_texture, rect, {0, short(8 * (progress - 1)), 8, 8});
}

void updateTexture(blockEngine::chunk* self) {
    self->update = false;
    swl::setRenderTarget(self->texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++)
            if(self->blocks[x_][y_].to_update) {
                swl::rect rect = {short(x_ * BLOCK_WIDTH), short(y_ * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH};
                swl::setDrawColor(135, 206, 235);
                swl::render(rect);
                draw(&self->blocks[x_][y_], x_, y_);
                self->blocks[x_][y_].to_update = false;
            }
    swl::resetRenderTarget();
}

INIT_SCRIPT
INIT_ASSERT(blockEngine::unique_blocks.size());
    blockEngine::unique_blocks[blockEngine::GRASS_BLOCK].connects_to.push_back(blockEngine::DIRT);
    blockEngine::unique_blocks[blockEngine::DIRT].connects_to.push_back(blockEngine::GRASS_BLOCK);
    blockEngine::unique_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::GRASS_BLOCK);
    blockEngine::unique_blocks[blockEngine::WOOD].connects_to.push_back(blockEngine::LEAVES);

    breaking_texture = swl::loadTextureFromFile("texturePack/misc/breaking.png");
INIT_SCRIPT_END

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
                if(blockEngine::getChunk(x, y).update)
                    updateTexture(&blockEngine::getChunk(x, y));
                blockEngine::getChunk(x, y).render(x, y);
            }
        }
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++)
            if(blockEngine::getBlock(x, y).to_update_light && blockEngine::getChunk(x >> 4, y >> 4).loaded)
                blockEngine::getBlock(x, y).light_update(x, y);
}
