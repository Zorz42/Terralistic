#include "serverItems.hpp"
#include <random>
#include "properties.hpp"

ItemType ServerItem::getType() const {
    return type;
}

ServerItem::ServerItem(ServerEntities* entities, ItemType type, int x, int y) : type(type), ServerEntity(entities, x, y) {}

void ServerItems::init() {
    blocks->block_break_event.addListener(this);
}

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
    ServerItem* item = new ServerItem(entities, type, x, y);
    items.push_back(item);
    entities->registerEntity(item);
    ServerItemCreationEvent event(*item);
    item_creation_event.call(event);
    return item;
}

void ServerItems::removeItem(ServerItem* item) {
    ServerItemDeletionEvent event(*item);
    item_deletion_event.call(event);
    items.erase(std::find(items.begin(), items.end(), item));
    entities->removeEntity(item);
    delete item;
}
