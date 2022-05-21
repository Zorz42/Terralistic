#include "serverEntities.hpp"

void ServerEntities::init() {
    entity_position_change_event.addListener(this);
    entity_velocity_change_event.addListener(this);
    entity_deletion_event.addListener(this);
}

void ServerEntities::stop() {
    entity_position_change_event.removeListener(this);
    entity_velocity_change_event.removeListener(this);
    entity_deletion_event.removeListener(this);
}

void ServerEntities::onEvent(EntityDeletionEvent& event) {
    Packet packet;
    packet << ServerPacketType::ENTITY_DELETION << event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::onEvent(EntityVelocityChangeEvent& event) {
    Packet packet;
    packet << ServerPacketType::ENTITY_VELOCITY << event.entity->getVelocityX() <<  event.entity->getVelocityY() << event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::onEvent(EntityPositionChangeEvent& event) {
    Packet packet;
    packet << ServerPacketType::ENTITY_POSITION << event.entity->getX() << event.entity->getY() << event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::update(float frame_length) {
    updateAllEntities(frame_length);
    
    if(timer.getTimeElapsed() > 1000) {
        timer.reset();
        for(auto i : getEntities()) {
            Packet packet;
            packet << ServerPacketType::ENTITY_POSITION << i->getX() << i->getY() << i->id;
            networking->sendToEveryone(packet);
            
            packet.clear();
            packet << ServerPacketType::ENTITY_VELOCITY << i->getVelocityX() << i->getVelocityY() << i->id;
            networking->sendToEveryone(packet);
        }
    }
}
