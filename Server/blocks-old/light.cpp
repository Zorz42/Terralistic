#include "serverBlocks-old.hpp"

void ServerBlock::lightUpdate() {
    block_data->update_light = false;
    
    ServerBlock neighbors[4];
    if(x != 0)
        neighbors[0] = blocks->getBlock(x - 1, y);
    if(x != blocks->getWidth() - 1)
        neighbors[1] = blocks->getBlock(x + 1, y);
    if(y != 0)
        neighbors[2] = blocks->getBlock(x, y - 1);
    if(y != blocks->getHeight() - 1)
        neighbors[3] = blocks->getBlock(x, y + 1);
    
    if(!block_data->light_source) {
        unsigned char level_to_be = 0;
        for(auto & neighbor : neighbors)
            if(neighbor.refersToABlock()) {
                unsigned char light_step = neighbor.getBlockInfo().transparent ? 3 : 15;
                unsigned char light = light_step > neighbor.getLightLevel() ? 0 : neighbor.getLightLevel() - light_step;
                if(light > level_to_be)
                    level_to_be = light;
            }
        setLightLevel(level_to_be);
    }
}

void ServerBlock::setLightSource(unsigned char power) {
    block_data->light_source = true;
    setLightLevel(power);
}

void ServerBlock::setLightLevel(unsigned char light_level) {
    if(block_data->light_level != light_level) {
        if(light_level == 0)
            block_data->light_source = false;
        block_data->light_level = light_level;
        updateNeighbors();
        
        ServerLightChangeEvent event(*this);
        blocks->light_change_event.call(event);
    }
}

void ServerBlocks::setLightLevelDirectly(unsigned short x, unsigned short y, unsigned char level) {
    getMapBlock(x, y)->light_level = level;
}

void ServerBlocks::setLightSourceDirectly(unsigned short x, unsigned short y, bool source) {
    getMapBlock(x, y)->light_source = source;
}
