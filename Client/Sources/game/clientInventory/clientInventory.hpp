#ifndef inventoryHandler_hpp
#define inventoryHandler_hpp

#include "properties.hpp"
#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"

#define INVENTORY_SIZE 20
#define INVENTORY_UI_SPACING 10
#define INVENTORY_ITEM_BACK_RECT_WIDTH (4 * 8 + INVENTORY_UI_SPACING)

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
    int x, y;
    void render() const;
    void renderWithBack() const;
    
    bool isHovered() const;
};

class DisplayRecipe {
    ClientInventoryItem result_display;
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
    gfx::Rect under_text_rect, behind_inventory_rect, select_rect, behind_crafting_rect;
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
