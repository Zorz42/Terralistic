#include "serverEntities.hpp"

void ServerEntities::init() {
    entity_position_change_event.addListener(this);
    entity_velocity_change_event.addListener(this);
    entity_deletion_event.addListener(this);
}

void ServerEntities::onEvent(EntityDeletionEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_DELETION << (unsigned short)event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::onEvent(EntityVelocityChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_VELOCITY << event.entity->getVelocityX() <<  event.entity->getVelocityY() << event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::onEvent(EntityPositionChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_POSITION << event.entity->getX() << event.entity->getY() << event.entity->id;
    networking->sendToEveryone(packet);
}

void ServerEntities::syncEntityPositions() {
    for(Entity* entity : getEntities()) {
        sf::Packet packet;
        packet << PacketType::ENTITY_POSITION << entity->getX() << entity->getY() << entity->id;
        networking->sendToEveryone(packet);
    }
}
