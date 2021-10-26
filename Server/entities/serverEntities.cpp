#include "serverEntities.hpp"
#include "graphics.hpp"

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
    sf::Packet packet;
    packet << ServerPacketType::ENTITY_DELETION << (unsigned short)event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::onEvent(EntityVelocityChangeEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::ENTITY_VELOCITY << event.entity->getVelocityX() <<  event.entity->getVelocityY() << event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::onEvent(EntityPositionChangeEvent& event) {
    sf::Packet packet;
    packet << ServerPacketType::ENTITY_POSITION << event.entity->getX() << event.entity->getY() << event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::update(float frame_length) {
    updateAllEntities(frame_length);
    
    if(gfx::getTicks() / 1000 > seconds) {
        seconds = gfx::getTicks() / 1000;
        for(Entity* entity : getEntities()) {
            sf::Packet packet;
            packet << ServerPacketType::ENTITY_POSITION << entity->getX() << entity->getY() << entity->id;
            networking->sendToEveryone(packet);
        }
    }
}
