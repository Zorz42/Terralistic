#include "items.hpp"

ItemType::ItemType(std::string name, int stack_size, BlockType* places) : name(std::move(name)), stack_size(stack_size), places(places) {}

ItemType* Item::getType() const {
    return type;
}

Item::Item(ItemType* type, int x, int y, int id) : type(type), Entity(EntityType::ITEM, x, y, id) {}

Item* Items::spawnItem(ItemType* type, int x, int y) {
    Item* item = new Item(type, x, y);
    entities->registerEntity(item);
    ItemCreationEvent event(item);
    item_creation_event.call(event);
    return item;
}

void Items::registerNewItemType(ItemType* item_type) {
    item_type->id = (int)item_types.size();
    item_types.push_back(item_type);
}

ItemType* Items::getItemTypeById(int item_id) {
    if(item_id < 0 || item_id >= item_types.size())
        throw Exception("Item type id does not exist.");
    return item_types[item_id];
}

int Items::getNumItemTypes() {
    return (int)item_types.size();
}

void Items::setBlockDrop(BlockType* block_type, ItemType* item_type) {
    if(drops.size() <= block_type->id)
        drops.resize(block_type->id + 1);
    
    drops[block_type->id] = item_type;
}

ItemType* Items::getBlockDrop(BlockType* block_type) {
    if(block_type->id < drops.size() && drops[block_type->id])
        return drops[block_type->id];
    return &ItemTypes::nothing;
}

ItemType* Items::getItemTypeByName(const std::string& name) {
    for(ItemType* item_info : item_types)
        if(item_info->name == name)
            return item_info;
    throw Exception("Could not find item by name");
    return nullptr;
}
