#include <cmath>
#include "clientEntities.hpp"

void ClientEntities::init() {
    manager->packet_event.addListener(this);
}
void ClientEntities::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case ServerPacketType::ENTITY_VELOCITY: {
            int id;
            float vel_x, vel_y;
            event.packet >> vel_x >> vel_y >> id;
            
            Entity* entity = getEntityById(id);
            setVelocityX(entity, vel_x);
            setVelocityY(entity, vel_y);
            break;
        }
        case ServerPacketType::ENTITY_POSITION: {
            int id;
            int x, y;
            event.packet >> x >> y >> id;
            
            Entity* entity = getEntityById(id);
            setX(entity, x);
            setY(entity, y);
            break;
        }
        case ServerPacketType::ENTITY_DELETION: {
            int id;
            event.packet >> id;
            
            Entity* entity = getEntityById(id);
            removeEntity(entity);
            break;
        }
        default:;
    }
}

void ClientEntities::update(float frame_length) {
    updateAllEntities(frame_length);
}

void ClientEntities::stop() {
    manager->packet_event.removeListener(this);
}
