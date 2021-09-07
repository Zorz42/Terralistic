#include "serverEntity.hpp"

void ServerEntityManager::updateAllEntities() {
    for(ServerEntity* entity : entities)
        entity->updateEntity(blocks);
}

void ServerEntityManager::addEntity(ServerEntity* entity) {
    entities.push_back(entity);
}

void ServerEntity::updateEntity(ServerBlocks* blocks) {
    
}

bool ServerEntity::isColliding(ServerBlocks* blocks) {
    if(x < 0 || y < 0 ||
       y >= blocks->getHeight() * BLOCK_WIDTH * 2 - getHeight() ||
       x >= blocks->getWidth() * BLOCK_WIDTH * 2 - getWidth())
        return true;

    unsigned short starting_x = x / (BLOCK_WIDTH * 2);
    unsigned short starting_y = y / (BLOCK_WIDTH * 2);
    unsigned short ending_x = (x + getWidth() - 1) / (BLOCK_WIDTH * 2);
    unsigned short ending_y = (y + getHeight() - 1) / (BLOCK_WIDTH * 2);
    
    for(unsigned short x_ = starting_x; x_ <= ending_x; x_++)
        for(unsigned short y_ = starting_y; y_ <= ending_y; y_++)
            if(!blocks->getBlock(x_, y_).getBlockInfo().ghost)
                return true;
    
    return false;
}
