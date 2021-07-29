#ifndef inventoryHandler_hpp
#define inventoryHandler_hpp

#define INVENTORY_SIZE 20

#include "properties.hpp"
#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"

class ClientInventoryItem {
    unsigned short stack;
public:
    ClientInventoryItem() : item_id(ItemType::NOTHING), stack(0) {}
    ItemType item_id;
    const ItemInfo& getUniqueItem() const;
    void setStack(unsigned short stack_);
    unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool stack_changed = true;
};

class ClientInventory : EventListener<ClientPacketEvent>, public gfx::GraphicalModule {
    ClientInventoryItem mouse_item;
    ClientInventoryItem inventory[INVENTORY_SIZE];
    char addItem(ItemType id, int quantity);
    bool open = false;
    unsigned char selected_slot = 0;
    void swapWithMouseItem(ClientInventoryItem* item);
    void clearMouseItem();
    
    void onEvent(ClientPacketEvent &event) override;
    void onKeyDown(gfx::Key key) override;
    gfx::Rect inventory_slots[INVENTORY_SIZE], under_text_rect, behind_inventory_rect, select_rect;
    gfx::Image stack_textures[20], mouse_stack_texture;
    void selectSlot(char slot);
    ClientInventoryItem *hovered = nullptr;
    void renderItem(ClientInventoryItem* item, int x, int y, int i);
    void updateStackTexture(int i);
    networkingManager* manager;
    bool inventory_hovered;
    ResourcePack* resource_pack;
public:
    ClientInventory(networkingManager* manager, ResourcePack* resource_pack) : manager(manager), resource_pack(resource_pack) {}
    void render() override;
    void init() override;
    inline bool isHovered() { return inventory_hovered; }
};

#endif
