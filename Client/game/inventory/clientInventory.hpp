#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"
#include "inventory.hpp"

#define INVENTORY_SIZE 20
#define INVENTORY_UI_SPACING 10
#define INVENTORY_ITEM_BACK_RECT_WIDTH (4 * 8 + INVENTORY_UI_SPACING)

class ClientInventory : public ClientModule, EventListener<ClientPacketEvent>, EventListener<WelcomePacketEvent> {
    gfx::Texture numbers[10];
    Inventory inventory;
    bool open = false;
    int selected_slot = 0;
    gfx::Rect under_text_rect, behind_inventory_rect, select_rect, behind_crafting_rect;
    int hovered = -1, hovered_recipe = -1;
    
    void selectSlot(int slot);
    void renderItem(ItemStack item, int x, int y);
    
    void init() override;
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
    void onEvent(WelcomePacketEvent &event) override;
    bool onKeyDown(gfx::Key key) override;
    void stop() override;
    
    ResourcePack* resource_pack;
    ClientNetworking* manager;
    Items* items;
public:
    ClientInventory(ClientNetworking* manager, ResourcePack* resource_pack, Items* items, Recipes* recipes) : manager(manager), resource_pack(resource_pack), inventory(items, recipes), items(items) {}
    
    char* loadFromSerial(char* iter);
};
