#include "serverItems.hpp"
#include <random>
#include "properties.hpp"

ItemType ServerItem::getType() const {
    return type;
}

void ServerItem::update() {    
    
}

void ServerItem::onSpawn() {
    ServerItemCreationEvent event(type, getX(), getY(), id);
    event.call();
}

void ServerItem::onDestroy() {
    
}

void ServerItems::onEvent(ServerBlockBreakEvent& event) {
    static std::random_device device;
    static std::mt19937 engine(device());
    if(event.block.getBlockInfo().drop != ItemType::NOTHING) {
        ServerItem* item = new ServerItem(event.block.getBlockInfo().drop, event.block.getX() * BLOCK_WIDTH * 2, event.block.getY() * BLOCK_WIDTH * 2);
        entities->registerEntity(item);
        item->addVelocityX(int(engine() % 40) - 20);
        item->addVelocityY(-int(engine() % 20) - 20);
    }
}
