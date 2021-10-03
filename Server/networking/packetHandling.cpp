#include "serverNetworking.hpp"
#include "print.hpp"

/*

void ServerNetworkingManager::onEvent(ItemCreationEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_CREATION << (int)event.item->getX() << (int)event.item->getY() << event.item->id << (unsigned char)event.item->getType();
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(EntityDeletionEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_DELETION << (unsigned short)event.entity->id;
    sendToEveryone(packet);
}

void ServerNetworkingManager::onEvent(EntityVelocityChangeEvent& event) {
    Connection* exclusion = nullptr;
    for(Connection& conn : connections)
        if(conn.player->id == event.entity->id) {
            exclusion = &conn;
            break;
        }
    
    sf::Packet packet;
    packet << PacketType::ENTITY_VELOCITY << event.entity->getVelocityX() <<  event.entity->getVelocityY() << event.entity->id;
    sendToEveryone(packet, exclusion);
}

void ServerNetworkingManager::syncEntityPositions() {
    for(Entity* entity : entities->getEntities()) {
        sf::Packet packet;
        packet << PacketType::ENTITY_POSITION << entity->getX() << entity->getY() << entity->id;
        sendToEveryone(packet);
    }
}

void ServerNetworkingManager::onEvent(EntityPositionChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::ENTITY_POSITION << event.entity->getX() << event.entity->getY() << event.entity->id;
    sendToEveryone(packet);
}

void Connection::onEvent(InventoryItemChangeEvent& event) {
    ItemStack item = player->inventory.getItem(event.item_pos);
    sf::Packet packet;
    packet << PacketType::INVENTORY << item.stack << (unsigned char)item.type << (short)event.item_pos;
    send(packet);
}*/
