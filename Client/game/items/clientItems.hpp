#ifndef clientItems_hpp
#define clientItems_hpp

#include "graphics.hpp"
#include "resourcePack.hpp"
#include "events.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"
#include "clientEntities.hpp"

#define ITEM_WIDTH 8

class ClientItem : public ClientEntity {
    ItemType item_type;
public:
    ClientItem(ItemType item_type, int x, int y, unsigned short id) : item_type(item_type), ClientEntity(id, EntityType::ITEM, x, y) {}
    ItemType getType() const { return item_type; }
    unsigned short getWidth() override { return ITEM_WIDTH * 2; }
    unsigned short getHeight() override { return ITEM_WIDTH * 2; }
};

class ClientItems : public gfx::SceneModule, EventListener<ClientPacketEvent> {
    ResourcePack* resource_pack;
    void onEvent(ClientPacketEvent& event) override;
    ClientBlocks* blocks;
    ClientEntities* entities;
    std::vector<ClientItem*> items;
public:
    ClientItems(ResourcePack* resource_pack, ClientBlocks* blocks, ClientEntities* entities) : resource_pack(resource_pack), blocks(blocks), entities(entities) {}
    void renderItems();
    const std::vector<ClientItem*>& getItems() { return items; }
};

#endif
