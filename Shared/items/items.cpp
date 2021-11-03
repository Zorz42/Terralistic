#include "items.hpp"
#include "properties.hpp"

ItemType::ItemType(std::string  name, unsigned short stack_size, BlockTypeOld places) : name(std::move(name)), stack_size(stack_size), places(places) {}

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

void Items::registerNewItemType(ItemType* item_type) {
    item_type->id = item_types.size();
    item_types.push_back(item_type);
}

ItemType* Items::getItemTypeById(unsigned char item_id) {
    return item_types[item_id];
}

unsigned char Items::getNumItemTypes() {
    return item_types.size();
}
