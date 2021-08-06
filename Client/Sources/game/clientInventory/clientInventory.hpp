#ifndef inventoryHandler_hpp
#define inventoryHandler_hpp

#define INVENTORY_SIZE 20
#define INVENTORY_UI_SPACING 10

#include "properties.hpp"
#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"

class ClientInventoryItem : ItemStack {
    gfx::Image stack_texture;
    ResourcePack* resource_pack = nullptr;
public:
    ClientInventoryItem(ResourcePack* resource_pack) : resource_pack(resource_pack) {}
    ClientInventoryItem() = default;
    
    ClientInventoryItem& operator=(const ClientInventoryItem& item);
    
    using ItemStack::type;
    const ItemInfo& getUniqueItem() const;
    
    void setStack(unsigned short stack_);
    unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    
    const gfx::Image& getStackTexture() const { return stack_texture; }
    void render(int x, int y) const;
};

class DisplayRecipe {
    ClientInventoryItem result_display;
    gfx::Rect back_rect;
public:
    DisplayRecipe(const Recipe* recipe, ResourcePack* resource_pack, int x, int y);
    void updateResult();
    void render();
    const Recipe* recipe;
};

class ClientInventory : EventListener<ClientPacketEvent>, public gfx::GraphicalModule {
    ClientInventoryItem mouse_item;
    ClientInventoryItem inventory[INVENTORY_SIZE];
    bool open = false;
    unsigned char selected_slot = 0;
    gfx::Rect inventory_slots[INVENTORY_SIZE], under_text_rect, behind_inventory_rect, select_rect, crafting_back;
    ClientInventoryItem *hovered = nullptr;
    bool inventory_hovered = false;
    std::vector<DisplayRecipe*> available_recipes;
    
    void swapWithMouseItem(ClientInventoryItem* item);
    void clearMouseItem();
    char addItem(ItemType id, int quantity);
    void selectSlot(char slot);
    
    void init() override;
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
    void onKeyDown(gfx::Key key) override;
    
    ResourcePack* resource_pack;
    networkingManager* manager;
public:
    ClientInventory(networkingManager* manager, ResourcePack* resource_pack);
    bool isHovered() const { return inventory_hovered; }
};

#endif
