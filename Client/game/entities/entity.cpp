#include "clientEntity.hpp"

void ClientEntities::updateAllEntities() {
    /*for(ClientEntity* entity : entities) {
        
    }*/
}

const std::vector<ClientEntity*>& ClientEntities::getEntities() {
    return entities;
}

void ClientEntities::addEntity(ClientEntity* entity) {
    entities.push_back(entity);
}

ClientEntity* ClientEntities::getEntityById(unsigned short id) {
    for(ClientEntity* entity : entities)
        if(entity->id == id)
            return entity;
    assert(false);
    return nullptr;
}

void ClientEntities::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case PacketType::ENTITY_MOVEMENT: {
            unsigned short id;
            int x, y;
            event.packet >> x >> y >> id;
            
            ClientEntity* entity = getEntityById(id);
            entity->x = x;
            entity->y = y;
            break;
        }
        case PacketType::ENTITY_DELETION: {
            unsigned short id;
            event.packet >> id;
            for(auto i = entities.begin(); i != entities.end(); i++)
                if((*i)->id == id) {
                    entities.erase(i);
                    delete *i;
                    break;
                }
            break;
        }
        default:;
    }
}
