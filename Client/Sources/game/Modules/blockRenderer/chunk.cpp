//
//  renderChunk.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 02/04/2021.
//

#include "core.hpp"

#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "blockRenderer.hpp"

blockRenderer::chunk& blockRenderer::getChunk(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < (scene->world_map.getWorldHeight() >> 4) && x >= 0 && x < (scene->world_map.getWorldWidth() >> 4), "requested chunk is out of bounds");
    return chunks[y * (scene->world_map.getWorldWidth() >> 4) + x];
}

void blockRenderer::updateChunkTexture(unsigned short x, unsigned short y) {
    chunk& chunk = getChunk(x, y);
    chunk.update = false;
    gfx::setRenderTarget(chunk.texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            block& block = getBlock((x << 4) + x_, (y << 4) + y_);
            if(block.update) {
                updateBlockOrientation((x << 4) + x_, (y << 4) + y_);
                gfx::render(gfx::rect(short(x_ * BLOCK_WIDTH), short(y_ * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH, {135, 206, 235}));
                renderBlock((x << 4) + x_, (y << 4) + y_);
                block.update = false;
            }
        }
    gfx::resetRenderTarget();
}

void blockRenderer::renderChunk(unsigned short x, unsigned short y) {
    gfx::render(getChunk(x, y).texture, (x << 4) * BLOCK_WIDTH - playerHandler::view_x + (gfx::getWindowWidth() >> 1), (y << 4) * BLOCK_WIDTH - playerHandler::view_y + (gfx::getWindowHeight() >> 1));
}

void blockRenderer::chunk::createTexture() {
    texture.setTexture(gfx::createBlankTexture(BLOCK_WIDTH << 4, BLOCK_WIDTH << 4));
}
