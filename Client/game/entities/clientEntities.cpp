#include "clientEntities.hpp"

void ClientEntities::init() {
    networking->packet_event.addListener(this);
}
void ClientEntities::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case ServerPacketType::ENTITY_VELOCITY: {
            int id;
            float vel_x, vel_y;
            event.packet >> vel_x >> vel_y >> id;
            
            Entity* entity = getEntityById(id);
            if(!entity->ignore_server_updates) {
                setVelocityX(entity, vel_x);
                setVelocityY(entity, vel_y);
            }
            
            break;
        }
        case ServerPacketType::ENTITY_POSITION: {
            Packet event_packet = event.packet;
            int id;
            int x, y;
            event_packet >> x >> y >> id;
            
            Entity* entity = getEntityById(id);
            if(!entity->ignore_server_updates) {
                setX(entity, x);
                setY(entity, y);
            }
            break;
        }
        case ServerPacketType::ENTITY_DELETION: {
            Packet event_packet = event.packet;
            int id;
            event_packet >> id;
            
            Entity* entity = getEntityById(id);
            removeEntity(entity);
            break;
        }
        default:;
    }
}

void ClientEntities::updateParallel(float frame_length) {
    updateAllEntities(frame_length);
}

void ClientEntities::stop() {
    networking->packet_event.removeListener(this);
}
