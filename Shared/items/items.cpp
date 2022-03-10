#include "items.hpp"

ItemType* Item::getType() const {
    return type;
}

Item::Item(ItemType* type, int x, int y, int entity_item_count, int id) : type(type), entity_item_count(entity_item_count), Entity(EntityType::ITEM, x, y, id) {}

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

void Items::setBlockDrop(BlockType* block_type, TileDrop drop) {
    if(block_drops.size() <= block_type->id)
        block_drops.resize(block_type->id + 1);
    block_drops[block_type->id] = drop;
}

TileDrop Items::getBlockDrop(BlockType* block_type) {
    if(block_type->id < block_drops.size() && block_drops[block_type->id].drop)
        return block_drops[block_type->id];
    return TileDrop(&nothing);
}

void Items::setWallDrop(WallType* wall_type, TileDrop drop) {
    if(wall_drops.size() <= wall_type->id)
        wall_drops.resize(wall_type->id + 1);
    wall_drops[wall_type->id] = drop;
}

TileDrop Items::getWallDrop(WallType* wall_type) {
    if(wall_type->id < wall_drops.size() && wall_drops[wall_type->id].drop)
        return wall_drops[wall_type->id];
    return TileDrop(&nothing);
}

ItemType* Items::getItemTypeByName(const std::string& name) {
    for(int i = 0; i < item_types.size(); i++)
        if(item_types[i]->name == name)
            return item_types[i];
    throw Exception("Could not find item by name");
}
