//
//  renderBlock.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 02/04/2021.
//

#include "core.hpp"

#include "playerHandler.hpp"
#include "networkingModule.hpp"
#include "blockRenderer.hpp"

blockRenderer::block& blockRenderer::getBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < blockEngine::world_height && x >= 0 && x < blockEngine::world_width, "requested block is out of bounds");
    return blocks[y * blockEngine::world_width + x];
}

void blockRenderer::updateBlockOrientation(unsigned short x, unsigned short y) {
    block& block = getBlock(x, y);
    if(!getUniqueBlock(x, y).single_texture) {
        block.orientation = 0;
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        unsigned char c = 1;
        for(int i = 0; i < 4; i++) {
            if(
               x + x_[i] >= blockEngine::world_width || x + x_[i] < 0 || y + y_[i] >= blockEngine::world_height || y + y_[i] < 0 ||
               blockEngine::getBlock(x + x_[i], y + y_[i]).block_id == blockEngine::getBlock(x, y).block_id ||
               std::count(getUniqueBlock(x, y).connects_to.begin(), getUniqueBlock(x, y).connects_to.end(), blockEngine::getBlock(x + x_[i], y + y_[i]).block_id)
            )
                block.orientation += c;
            c += c;
        }
    }
}

void blockRenderer::renderBlock(unsigned short x, unsigned short y) {
    block& block = getBlock(x, y);
    gfx::rect rect((x & 15) * BLOCK_WIDTH, (y & 15) * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * blockEngine::getBlock(x, y).light_level)});
    
    if(getUniqueBlock(x, y).texture.getTexture() && blockEngine::getBlock(x, y).light_level)
        gfx::render(getUniqueBlock(x, y).texture, rect.x, rect.y, gfx::rectShape(0, short((BLOCK_WIDTH >> 1) * block.orientation), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
    
    if(blockEngine::getBlock(x, y).light_level != MAX_LIGHT)
        gfx::render(rect);

    if(blockEngine::getBlock(x, y).break_progress)
        gfx::render(breaking_texture, rect.x, rect.y, gfx::rectShape(0, short(BLOCK_WIDTH * (blockEngine::getBlock(x, y).break_progress - 1)), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
}

void blockRenderer::uniqueBlock::loadFromUniqueBlock(blockEngine::uniqueBlock* unique_block) {
    texture.setTexture(unique_block->name == "air" ? nullptr : gfx::loadImageFile("texturePack/blocks/" + unique_block->name + ".png"));
    single_texture = texture.getTextureHeight() == 8;
    texture.scale = 2;
}

void blockRenderer::scheduleTextureUpdateForBlock(unsigned short x, unsigned short y) {
    getBlock(x, y).update = true;
    getChunk(x >> 4, y >> 4).update = true;
}

blockRenderer::uniqueBlock& blockRenderer::getUniqueBlock(unsigned short x, unsigned short y) {
    return unique_blocks[blockEngine::getBlock(x, y).block_id];
}

void blockRenderer::updateBlock(unsigned short x, unsigned short y) {
    scheduleTextureUpdateForBlock(x, y);
    
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
            scheduleTextureUpdateForBlock(neighbors[i].first, neighbors[i].second);
}
