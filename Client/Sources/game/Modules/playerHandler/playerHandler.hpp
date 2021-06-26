//
//  playerHandler.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef playerHandler_hpp
#define playerHandler_hpp

#include <string>

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#else
#include "graphics.hpp"
#endif

#include "inventory.hpp"

class mainPlayer {
public:
    int position_x, position_y;
    short velocity_x = 0, velocity_y = 0;
    bool flipped = false;
    clientInventory player_inventory;
    std::string name;
};

class playerHandler : public gfx::sceneModule, packetListener {
    bool key_up = false, jump = false, key_left = false, key_right = false;

    mainPlayer* player;
    map* world_map;
    networkingManager* manager;

    clientInventoryItem *hovered = nullptr;

    gfx::rect select_rect{0, 0, BLOCK_WIDTH, BLOCK_WIDTH, {255, 0, 0}};
    bool is_left_button_pressed = false;
    unsigned short prev_selected_x{}, prev_selected_y{}, selected_block_x{}, selected_block_y{};

    bool isPlayerColliding();
    bool touchingGround();

    void renderInventory();
    void renderBlockSelector();

    void onKeyDownInventory(gfx::key key);
    void onKeyDownSelector(gfx::key key);
    void onKeyUpSelector(gfx::key key);
    void onPacketInventory(packets::packet &packet);

    void initInventory();

    void renderItem(clientInventoryItem* item, int x, int y, int i);
    void updateStackTexture(int i);

    gfx::rect inventory_slots[20],
    select_rect_inventory{0, 5, 2 * (BLOCK_WIDTH + 10), 2 * (BLOCK_WIDTH + 10), {50, 50, 50}, gfx::top},
    under_text_rect{0, 0, 0, 0, {0, 0, 0}};
    gfx::image stack_textures[20], mouse_stack_texture;

    bool received_spawn_coords = false;
public:
    playerHandler(networkingManager* manager, mainPlayer* player, map* world_map) : packetListener(manager), manager(manager), player(player), world_map(world_map) { listening_to = {packets::SPAWN_POS, packets::INVENTORY_CHANGE}; }
    void onKeyUp(gfx::key key) override;
    void onKeyDown(gfx::key key) override;
    void init() override;
    void update() override;
    void render() override;
    void selectSlot(char slot);
    void onPacket(packets::packet &packet) override;

    static void initItems();
};

#endif /* playerHandler_hpp */
