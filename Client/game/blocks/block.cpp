#include <algorithm>
#include <cassert>
#include "clientBlocks.hpp"
#include "resourcePack.hpp"

ClientBlock ClientBlocks::getBlock(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < getHeight() && x >= 0 && x < getWidth());
    return {x, y, &blocks[y * getWidth() + x], this};
}

void ClientBlock::setType(BlockType block_id, LiquidType liquid_id) {
    block_data->block_id = block_id;
    block_data->liquid_id = liquid_id;
    updateState();
    if(x != 0)
        blocks->getBlock(x - 1, y).updateState();
    if(x != blocks->getWidth() - 1)
        blocks->getBlock(x + 1, y).updateState();
    if(y != 0)
        blocks->getBlock(x, y - 1).updateState();
    if(y != blocks->getHeight() - 1)
        blocks->getBlock(x, y + 1).updateState();
}

void ClientBlock::setLightLevel(unsigned char level) {
    block_data->light_level = level;
}

void ClientBlock::updateState() {
    block_data->state = 0;
    for(auto & stateFunction : blocks->stateFunctions[(int)getBlockType()])
        std::invoke(stateFunction, blocks, x, y);
}


void ClientBlock::setBreakStage(unsigned char stage) {
    block_data->break_stage = stage;
}

void ClientBlock::setState(unsigned char state) {
    block_data->state = state;
}

short ClientBlocks::getViewBeginX() const {
    return std::max(view_x / (BLOCK_WIDTH * 2) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) - 2, 0);
}

short ClientBlocks::getViewEndX() const {
    return std::min(view_x / (BLOCK_WIDTH * 2) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) + 2, (int)getWidth());
}

short ClientBlocks::getViewBeginY() const {
    return std::max(view_y / (BLOCK_WIDTH * 2) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) - 2, 0);
}

short ClientBlocks::getViewEndY() const {
    return std::min(view_y / (BLOCK_WIDTH * 2) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) + 2, (int)getHeight());
}

void ClientBlocks::renderBackBlocks() {
    gfx::RectArray block_rects((getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()));
    
    int block_index = 0;
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getBlock(x, y).getState() == 16)
                getBlock(x, y).updateState();
            
            if(getBlock(x, y).getBlockType() != BlockType::AIR) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                int texture_x = (getBlock(x, y).getVariation()) % (resource_pack->getTextureRectangle(getBlock(x, y).getBlockType()).w / BLOCK_WIDTH) * BLOCK_WIDTH;
                int texture_y = resource_pack->getTextureRectangle(getBlock(x, y).getBlockType()).y + BLOCK_WIDTH * getBlock(x, y).getState();
                
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
            if(getBlock(x, y).getBreakStage()) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                getResourcePack()->getBreakingTexture().render(2, block_x, block_y, gfx::RectShape(0, short(BLOCK_WIDTH * (getBlock(x, y).getBreakStage() - 1)), BLOCK_WIDTH, BLOCK_WIDTH));
            }
        }
}

void ClientBlocks::renderFrontBlocks() {
    gfx::RectArray liquid_rects((getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()));
    gfx::RectArray light_rects((getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()));
    
    int light_index = 0, liquid_index = 0;
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
            
            unsigned short low_x = x + 1 == getWidth() ? x : x + 1, low_y = y + 1 == getHeight() ? y : y + 1;
            unsigned char light_levels[] = {getBlock(x, y).getLightLevel(), getBlock(low_x, y).getLightLevel(), getBlock(low_x, low_y).getLightLevel(), getBlock(x, low_y).getLightLevel()};
            
            if(light_levels[0] != MAX_LIGHT || light_levels[1] != MAX_LIGHT || light_levels[2] != MAX_LIGHT || light_levels[3] != MAX_LIGHT) {
                light_rects.setColor(light_index * 4, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(x, y).getLightLevel())});
                light_rects.setColor(light_index * 4 + 1, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(low_x, y).getLightLevel())});
                light_rects.setColor(light_index * 4 + 2, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(low_x, low_y).getLightLevel())});
                light_rects.setColor(light_index * 4 + 3, {0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(x, low_y).getLightLevel())});
                
                light_rects.setRect(light_index, {short(block_x + BLOCK_WIDTH), short(block_y + BLOCK_WIDTH), (short)BLOCK_WIDTH * 2, (short)BLOCK_WIDTH * 2});
                
                light_index++;
            }

            if(getBlock(x, y).getLiquidType() != LiquidType::EMPTY) {
                int texture_y = resource_pack->getTextureRectangle(getBlock(x, y).getLiquidType()).y * 2;
                
                liquid_rects.setTextureCoords(liquid_index, {0, (short)texture_y, BLOCK_WIDTH, BLOCK_WIDTH});
                
                int level = ((int)getBlock(x, y).getLiquidLevel() + 1) / 8;
                liquid_rects.setRect(liquid_index, {(short)block_x, short(block_y + BLOCK_WIDTH * 2 - level), (short)BLOCK_WIDTH * 2, (unsigned short)level});
                liquid_index++;
            }
        }
    
    liquid_rects.resize(liquid_index);
    liquid_rects.setImage(&resource_pack->getLiquidTexture());
    if(liquid_index)
        liquid_rects.render();
    
    /*light_rects.resize(light_index);
    if(light_index)
        light_rects.render();*/
}
