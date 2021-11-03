#include "items.hpp"
#include "properties.hpp"

ItemTypeOld Item::getType() const {
    return type;
}

Item::Item(ItemTypeOld type, int x, int y, unsigned short id) : type(type), Entity(EntityType::ITEM, x, y, id) {}

Item* Items::spawnItem(ItemTypeOld type, int x, int y) {
    Item* item = new Item(type, x, y);
    entities->registerEntity(item);
    ItemCreationEvent event(item);
    item_creation_event.call(event);
    return item;
}
