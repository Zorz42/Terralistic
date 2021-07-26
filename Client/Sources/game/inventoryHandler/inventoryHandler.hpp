#ifndef inventoryHandler_hpp
#define inventoryHandler_hpp

#define INVENTORY_SIZE 20

#include "properties.hpp"
#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "clientMap.hpp"

class ClientInventoryItem {
    unsigned short stack;
public:
    ClientInventoryItem() : item_id(ItemType::NOTHING), stack(0) {}
    ItemType item_id;
    [[nodiscard]] const ItemInfo& getUniqueItem() const;
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool stack_changed = true;
};

class ClientInventory {
    ClientInventoryItem mouse_item;
public:
    ClientInventoryItem inventory[INVENTORY_SIZE];
    char addItem(ItemType id, int quantity);
    bool open = false;
    unsigned char selected_slot = 0;
    void swapWithMouseItem(ClientInventoryItem* item);
    void clearMouseItem();
    ClientInventoryItem* getMouseItem();
};

class InventoryHandler : EventListener<ClientPacketEvent>, public gfx::GraphicalModule {
    ClientInventory inventory;
    void onEvent(ClientPacketEvent &event) override;
    void onKeyDown(gfx::Key key) override;
    gfx::Rect inventory_slots[INVENTORY_SIZE],
    select_rect_inventory{0, 5, 2 * (BLOCK_WIDTH + 10), 2 * (BLOCK_WIDTH + 10), {50, 50, 50}, gfx::TOP},
    under_text_rect{0, 0, 0, 0, {0, 0, 0}};
    gfx::Image stack_textures[20], mouse_stack_texture;
    void selectSlot(char slot);
    ClientInventoryItem *hovered = nullptr;
    void renderItem(ClientInventoryItem* item, int x, int y, int i);
    void updateStackTexture(int i);
    networkingManager* manager;
public:
    InventoryHandler(networkingManager* manager) : manager(manager) {}
    void render() override;
    void init() override;
    inline bool isHovered() { return hovered; }
};

#endif /* inventoryHandler_hpp */
