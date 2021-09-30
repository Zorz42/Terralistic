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
            
            Entity* entity = entities->getEntityById(id);
            entity->velocity_x = vel_x;
            entity->velocity_y = vel_y;
            break;
        }
        case PacketType::ENTITY_POSITION: {
            unsigned short id;
            int x, y;
            event.packet >> x >> y >> id;
            
            Entity* entity = entities->getEntityById(id);
            entity->x = x;
            entity->y = y;
            break;
        }
        default:;
    }
}
