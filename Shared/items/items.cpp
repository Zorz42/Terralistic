#include "items.hpp"
#include <random>
#include "properties.hpp"

ItemType Item::getType() const {
    return type;
}

Item::Item(Entities* entities, ItemType type, int x, int y) : type(type), Entity(EntityType::ITEM, x, y) {}

void Items::init() {
    blocks->block_break_event.addListener(this);
}

void Items::onEvent(BlockBreakEvent& event) {
    static std::random_device device;
    static std::mt19937 engine(device());
    if(blocks->getBlockInfo(event.x, event.y).drop != ItemType::NOTHING) {
        Item* item = spawnItem(blocks->getBlockInfo(event.x, event.y).drop, event.x * BLOCK_WIDTH * 2, event.y * BLOCK_WIDTH * 2);
        entities->addVelocityX(item, int(engine() % 40) - 20);
        entities->addVelocityY(item, -int(engine() % 20) - 20);
    }
}

Item* Items::spawnItem(ItemType type, int x, int y) {
    Item* item = new Item(entities, type, x, y);
    entities->registerEntity(item);
    ItemCreationEvent event(item);
    item_creation_event.call(event);
    return item;
}
