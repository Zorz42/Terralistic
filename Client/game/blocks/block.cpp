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
    updateOrientation();
    if(x != 0)
        blocks->getBlock(x - 1, y).updateOrientation();
    if(x != blocks->getWidth() - 1)
        blocks->getBlock(x + 1, y).updateOrientation();
    if(y != 0)
        blocks->getBlock(x, y - 1).updateOrientation();
    if(y != blocks->getHeight() - 1)
        blocks->getBlock(x, y + 1).updateOrientation();
}

void ClientBlock::setLightLevel(unsigned char level) {
    block_data->light_level = level;
}

void ClientBlock::updateOrientation() {
    if(blocks->getResourcePack()->getTextureRectangle(getBlockType()).h != 8) {
        block_data->orientation = 0;
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        unsigned char c = 1;
        for(int i = 0; i < 4; i++) {
            if(
                    x + x_[i] >= blocks->getWidth() || x + x_[i] < 0 || y + y_[i] >= blocks->getHeight() || y + y_[i] < 0 ||
                    blocks->getBlock(x + x_[i], y + y_[i]).getBlockType() == getBlockType() ||
                    std::count(getBlockInfo().connects_to.begin(), getBlockInfo().connects_to.end(), blocks->getBlock(x + x_[i], y + y_[i]).getBlockType())
                )
                block_data->orientation += c;
            c += c;
        }
    }
    else
        block_data->orientation = 0;
}

void ClientBlock::setBreakStage(unsigned char stage) {
    block_data->break_stage = stage;
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
    sf::VertexArray block_vertex_array(sf::Quads, (getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()) * 4);
    
    int block_index = 0;
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getBlock(x, y).getOrientation() == 16)
                getBlock(x, y).updateOrientation();
            
            if(getBlock(x, y).getBlockType() != BlockType::AIR) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                float texture_y = resource_pack->getTextureRectangle(getBlock(x, y).getBlockType()).y + BLOCK_WIDTH * getBlock(x, y).getOrientation();
                float texture_x = (getBlock(x, y).getVariation()) % (resource_pack->getTextureRectangle(getBlock(x, y).getBlockType()).w / BLOCK_WIDTH) * BLOCK_WIDTH;
                
                block_vertex_array[block_index].texCoords = {texture_x, texture_y};
                block_vertex_array[block_index + 1].texCoords = {texture_x + (float)BLOCK_WIDTH, texture_y};
                block_vertex_array[block_index + 2].texCoords = {texture_x + (float)BLOCK_WIDTH, texture_y + BLOCK_WIDTH};
                block_vertex_array[block_index + 3].texCoords = {texture_x, texture_y + BLOCK_WIDTH};
                
                block_vertex_array[block_index].position = {(float)block_x, (float)block_y};
                block_vertex_array[block_index + 1].position = {(float)block_x + BLOCK_WIDTH * 2, (float)block_y};
                block_vertex_array[block_index + 2].position = {(float)block_x + BLOCK_WIDTH * 2, (float)block_y + BLOCK_WIDTH * 2};
                block_vertex_array[block_index + 3].position = {(float)block_x, (float)block_y + BLOCK_WIDTH * 2};
                
                block_index += 4;
            }
        }
    
    block_vertex_array.resize(block_index);
    if(block_index)
        gfx::drawVertices(block_vertex_array, resource_pack->getBlockTexture().getSfmlTexture()->getTexture());
    
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            if(getBlock(x, y).getBreakStage()) {
                int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
                getResourcePack()->getBreakingTexture().render(2, block_x, block_y, gfx::RectShape(0, short(BLOCK_WIDTH * (getBlock(x, y).getBreakStage() - 1)), BLOCK_WIDTH, BLOCK_WIDTH));
            }
        }
}

void ClientBlocks::renderFrontBlocks() {
    sf::VertexArray liquid_vertex_array(sf::Quads, (getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()) * 4);
    sf::VertexArray light_vertex_array(sf::Quads, (getViewEndX() - getViewBeginX()) * (getViewEndY() - getViewBeginY()) * 4);
    
    int light_index = 0, liquid_index = 0;
    for(unsigned short x = getViewBeginX(); x < getViewEndX(); x++)
        for(unsigned short y = getViewBeginY(); y < getViewEndY(); y++) {
            int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
            
            unsigned short low_x = x + 1 == getWidth() ? x : x + 1, low_y = y + 1 == getHeight() ? y : y + 1;
            unsigned char light_levels[] = {getBlock(x, y).getLightLevel(), getBlock(low_x, y).getLightLevel(), getBlock(low_x, low_y).getLightLevel(), getBlock(x, low_y).getLightLevel()};
            
            if(light_levels[0] != MAX_LIGHT || light_levels[1] != MAX_LIGHT || light_levels[2] != MAX_LIGHT || light_levels[3] != MAX_LIGHT) {
                light_vertex_array[light_index].color = { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(x, y).getLightLevel()) };
                light_vertex_array[light_index + 1].color = { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(low_x, y).getLightLevel()) };
                light_vertex_array[light_index + 2].color = { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(low_x, low_y).getLightLevel()) };
                light_vertex_array[light_index + 3].color = { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(x, low_y).getLightLevel()) };
                
                light_vertex_array[light_index].position = {(float)block_x + BLOCK_WIDTH, (float)block_y + BLOCK_WIDTH};
                light_vertex_array[light_index + 1].position = {(float)block_x + BLOCK_WIDTH * 3, (float)block_y + BLOCK_WIDTH};
                light_vertex_array[light_index + 2].position = {(float)block_x + BLOCK_WIDTH * 3, (float)block_y + BLOCK_WIDTH * 3};
                light_vertex_array[light_index + 3].position = {(float)block_x + BLOCK_WIDTH, (float)block_y + BLOCK_WIDTH * 3};
                
                light_index += 4;
            }

            if(getBlock(x, y).getLiquidType() != LiquidType::EMPTY) {
                float texture_y = resource_pack->getTextureRectangle(getBlock(x, y).getLiquidType()).y * 2,
                texture_height = getBlock(x, y).getLiquidLevel();
                
                liquid_vertex_array[liquid_index].texCoords = {0.f, texture_y};
                liquid_vertex_array[liquid_index + 1].texCoords = {(float)BLOCK_WIDTH, texture_y};
                liquid_vertex_array[liquid_index + 2].texCoords = {(float)BLOCK_WIDTH, texture_y + texture_height};
                liquid_vertex_array[liquid_index + 3].texCoords = {0.f, texture_y + texture_height};
                
                int level = ((int)getBlock(x, y).getLiquidLevel() + 1) / 8;
                liquid_vertex_array[liquid_index].position = {(float)block_x, (float)block_y + BLOCK_WIDTH * 2 - level};
                liquid_vertex_array[liquid_index + 1].position = {(float)block_x + BLOCK_WIDTH * 2, (float)block_y + BLOCK_WIDTH * 2 - level};
                liquid_vertex_array[liquid_index + 2].position = {(float)block_x + BLOCK_WIDTH * 2, (float)block_y + BLOCK_WIDTH * 2};
                liquid_vertex_array[liquid_index + 3].position = {(float)block_x, (float)block_y + BLOCK_WIDTH * 2};
                liquid_index += 4;
            }
        }
    
    liquid_vertex_array.resize(liquid_index);
    if(liquid_index)
        gfx::drawVertices(liquid_vertex_array, resource_pack->getLiquidTexture().getSfmlTexture()->getTexture());
    
    light_vertex_array.resize(light_index);
    if(light_index)
        gfx::drawVertices(light_vertex_array);
}
