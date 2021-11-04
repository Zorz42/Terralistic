#include "items.hpp"

ItemType::ItemType(std::string name, unsigned short stack_size, BlockType* places) : name(std::move(name)), stack_size(stack_size), places(places) {}

ItemType* Item::getType() const {
    return type;
}

Item::Item(ItemType* type, int x, int y, unsigned short id) : type(type), Entity(EntityType::ITEM, x, y, id) {}

Item* Items::spawnItem(ItemType* type, int x, int y) {
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
    return nullptr;
}
