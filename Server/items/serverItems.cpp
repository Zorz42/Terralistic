#include <random>
#include "serverItems.hpp"
#include "packetType.hpp"

void ServerItems::init() {
    blocks->block_break_event.addListener(this);
    item_creation_event.addListener(this);
    networking->new_connection_event.addListener(this);
}

void ServerItems::stop() {
    blocks->block_break_event.removeListener(this);
    item_creation_event.removeListener(this);
    networking->new_connection_event.removeListener(this);
}

void ServerItems::onEvent(ItemCreationEvent &event) {
    sf::Packet packet;
    packet << PacketType::ITEM_CREATION << (int)event.item->getX() << (int)event.item->getY() << event.item->id << (unsigned char)event.item->getType();
    networking->sendToEveryone(packet);
}

void ServerItems::onEvent(ServerNewConnectionEvent& event) {
    for(const Entity* entity : entities->getEntities())
        if(entity->type == EntityType::ITEM) {
            Item* item = (Item*)entity;
            sf::Packet item_packet;
            item_packet << PacketType::ITEM_CREATION << item->getX() << item->getY() << item->id << (unsigned char)item->getType();
            event.connection->send(item_packet);
        }
}

void ServerItems::onEvent(BlockBreakEvent& event) {
    static std::random_device device;
    static std::mt19937 engine(device());
    if(blocks->getBlockInfo(event.x, event.y).drop != ItemType::NOTHING) {
        Item* item = spawnItem(blocks->getBlockInfo(event.x, event.y).drop, event.x * BLOCK_WIDTH * 2, event.y * BLOCK_WIDTH * 2);
        entities->addVelocityX(item, int(engine() % 40) - 20);
        entities->addVelocityY(item, -int(engine() % 20) - 20);
    }
}
