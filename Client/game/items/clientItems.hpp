#ifndef clientItems_hpp
#define clientItems_hpp

#include "graphics.hpp"
#include "resourcePack.hpp"
#include "events.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"
#include "clientEntities.hpp"

#define ITEM_WIDTH 8

class ClientItem : public Entity {
    ItemType item_type;
public:
    ClientItem(ItemType item_type, int x, int y, unsigned short id) : item_type(item_type), Entity(EntityType::ITEM, x, y, id) {}
    ItemType getType() const { return item_type; }
    unsigned short getWidth() override { return ITEM_WIDTH * 2; }
    unsigned short getHeight() override { return ITEM_WIDTH * 2; }
};

class ClientItems : EventListener<ClientPacketEvent> {
    ResourcePack* resource_pack;
    void onEvent(ClientPacketEvent& event) override;
    ClientBlocks* blocks;
    Entities* entities;
    NetworkingManager* manager;
public:
    ClientItems(ResourcePack* resource_pack, ClientBlocks* blocks, Entities* entities, NetworkingManager* manager) : resource_pack(resource_pack), blocks(blocks), entities(entities), manager(manager) {}
    void init();
    void renderItems();
};

#endif
