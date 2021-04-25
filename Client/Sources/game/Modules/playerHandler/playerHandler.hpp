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

namespace playerHandler {

class mainPlayer {
public:
    int position_x, position_y;
    short velocity_x = 0, velocity_y = 0;
    gfx::sprite player;
};

class module : public gfx::sceneModule, packetListener {
    bool key_up = false, jump = false, key_left = false, key_right = false;
    
    mainPlayer* player;
    renderMap* map;
    networkingManager* manager;
    bool multiplayer;
    inventory* player_inventory;
    
    bool isPlayerColliding();
    bool touchingGround();
public:
    module(networkingManager* manager, mainPlayer* player, renderMap* map, bool multiplayer, inventory* player_inventory) : packetListener(manager), player(player), map(map), manager(manager), multiplayer(multiplayer), player_inventory(player_inventory) {}
    void onKeyUp(gfx::key key) override;
    void onKeyDown(gfx::key key) override;
    void init() override;
    void update() override;
    void render() override;
    void selectSlot(char slot);
    void onPacket(packets::packet packet) override;
};

inline inventoryItem *hovered = nullptr;

}

#endif /* playerHandler_hpp */
