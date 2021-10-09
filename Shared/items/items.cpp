#include "items.hpp"
#include "properties.hpp"

ItemType Item::getType() const {
    return type;
}

Item::Item(Entities* entities, ItemType type, int x, int y, unsigned short id) : type(type), Entity(EntityType::ITEM, x, y, id) {}

Item* Items::spawnItem(ItemType type, int x, int y) {
    Item* item = new Item(entities, type, x, y);
    entities->registerEntity(item);
    ItemCreationEvent event(item);
    item_creation_event.call(event);
    return item;
}
