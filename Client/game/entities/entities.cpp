#include <cassert>
#include <cmath>
#include "clientEntities.hpp"

void ClientEntities::init() {
    manager->packet_event.addListener(this);
}
void ClientEntities::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case PacketType::ENTITY_VELOCITY: {
            unsigned short id;
            float vel_x, vel_y;
            event.packet >> vel_x >> vel_y >> id;
            
            Entity* entity = getEntityById(id);
            setVelocityX(entity, vel_x);
            setVelocityY(entity, vel_y);
            break;
        }
        case PacketType::ENTITY_POSITION: {
            unsigned short id;
            int x, y;
            event.packet >> x >> y >> id;
            
            Entity* entity = getEntityById(id);
            setX(entity, x);
            setY(entity, y);
            break;
        }
        case PacketType::ENTITY_DELETION: {
            unsigned short id;
            event.packet >> id;
            
            Entity* entity = getEntityById(id);
            removeEntity(entity);
            break;
        }
        default:;
    }
}
