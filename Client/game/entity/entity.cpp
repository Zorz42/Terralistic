#include "clientEntity.hpp"

void ClientEntityManager::updateAllEntities() {
    for(ClientEntity* entity : entities) {
        
    }
}

void ClientEntityManager::addEntity(ClientEntity* entity) {
    entities.push_back(entity);
}
