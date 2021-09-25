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
    explicit ClientInventoryItem(ResourcePack* resource_pack) : resource_pack(resource_pack) {}
    ClientInventoryItem() = default;
    
    ClientInventoryItem& operator=(const ClientInventoryItem& item);
    
    using ItemStack::type;
    const ItemInfo& getUniqueItem() const;
    
    void setStack(unsigned short stack_);
    unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    
    const gfx::Image& getStackTexture() const { return stack_texture; }
    int x = 0, y = 0;
    void render() const;
    void renderWithBack(unsigned short mouse_x, unsigned short mouse_y) const;
    
    bool isHovered(unsigned short mouse_x, unsigned short mouse_y) const;
};

class DisplayRecipe {
    ClientInventoryItem result_display;
    std::vector<ClientInventoryItem> ingredients;
public:
    DisplayRecipe(const Recipe* recipe, ResourcePack* resource_pack, int x, int y);
    void render(unsigned short mouse_x, unsigned short mouse_y);
    void renderIngredients(int x, int y, unsigned short mouse_x, unsigned short mouse_y);
    bool isHovered(unsigned short mouse_x, unsigned short mouse_y) { return result_display.isHovered(mouse_x, mouse_y); }
    const Recipe* recipe;
};

class ClientInventory : public gfx::SceneModule, EventListener<ClientPacketEvent> {
    ClientInventoryItem mouse_item;
    ClientInventoryItem inventory[INVENTORY_SIZE];
    bool open = false;
    unsigned char selected_slot = 0;
    gfx::Rect under_text_rect, behind_inventory_rect, select_rect, behind_crafting_rect;
    ClientInventoryItem *hovered = nullptr;
    char crafting_hovered = -1;
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
    NetworkingManager* manager;
public:
    ClientInventory(NetworkingManager* manager, ResourcePack* resource_pack);
    bool isHovered() const { return inventory_hovered; }
};

#endif
