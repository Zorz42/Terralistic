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
    ASSERT(y >= 0 && y < scene->world_map.getWorldHeight() && x >= 0 && x < scene->world_map.getWorldWidth(), "requested block is out of bounds");
    return blocks[y * scene->world_map.getWorldWidth() + x];
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
               x + x_[i] >= scene->world_map.getWorldWidth() || x + x_[i] < 0 || y + y_[i] >= scene->world_map.getWorldHeight() || y + y_[i] < 0 ||
               scene->world_map.getBlock(x + x_[i], y + y_[i]).getType() == scene->world_map.getBlock(x, y).getType() ||
               std::count(getUniqueBlock(x, y).connects_to.begin(), getUniqueBlock(x, y).connects_to.end(), scene->world_map.getBlock(x + x_[i], y + y_[i]).getType())
            )
                block.orientation += c;
            c += c;
        }
    }
}

void blockRenderer::renderBlock(unsigned short x, unsigned short y) {
    block& block = getBlock(x, y);
    gfx::rect rect((x & 15) * BLOCK_WIDTH, (y & 15) * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * scene->world_map.getBlock(x, y).getLightLevel())});
    
    if(getUniqueBlock(x, y).texture.getTexture() && scene->world_map.getBlock(x, y).getLightLevel())
        gfx::render(getUniqueBlock(x, y).texture, rect.x, rect.y, gfx::rectShape(0, short((BLOCK_WIDTH >> 1) * block.orientation), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
    
    if(scene->world_map.getBlock(x, y).getLightLevel() != MAX_LIGHT)
        gfx::render(rect);

    if(scene->world_map.getBlock(x, y).getBreakStage())
        gfx::render(breaking_texture, rect.x, rect.y, gfx::rectShape(0, short(BLOCK_WIDTH * (scene->world_map.getBlock(x, y).getBreakStage() - 1)), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
}

void blockRenderer::uniqueBlock::loadFromUniqueBlock(map::uniqueBlock* unique_block) {
    texture.setTexture(unique_block->name == "air" ? nullptr : gfx::loadImageFile("texturePack/blocks/" + unique_block->name + ".png"));
    single_texture = texture.getTextureHeight() == 8;
    texture.scale = 2;
}

void blockRenderer::scheduleTextureUpdateForBlock(unsigned short x, unsigned short y) {
    getBlock(x, y).update = true;
    getChunk(x >> 4, y >> 4).update = true;
}

blockRenderer::uniqueBlock& blockRenderer::getUniqueBlock(unsigned short x, unsigned short y) {
    return unique_blocks[(int)scene->world_map.getBlock(x, y).getType()];
}

void blockRenderer::updateBlock(unsigned short x, unsigned short y) {
    scheduleTextureUpdateForBlock(x, y);
    
    std::pair<short, short> neighbors[4] = {{-1, 0}, {-1, 0}, {-1, 0}, {-1, 0}};
    if(x != 0)
        neighbors[0] = {x - 1, y};
    if(x != scene->world_map.getWorldWidth() - 1)
        neighbors[1] = {x + 1, y};
    if(y != 0)
        neighbors[2] = {x, y - 1};
    if(y != scene->world_map.getWorldHeight() - 1)
        neighbors[3] = {x, y + 1};
    for(int i = 0; i < 4; i++)
        if(neighbors[i].first != -1)
            scheduleTextureUpdateForBlock(neighbors[i].first, neighbors[i].second);
}
