#pragma once

#include "graphics.hpp"
#include "clientBlocks.hpp"
#include "clientInventory.hpp"
#include "clientPlayers.hpp"

class BlockSelector : public ClientModule {
    void init() override;
    void render() override;

    bool onKeyDown(gfx::Key key) override;
    
    int prev_selected_x = 0, prev_selected_y = 0, selected_block_x = 0, selected_block_y = 0;
    gfx::Rect select_rect;
    
    bool is_left_button_pressed = false;
    
    ClientNetworking* manager;
    ClientBlocks* blocks;
    ClientPlayers* player_handler;
public:
    BlockSelector(ClientNetworking* manager, ClientBlocks* blocks, ClientPlayers* player_handler) : manager(manager), blocks(blocks), player_handler(player_handler) {}
};
