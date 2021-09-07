#include "serverItems.hpp"
#include <random>
#include "properties.hpp"

const ItemInfo& ServerItem::getItemInfo() const {
    return ::getItemInfo(type);
}

unsigned short ServerItem::getId() const {
    return id;
}

ItemType ServerItem::getType() const {
    return type;
}

void ServerItem::update() {    
    if(hasMoved()) {
        ServerItemMovementEvent event(*this);
        event.call();
    }
}

void ServerItem::onSpawn() {
    ServerItemCreationEvent event(type, getX(), getY(), getId());
    event.call();
}

void ServerItem::onDestroy() {
    ServerItemDeletionEvent event(*this);
    event.call();
}

void ServerItems::onEvent(ServerBlockBreakEvent& event) {
    static std::random_device device;
    static std::mt19937 engine(device());
    if(event.block.getBlockInfo().drop != ItemType::NOTHING) {
        ServerItem* item = new ServerItem(event.block.getBlockInfo().drop, event.block.getX() * BLOCK_WIDTH * 2, event.block.getY() * BLOCK_WIDTH * 2);
        item->addVelocityX(int(engine() % 20) - 10);
        item->addVelocityY(-int(engine() % 10) - 10);
        entity_manager->addEntity(item);
    }
}
