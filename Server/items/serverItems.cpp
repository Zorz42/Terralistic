#include "serverItems.hpp"
#include "packetType.hpp"

void ServerItems::init() {
    Items::init();
    item_creation_event.addListener(this);
}

void ServerItems::onEvent(ItemCreationEvent &event) {
    sf::Packet packet;
    packet << PacketType::ITEM_CREATION << (int)event.item->getX() << (int)event.item->getY() << event.item->id << (unsigned char)event.item->getType();
    networking->sendToEveryone(packet);
}
