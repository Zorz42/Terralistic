//
//  playerHandler.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef playerHandler_hpp
#define playerHandler_hpp

#include <string>
#include "graphics.hpp"
#include "inventoryHandler.hpp"
#include "clientMap.hpp"

class ClientPlayer {
public:
    int x, y;
    bool flipped;
    std::string name;
};

class MainPlayer : public ClientPlayer {
public:
    short velocity_x = 0, velocity_y = 0;
    clientInventory inventory;
};

class OtherPlayer : public ClientPlayer {
public:
    unsigned short id{0};
    gfx::Image name_text;
};

class playerHandler : public gfx::GraphicalModule, EventListener<ClientPacketEvent> {
    bool key_up = false, jump = false, key_left = false, key_right = false;

    gfx::Image player_image;
    
    MainPlayer* player;
    std::vector<OtherPlayer*> other_players;
    map* world_map;
    networkingManager* manager;
    OtherPlayer* getPlayerById(unsigned short id);
    
    void render(int x, int y, int view_x, int view_y, bool flipped);
    void render(int x, int y, int view_x, int view_y, bool flipped, gfx::Image& header);
    unsigned short getPlayerWidth();
    unsigned short getPlayerHeight();

    clientInventoryItem *hovered = nullptr;
    
    void initRenderer();

    gfx::Rect select_rect{0, 0, BLOCK_WIDTH, BLOCK_WIDTH, {255, 0, 0}};
    bool is_left_button_pressed = false;
    unsigned short prev_selected_x{}, prev_selected_y{}, selected_block_x{}, selected_block_y{};

    bool isPlayerColliding();
    bool touchingGround();

    void renderInventory();
    void renderBlockSelector();

    void onKeyDownInventory(gfx::Key key);
    void onKeyDownSelector(gfx::Key key);
    void onKeyUpSelector(gfx::Key key);
    void onPacketInventory(ClientPacketEvent &event);

    void initInventory();

    void renderItem(clientInventoryItem* item, int x, int y, int i);
    void updateStackTexture(int i);

    gfx::Rect inventory_slots[20],
    select_rect_inventory{0, 5, 2 * (BLOCK_WIDTH + 10), 2 * (BLOCK_WIDTH + 10), {50, 50, 50}, gfx::TOP},
    under_text_rect{0, 0, 0, 0, {0, 0, 0}};
    gfx::Image stack_textures[20], mouse_stack_texture;

    bool received_spawn_coords = false;
    
    void onKeyUp(gfx::Key key) override;
    void onKeyDown(gfx::Key key) override;
    void init() override;
    void update() override;
    void render() override;
    void selectSlot(char slot);
    void onEvent(ClientPacketEvent& event) override;
    
public:
    playerHandler(networkingManager* manager, MainPlayer* player, map* world_map) : manager(manager), player(player), world_map(world_map) {}
};

#endif /* playerHandler_hpp */
