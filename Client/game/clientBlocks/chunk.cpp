#include <cassert>
#include "clientBlocks.hpp"

void ClientBlocks::updateChunks() {
    short begin_x = view_x / (BLOCK_WIDTH * 32) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 32) - 1;
    short end_x = view_x / (BLOCK_WIDTH * 32) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 32) + 2;

    short begin_y = view_y / (BLOCK_WIDTH * 32) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 32) - 1;
    short end_y = view_y / (BLOCK_WIDTH * 32) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 32) + 2;

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
            }
        }
}

void ClientBlocks::renderBackChunks() {
    short begin_x = view_x / (BLOCK_WIDTH * 32) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 32) - 1;
    short end_x = view_x / (BLOCK_WIDTH * 32) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 32) + 2;

    short begin_y = view_y / (BLOCK_WIDTH * 32) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 32) - 1;
    short end_y = view_y / (BLOCK_WIDTH * 32) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 32) + 2;

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
            if(getChunk(x, y).getState() == ChunkState::loaded) {
                if(getChunk(x, y).hasToUpdate())
                    getChunk(x, y).updateTexture();
                getChunk(x, y).drawBack();
            }
        }
}

void ClientBlocks::renderFrontChunks() {
    short begin_x = view_x / (BLOCK_WIDTH * 32) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 32) - 1;
    short end_x = view_x / (BLOCK_WIDTH * 32) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 32) + 2;

    short begin_y = view_y / (BLOCK_WIDTH * 32) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 32) - 1;
    short end_y = view_y / (BLOCK_WIDTH * 32) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 32) + 2;

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

void ClientChunk::updateTexture() {
    chunk_data->update = false;
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            ClientBlock curr_block = parent_map->getBlock((x << 4) + x_, (y << 4) + y_);
            if(curr_block.hasToUpdateTexture())
                curr_block.updateTexture();
        }
    
    gfx::setRenderTarget(chunk_data->back_texture);
    chunk_data->back_texture.clear();
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            ClientBlock curr_block = parent_map->getBlock((x << 4) + x_, (y << 4) + y_);
            curr_block.drawBackInChunk();
        }
    gfx::resetRenderTarget();
    
    gfx::setRenderTarget(chunk_data->front_texture);
    chunk_data->front_texture.clear();
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            ClientBlock curr_block = parent_map->getBlock((x << 4) + x_, (y << 4) + y_);
            curr_block.drawFrontInChunk();
        }
    gfx::resetRenderTarget();
}

void ClientChunk::drawBack() {
    chunk_data->back_texture.render(1, (x * 16) * BLOCK_WIDTH * 2 - parent_map->view_x + gfx::getWindowWidth() / 2, (y << 4) * BLOCK_WIDTH * 2 - parent_map->view_y + gfx::getWindowHeight() / 2);
}

void ClientChunk::drawFront() {
    chunk_data->front_texture.render(1, (x * 16) * BLOCK_WIDTH * 2 - parent_map->view_x + gfx::getWindowWidth() / 2, (y << 4) * BLOCK_WIDTH * 2 - parent_map->view_y + gfx::getWindowHeight() / 2);
}

void ClientChunk::createTexture() {
    chunk_data->back_texture.createBlankImage(BLOCK_WIDTH * 32, BLOCK_WIDTH * 32);
    chunk_data->front_texture.createBlankImage(BLOCK_WIDTH * 32, BLOCK_WIDTH * 32);
}

ClientChunk ClientBlocks::getChunk(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < (getWorldHeight() >> 4) && x >= 0 && x < (getWorldWidth() >> 4));
    return {x, y, &chunks[y * (getWorldWidth() >> 4) + x], this};
}
