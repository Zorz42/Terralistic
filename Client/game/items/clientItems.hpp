#ifndef clientItems_hpp
#define clientItems_hpp

#include "graphics.hpp"
#include "resourcePack.hpp"
#include "events.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"

class ClientItem {
    const ItemInfo& getUniqueItem() const;
    unsigned short id;
    ItemType item_type;
public:
    ClientItem(ItemType item_type, int x, int y, unsigned short id) : x(x), y(y), id(id), item_type(item_type) {}
    int x, y;
    unsigned short getId() const { return id; }
    ItemType getType() const { return item_type; }
};

class ClientItems : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    std::vector<ClientItem> items;
    ResourcePack* resource_pack;
    void onEvent(ClientPacketEvent& event) override;
    ClientBlocks* blocks;
public:
    ClientItems(ResourcePack* resource_pack, ClientBlocks* blocks) : resource_pack(resource_pack), blocks(blocks) {}
    ClientItem* getItemById(unsigned short id);
    void renderItems();
};

#endif
