#include "entity.hpp"

void EntityManager::updateAllEntities() {
    for(Entity* entity : entities) {
        
    }
}

void EntityManager::addEntity(Entity* entity) {
    entities.push_back(entity);
}
