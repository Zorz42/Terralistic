#include <random>
#include "serverItems.hpp"

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
    packet << ServerPacketType::ITEM_CREATION << (int)event.item->getX() << (int)event.item->getY() << event.item->id << event.item->getType()->id;
    networking->sendToEveryone(packet);
}

void ServerItems::onEvent(ServerNewConnectionEvent& event) {
    for(int i = 0; i < entities->getEntities().size(); i++)
        if(entities->getEntities()[i]->type == EntityType::ITEM) {
            Item* item = (Item*)entities->getEntities()[i];
            sf::Packet item_packet;
            item_packet << ServerPacketType::ITEM_CREATION << item->getX() << item->getY() << item->id << item->getType()->id;
            event.connection->send(item_packet);
        }
}

void ServerItems::onEvent(BlockBreakEvent& event) {
    static std::random_device device;
    static std::mt19937 engine(device());
    BlockDrop drop = getBlockDrop(blocks->getBlockType(event.x, event.y));
    if(drop.drop != &ItemTypes::nothing && (double)rand() / RAND_MAX < drop.chance) {
        Item* item = spawnItem(getItemTypeById(drop.drop->id), event.x * BLOCK_WIDTH * 2, event.y * BLOCK_WIDTH * 2);
        entities->addVelocityX(item, int(engine() % 40) - 20);
        entities->addVelocityY(item, -int(engine() % 20) - 20);
    }
}
