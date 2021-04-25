//
//  blockSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/07/2020.
//

#ifndef blockSelector_hpp
#define blockSelector_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"

class blockSelector : public gfx::sceneModule {
    gfx::rect select_rect{0, 0, BLOCK_WIDTH, BLOCK_WIDTH, {255, 0, 0}};
    bool is_left_button_pressed = false;
    unsigned short prev_selected_x, prev_selected_y, selected_block_x, selected_block_y;
    
    bool multiplayer;
    networkingManager* manager;
    renderMap* map;
    playerHandler::mainPlayer* player;
    inventory* player_inventory;
public:
    blockSelector(bool multiplayer, networkingManager* manager, renderMap* map, playerHandler::mainPlayer* player, inventory* player_inventory) : multiplayer(multiplayer), manager(manager), map(map), player(player), player_inventory(player_inventory) {}
    
    void render() override;
    void onKeyDown(gfx::key key) override;
    void onKeyUp(gfx::key key) override;
    
    static void initEvents();
};

#endif /* blockSelector_hpp */
