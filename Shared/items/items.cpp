#include "items.hpp"

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

void Items::setBlockDrop(BlockType* block_type, BlockDrop block_drop) {
    if(drops.size() <= block_type->id)
        drops.resize(block_type->id + 1);
    
    drops[block_type->id] = block_drop;
}

BlockDrop Items::getBlockDrop(BlockType* block_type) {
    if(block_type->id < drops.size() && drops[block_type->id].drop)
        return drops[block_type->id];
    return BlockDrop(&nothing);
}

ItemType* Items::getItemTypeByName(const std::string& name) {
    for(int i = 0; i < item_types.size(); i++)
        if(item_types[i]->name == name)
            return item_types[i];
    throw Exception("Could not find item by name");
    return nullptr;
}
