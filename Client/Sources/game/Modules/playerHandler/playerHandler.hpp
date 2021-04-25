//
//  playerHandler.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef playerHandler_hpp
#define playerHandler_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "networkingModule.hpp"
#include "renderMap.hpp"
#include "inventory.hpp"

class mainPlayer {
public:
    int position_x, position_y;
    short velocity_x = 0, velocity_y = 0;
    bool flipped = false;
};

class playerHandler : public gfx::sceneModule, packetListener {
    bool key_up = false, jump = false, key_left = false, key_right = false;
    
    mainPlayer* player;
    renderMap* map;
    networkingManager* manager;
    bool multiplayer;
    inventory* player_inventory;
    
    inventoryItem *hovered = nullptr;
    
    gfx::rect select_rect{0, 0, BLOCK_WIDTH, BLOCK_WIDTH, {255, 0, 0}};
    bool is_left_button_pressed = false;
    unsigned short prev_selected_x, prev_selected_y, selected_block_x, selected_block_y;
    
    bool isPlayerColliding();
    bool touchingGround();
    
    void renderInventory();
    void renderBlockSelector();
    
    void onKeyDownInventory(gfx::key key);
    void onKeyDownSelector(gfx::key key);
    void onKeyUpSelector(gfx::key key);
    void onPacketInventory(packets::packet packet);
    
    void initInventory();
    
    void renderItem(inventoryItem* item, int x, int y, int i);
    void updateStackTexture(int i);
    
    gfx::rect inventory_slots[20],
    select_rect_inventory{0, 5, 2 * (BLOCK_WIDTH + 10), 2 * (BLOCK_WIDTH + 10), {50, 50, 50}, gfx::top},
    under_text_rect{0, 0, 0, 0, {0, 0, 0}};
    gfx::image stack_textures[20], mouse_stack_texture;
public:
    playerHandler(networkingManager* manager, mainPlayer* player, renderMap* map, bool multiplayer, inventory* player_inventory) : packetListener(manager), player(player), map(map), manager(manager), multiplayer(multiplayer), player_inventory(player_inventory) {}
    void onKeyUp(gfx::key key) override;
    void onKeyDown(gfx::key key) override;
    void init() override;
    void update() override;
    void render() override;
    void selectSlot(char slot);
    void onPacket(packets::packet packet) override;
    
    static void initEvents();
};

#endif /* playerHandler_hpp */
