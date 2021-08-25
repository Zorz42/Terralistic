#include <algorithm>
#include <cassert>
#include "clientBlocks.hpp"
#include "print.hpp"
#include "resourcePack.hpp"

ClientBlock ClientBlocks::getBlock(unsigned short x, unsigned short y) {
    assert(y >= 0 && y < getWorldHeight() && x >= 0 && x < getWorldWidth());
    return {x, y, &blocks[y * getWorldWidth() + x], this};
}

void ClientBlock::setType(BlockType block_id, LiquidType liquid_id) {
    block_data->block_id = block_id;
    block_data->liquid_id = liquid_id;
    scheduleTextureUpdate();
    scheduleTextureUpdateForNeighbors();
}

void ClientBlock::setLightLevel(unsigned char level) {
    block_data->light_level = level;
    scheduleTextureUpdate();
}

void ClientBlock::updateTexture() {
    if(parent_map->getResourcePack()->getTextureRectangle(getBlockType()).h != 8) {
        block_data->orientation = 0;
        char x_[] = {0, 1, 0, -1};
        char y_[] = {-1, 0, 1, 0};
        unsigned char c = 1;
        for(int i = 0; i < 4; i++) {
            if(
                    x + x_[i] >= parent_map->getWorldWidth() || x + x_[i] < 0 || y + y_[i] >= parent_map->getWorldHeight() || y + y_[i] < 0 ||
                    parent_map->getBlock(x + x_[i], y + y_[i]).getBlockType() == getBlockType() ||
                    std::count(getBlockInfo().connects_to.begin(), getBlockInfo().connects_to.end(), parent_map->getBlock(x + x_[i], y + y_[i]).getBlockType())
                    )
                block_data->orientation += c;
            c += c;
        }
    }
    else
        block_data->orientation = 0;
    block_data->update = false;
}

void ClientBlock::drawBackInChunk() {
    int block_x = (x & 15) * BLOCK_WIDTH * 2, block_y = (y & 15) * BLOCK_WIDTH * 2;

    if(getLightLevel())
        parent_map->getResourcePack()->getBlockTexture().render(2, block_x, block_y, gfx::RectShape(0, short(parent_map->getResourcePack()->getTextureRectangle(getBlockType()).y + BLOCK_WIDTH * block_data->orientation), BLOCK_WIDTH, BLOCK_WIDTH));

    if(getBreakStage())
        parent_map->getResourcePack()->getBreakingTexture().render(2, block_x, block_y, gfx::RectShape(0, short(BLOCK_WIDTH * (getBreakStage() - 1)), BLOCK_WIDTH, BLOCK_WIDTH));
}

void ClientBlock::drawFrontInChunk() {
    gfx::Rect rect(x % 16 * BLOCK_WIDTH * 2, y % 16 * BLOCK_WIDTH * 2, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2, { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel()) });

    if(getLiquidType() != LiquidType::EMPTY) {
        int level = ((int)getLiquidLevel() + 1) / 16;
        parent_map->getResourcePack()->getLiquidTexture(getLiquidType()).render(2, rect.getX(), rect.getY() + BLOCK_WIDTH * 2 - level * 2, gfx::RectShape(0, 0, BLOCK_WIDTH, level));
    }
    
    if(getLightLevel() != MAX_LIGHT)
        rect.render();
}

void ClientBlock::drawBack() {
    
    //if(getLightLevel())
        //parent_map->getResourcePack()->getBlockTexture().render(2, block_x, block_y, gfx::RectShape(0, short(parent_map->getResourcePack()->getTextureRectangle(getBlockType()).y + BLOCK_WIDTH * block_data->orientation), BLOCK_WIDTH, BLOCK_WIDTH));
    
    if(getBreakStage()) {
        int block_x = x * BLOCK_WIDTH * 2 - parent_map->view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - parent_map->view_y + gfx::getWindowHeight() / 2;
        parent_map->getResourcePack()->getBreakingTexture().render(2, block_x, block_y, gfx::RectShape(0, short(BLOCK_WIDTH * (getBreakStage() - 1)), BLOCK_WIDTH, BLOCK_WIDTH));
    }
}

void ClientBlock::drawFront() {
    //gfx::Rect rect(x * BLOCK_WIDTH * 2 - parent_map->view_x + gfx::getWindowWidth() / 2, y * BLOCK_WIDTH * 2 - parent_map->view_y + gfx::getWindowHeight() / 2, BLOCK_WIDTH * 2, BLOCK_WIDTH * 2, { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getLightLevel()) });

    //if(getLiquidType() != LiquidType::EMPTY) {
        //int level = ((int)getLiquidLevel() + 1) / 16;
        //parent_map->getResourcePack()->getLiquidTexture(getLiquidType()).render(2, rect.getX(), rect.getY() + BLOCK_WIDTH * 2 - level * 2, gfx::RectShape(0, 0, BLOCK_WIDTH, level));
    //}
    
    //if(getLightLevel() != MAX_LIGHT)
        //rect.render();
}

void ClientBlock::scheduleTextureUpdate() {
    block_data->update = true;
    parent_map->getChunk(x >> 4, y >> 4).scheduleUpdate();
}

