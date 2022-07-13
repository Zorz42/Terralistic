#pragma once
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "inventory.hpp"
#include "clientItems.hpp"
#include "clientPlayers.hpp"
#include "clientBlocks.hpp"

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
    
    gfx::Texture *item_text_textures = nullptr;
    
    void selectSlot(int slot);
    void renderItem(ItemStack item, int x, int y);
    int countMaxCraftNumber(const Recipe *recipe);

    void init() override;
    void loadTextures() override;
    void update(float frame_length) override;
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
    void onEvent(WelcomePacketEvent &event) override;
    bool onKeyDown(gfx::Key key) override;
    void stop() override;
    
    ResourcePack* resource_pack;
    ClientNetworking* networking;
    ClientItems* items;
    Recipes* recipes;
    ClientPlayers* players;
public:
    ClientInventory(ClientNetworking* networking, ResourcePack* resource_pack, ClientItems* items, Recipes* recipes, ClientPlayers* players, ClientBlocks* blocks) : ClientModule("ClientInventory"), networking(networking), resource_pack(resource_pack), items(items), recipes(recipes), players(players), inventory(items, recipes, blocks) {inventory.setPlayer((Player*)players->getMainPlayer());}
    
    const gfx::Texture& getItemTextTexture(ItemType* type);
};
