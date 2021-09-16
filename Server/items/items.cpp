#include "serverItems.hpp"
#include <random>
#include "properties.hpp"

ItemType ServerItem::getType() const {
    return type;
}

ServerItem::ServerItem(ItemType type, int x, int y) : type(type), ServerEntity(x, y) {}

void ServerItems::onEvent(ServerBlockBreakEvent& event) {
    static std::random_device device;
    static std::mt19937 engine(device());
    if(event.block.getBlockInfo().drop != ItemType::NOTHING) {
        ServerItem* item = spawnItem(event.block.getBlockInfo().drop, event.block.getX() * BLOCK_WIDTH * 2, event.block.getY() * BLOCK_WIDTH * 2);
        item->addVelocityX(int(engine() % 40) - 20);
        item->addVelocityY(-int(engine() % 20) - 20);
    }
}

ServerItem* ServerItems::spawnItem(ItemType type, int x, int y) {
    ServerItem* item = new ServerItem(type, x, y);
    items.push_back(item);
    entities->registerEntity(item);
    ServerItemCreationEvent event(*item);
    event.call();
    return item;
}

void ServerItems::removeItem(ServerItem* item) {
    items.erase(std::find(items.begin(), items.end(), item));
    entities->removeEntity(item);
    ServerItemDeletionEvent event(*item);
    event.call();
    delete item;
}
