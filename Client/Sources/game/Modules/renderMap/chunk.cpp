//
//  chunk.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/04/2021.
//

#include "renderMap.hpp"
#include "dev.hpp"

renderMap::renderChunk& renderMap::getRenderChunk(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < (getWorldHeight() >> 4) && x >= 0 && x < (getWorldWidth() >> 4), "requested chunk is out of bounds");
    return render_chunks[y * (getWorldWidth() >> 4) + x];
}

void renderMap::updateChunkTexture(unsigned short x, unsigned short y) {
    renderChunk& chunk = getRenderChunk(x, y);
    chunk.update = false;
    gfx::setRenderTarget(chunk.texture);
    for(unsigned short y_ = 0; y_ < 16; y_++)
        for(unsigned short x_ = 0; x_ < 16; x_++) {
            renderBlock& block = getRenderBlock((x << 4) + x_, (y << 4) + y_);
            if(block.update) {
                updateBlockOrientation((x << 4) + x_, (y << 4) + y_);
                gfx::render(gfx::rect(short(x_ * BLOCK_WIDTH), short(y_ * BLOCK_WIDTH), BLOCK_WIDTH, BLOCK_WIDTH, {135, 206, 235}));
                drawBlock((x << 4) + x_, (y << 4) + y_);
                block.update = false;
            }
        }
    gfx::resetRenderTarget();
}

void renderMap::drawChunk(unsigned short x, unsigned short y) {
    gfx::render(getRenderChunk(x, y).texture, (x << 4) * BLOCK_WIDTH - view_x + (gfx::getWindowWidth() >> 1), (y << 4) * BLOCK_WIDTH - view_y + (gfx::getWindowHeight() >> 1));
}

void renderMap::renderChunk::createTexture() {
    texture.setTexture(gfx::createBlankTexture(BLOCK_WIDTH << 4, BLOCK_WIDTH << 4));
}