void ClientBlock::scheduleTextureUpdateForNeighbors() {
    if(x != 0)
        parent_map->getBlock(x - 1, y).scheduleTextureUpdate();
    if(x != parent_map->getWorldWidth() - 1)
        parent_map->getBlock(x + 1, y).scheduleTextureUpdate();
    if(y != 0)
        parent_map->getBlock(x, y - 1).scheduleTextureUpdate();
    if(y != parent_map->getWorldHeight() - 1)
        parent_map->getBlock(x, y + 1).scheduleTextureUpdate();
}

void ClientBlock::setBreakStage(unsigned char stage) {
    block_data->break_stage = stage;
    scheduleTextureUpdate();
}

void ClientBlocks::renderBackBlocks() {
    short begin_x = view_x / (BLOCK_WIDTH * 2) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) - 1;
    short end_x = view_x / (BLOCK_WIDTH * 2) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) + 2;

    short begin_y = view_y / (BLOCK_WIDTH * 2) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) - 1;
    short end_y = view_y / (BLOCK_WIDTH * 2) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) + 2;

    if(begin_x < 0)
        begin_x = 0;
    if(end_x > getWorldWidth())
        end_x = getWorldWidth();
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > getWorldHeight())
        end_y = getWorldHeight();

    sf::VertexArray vertex_array(sf::Quads, (end_x - begin_x) * (end_y - begin_y) * 4);
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++) {
            if(getBlock(x, y).hasToUpdateTexture())
                getBlock(x, y).updateTexture();
            getBlock(x, y).drawBack();
            
            int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
            int index = ((x - begin_x) * (end_y - begin_y) + (y - begin_y)) * 4;
            
            vertex_array[index].position = {(float)block_x, (float)block_y};
            vertex_array[index + 1].position = {(float)block_x + BLOCK_WIDTH * 2, (float)block_y};
            vertex_array[index + 2].position = {(float)block_x + BLOCK_WIDTH * 2, (float)block_y + BLOCK_WIDTH * 2};
            vertex_array[index + 3].position = {(float)block_x, (float)block_y + BLOCK_WIDTH * 2};
            
            float texture_y = resource_pack->getTextureRectangle(getBlock(x, y).getBlockType()).y + BLOCK_WIDTH * getBlock(x, y).getOrientation();
            
            vertex_array[index].texCoords = {0.f, texture_y};
            vertex_array[index + 1].texCoords = {(float)BLOCK_WIDTH, texture_y};
            vertex_array[index + 2].texCoords = {(float)BLOCK_WIDTH, texture_y + BLOCK_WIDTH};
            vertex_array[index + 3].texCoords = {0.f, texture_y + BLOCK_WIDTH};
        }
    
    gfx::drawVertices(vertex_array, resource_pack->getBlockTexture().getSfmlTexture()->getTexture());
}

void ClientBlocks::renderFrontBlocks() {
    short begin_x = view_x / (BLOCK_WIDTH * 2) - gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) - 1;
    short end_x = view_x / (BLOCK_WIDTH * 2) + gfx::getWindowWidth() / 2 / (BLOCK_WIDTH * 2) + 2;

    short begin_y = view_y / (BLOCK_WIDTH * 2) - gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) - 1;
    short end_y = view_y / (BLOCK_WIDTH * 2) + gfx::getWindowHeight() / 2 / (BLOCK_WIDTH * 2) + 2;

    if(begin_x < 0)
        begin_x = 0;
    if(end_x > getWorldWidth())
        end_x = getWorldWidth();
    if(begin_y < 0)
        begin_y = 0;
    if(end_y > getWorldHeight())
        end_y = getWorldHeight();


    sf::VertexArray vertex_array(sf::Quads, (end_x - begin_x) * (end_y - begin_y) * 4);
    
    for(unsigned short x = begin_x; x < end_x; x++)
        for(unsigned short y = begin_y; y < end_y; y++) {
            //getBlock(x, y).drawFront();
            
            int block_x = x * BLOCK_WIDTH * 2 - view_x + gfx::getWindowWidth() / 2, block_y = y * BLOCK_WIDTH * 2 - view_y + gfx::getWindowHeight() / 2;
            int index = ((x - begin_x) * (end_y - begin_y) + (y - begin_y)) * 4;
            
            sf::Color light_color = { 0, 0, 0, (unsigned char)(255 - 255.0 / MAX_LIGHT * getBlock(x, y).getLightLevel()) };
            
            vertex_array[index].position = {(float)block_x, (float)block_y};
            vertex_array[index + 1].position = {(float)block_x + BLOCK_WIDTH * 2, (float)block_y};
            vertex_array[index + 2].position = {(float)block_x + BLOCK_WIDTH * 2, (float)block_y + BLOCK_WIDTH * 2};
            vertex_array[index + 3].position = {(float)block_x, (float)block_y + BLOCK_WIDTH * 2};
            
            vertex_array[index].color = light_color;
            vertex_array[index + 1].color = light_color;
            vertex_array[index + 2].color = light_color;
            vertex_array[index + 3].color = light_color;
        }
    
    gfx::drawVertices(vertex_array);
}
