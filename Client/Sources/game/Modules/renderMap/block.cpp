//
//  block.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/04/2021.
//

#include "renderMap.hpp"
#include <algorithm>
#include "dev.hpp"

renderMap::renderBlock& renderMap::getRenderBlock(unsigned short x, unsigned short y) {
    ASSERT(y >= 0 && y < getWorldHeight() && x >= 0 && x < getWorldWidth(), "requested block is out of bounds");
    return render_blocks[y * getWorldWidth() + x];
}

void renderMap::updateBlockOrientation(unsigned short x, unsigned short y) {
    renderBlock& block = getRenderBlock(x, y);
    if(!getUniqueRenderBlock(x, y).single_texture) {
        block.orientation = 0;
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        unsigned char c = 1;
        for(int i = 0; i < 4; i++) {
            if(
               x + x_[i] >= getWorldWidth() || x + x_[i] < 0 || y + y_[i] >= getWorldHeight() || y + y_[i] < 0 ||
               getBlock(x + x_[i], y + y_[i]).getType() == getBlock(x, y).getType() ||
               std::count(getUniqueRenderBlock(x, y).connects_to.begin(), getUniqueRenderBlock(x, y).connects_to.end(), getBlock(x + x_[i], y + y_[i]).getType())
            )
                block.orientation += c;
            c += c;
        }
    }
}

void renderMap::drawBlock(unsigned short x, unsigned short y) {
    renderBlock& block = getRenderBlock(x, y);
    gfx::rect rect((x & 15) * BLOCK_WIDTH, (y & 15) * BLOCK_WIDTH, BLOCK_WIDTH, BLOCK_WIDTH, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(x, y).getLightLevel())});
    
    if(getUniqueRenderBlock(x, y).texture.getTexture() && getBlock(x, y).getLightLevel())
        gfx::render(getUniqueRenderBlock(x, y).texture, rect.x, rect.y, gfx::rectShape(0, short((BLOCK_WIDTH >> 1) * block.orientation), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
    
    if(getBlock(x, y).getLightLevel() != MAX_LIGHT)
        gfx::render(rect);

    if(getBlock(x, y).getBreakStage())
        gfx::render(breaking_texture, rect.x, rect.y, gfx::rectShape(0, short(BLOCK_WIDTH * (getBlock(x, y).getBreakStage() - 1)), BLOCK_WIDTH >> 1, BLOCK_WIDTH >> 1));
}

void renderMap::uniqueRenderBlock::loadFromUniqueBlock(map::uniqueBlock* unique_block) {
    texture.setTexture(unique_block->name == "air" ? nullptr : gfx::loadImageFile("texturePack/blocks/" + unique_block->name + ".png"));
    single_texture = texture.getTextureHeight() == 8;
    texture.scale = 2;
}

void renderMap::scheduleTextureUpdateForBlock(unsigned short x, unsigned short y) {
    getRenderBlock(x, y).update = true;
    getRenderChunk(x >> 4, y >> 4).update = true;
}

renderMap::uniqueRenderBlock& renderMap::getUniqueRenderBlock(unsigned short x, unsigned short y) {
    return unique_render_blocks[(int)getBlock(x, y).getType()];
}

void renderMap::updateBlock(unsigned short x, unsigned short y) {
    scheduleTextureUpdateForBlock(x, y);
    
    std::pair<short, short> neighbors[4] = {{-1, 0}, {-1, 0}, {-1, 0}, {-1, 0}};
    if(x != 0)
        neighbors[0] = {x - 1, y};
    if(x != getWorldWidth() - 1)
        neighbors[1] = {x + 1, y};
    if(y != 0)
        neighbors[2] = {x, y - 1};
    if(y != getWorldHeight() - 1)
        neighbors[3] = {x, y + 1};
    for(int i = 0; i < 4; i++)
        if(neighbors[i].first != -1)
            scheduleTextureUpdateForBlock(neighbors[i].first, neighbors[i].second);
}
