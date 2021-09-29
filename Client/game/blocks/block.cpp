#include <algorithm>
#include <cassert>
#include "blockRenderer.hpp"
#include "resourcePack.hpp"

void BlockRenderer::updateState(unsigned short x, unsigned short y) {
    getClientBlock(x, y)->state = 0;
    for(auto& stateFunction : stateFunctions[(int)blocks->getBlockType(x, y)])
        stateFunction(blocks, this, x, y);
}

short BlockRenderer::getViewBeginX() const {
    return std::max(view_x / (BLOCK_WIDTH * 2) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) - 2, 0);
}

short BlockRenderer::getViewEndX() const {
    return std::min(view_x / (BLOCK_WIDTH * 2) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) + 2, (int)blocks->getWidth());
}

short BlockRenderer::getViewBeginY() const {
    return std::max(view_y / (BLOCK_WIDTH * 2) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) - 2, 0);
}

short BlockRenderer::getViewEndY() const {
    return std::min(view_y / (BLOCK_WIDTH * 2) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) + 2, (int)blocks->getHeight());
}

void BlockRenderer::setState(unsigned short x, unsigned short y, unsigned char state) {
    getClientBlock(x, y)->state = state;
}

unsigned char BlockRenderer::getState(unsigned short x, unsigned short y) {
    return getClientBlock(x, y)->state;
}

void BlockRenderer::renderBackBlocks() {
    gfx::RectArray block_rects((getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()));
    
    int block_index = 0;
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getState(x, y) == 16)
                updateState(x, y);
            
            if(blocks->getBlockType(x, y) != BlockType::AIR) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                int texture_x = (getClientBlock(x, y)->variation) % (resource_pack->getTextureRectangle(blocks->getBlockType(x, y)).w / BLOCK_WIDTH) * BLOCK_WIDTH;
                int texture_y = resource_pack->getTextureRectangle(blocks->getBlockType(x, y)).y + BLOCK_WIDTH * getClientBlock(x, y)->state;
                
                block_rects.setTextureCoords(block_index, {(short)texture_x, (short)texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
                block_rects.setRect(block_index, {(short)block_x, (short)block_y, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2});
                
                block_index++;
            }
        }
    
    block_rects.resize(block_index);
    block_rects.setImage(&resource_pack->getBlockTexture());
    
    if(block_index)
        block_rects.render();
    
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            if(blocks->getBreakStage(x, y)) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                getResourcePack()->getBreakingTexture().render(2, block_x, block_y, gfx::RectShape(0, short(BLOCK_WIDTH * (blocks->getBreakStage(x, y) - 1)), BLOCK_WIDTH, BLOCK_WIDTH));
            }
        }
}

void BlockRenderer::renderFrontBlocks() {
    gfx::RectArray liquid_rects((getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()));
    gfx::RectArray light_rects((getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()));
    
    int light_index = 0, liquid_index = 0;
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
            
            unsigned short low_x = x + 1 == blocks->getWidth() ? x : x + 1, low_y = y + 1 == blocks->getHeight() ? y : y + 1;
            unsigned char light_levels[] = {lights->getLightLevel(x, y), lights->getLightLevel(low_x, y), lights->getLightLevel(low_x, low_y), lights->getLightLevel(x, low_y)};
            
            if(light_levels[0] != MAX_LIGHT || light_levels[1] != MAX_LIGHT || light_levels[2] != MAX_LIGHT || light_levels[3] != MAX_LIGHT) {
                light_rects.setColor(light_index * 4, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * lights->getLightLevel(x, y))});
                light_rects.setColor(light_index * 4 + 1, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * lights->getLightLevel(low_x, y))});
                light_rects.setColor(light_index * 4 + 2, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * lights->getLightLevel(low_x, low_y))});
                light_rects.setColor(light_index * 4 + 3, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * lights->getLightLevel(x, low_y))});
                
                light_rects.setRect(light_index, {short(block_x + BLOCK_WIDTH), short(block_y + BLOCK_WIDTH), (short)BLOCK_WIDTH * 2, (short)BLOCK_WIDTH * 2});
                
                light_index++;
            }

            if(liquids->getLiquidType(x, y) != LiquidType::EMPTY) {
                int texture_y = resource_pack->getTextureRectangle(liquids->getLiquidType(x, y)).y * 2;
                
                liquid_rects.setTextureCoords(liquid_index, {0, (short)texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
                
                int level = ((int)liquids->getLiquidLevel(x, y) + 1) / 8;
                liquid_rects.setRect(liquid_index, {(short)block_x, short(block_y + BLOCK_WIDTH * 2 - level), (short)BLOCK_WIDTH * 2, (unsigned short)level});
                liquid_index++;
            }
        }
    
    liquid_rects.resize(liquid_index);
    liquid_rects.setImage(&resource_pack->getLiquidTexture());
    if(liquid_index)
        liquid_rects.render();
    
    light_rects.resize(light_index);
    if(light_index)
        light_rects.render();
}
