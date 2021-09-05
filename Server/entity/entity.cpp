#include "serverEntity.hpp"

void ServerEntityManager::updateAllEntities() {
    for(ServerEntity* entity : entities) {
        
    }
}

void ServerEntityManager::addEntity(ServerEntity* entity) {
    entities.push_back(entity);
}
