#ifndef inventoryHandler_hpp
#define inventoryHandler_hpp

#include "properties.hpp"
#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"
#include "inventory.hpp"

#define INVENTORY_SIZE 20
#define INVENTORY_UI_SPACING 10
#define INVENTORY_ITEM_BACK_RECT_WIDTH (4 * 8 + INVENTORY_UI_SPACING)

class ClientInventory : public gfx::SceneModule, EventListener<ClientPacketEvent> {
    gfx::Texture numbers[10];
    Inventory inventory;
    bool open = false;
    unsigned char selected_slot = 0;
    gfx::Rect under_text_rect, behind_inventory_rect, select_rect, behind_crafting_rect;
    char hovered = -1, hovered_recipe = -1;
    
    void selectSlot(char slot);
    void renderItem(ItemStack item, short x, short y);
    
    void init() override;
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
    bool onKeyDown(gfx::Key key) override;
    
    ResourcePack* resource_pack;
    NetworkingManager* manager;
public:
    ClientInventory(NetworkingManager* manager, ResourcePack* resource_pack) : manager(manager), resource_pack(resource_pack) {}
    
    char* loadFromSerial(char* iter);
    
    void onWelcomePacket(sf::Packet& packet, WelcomePacketType type);
};

#endif
