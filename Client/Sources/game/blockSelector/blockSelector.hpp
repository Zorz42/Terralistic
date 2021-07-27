#ifndef blockSelector_hpp
#define blockSelector_hpp

#include "graphics.hpp"
#include "clientBlocks.hpp"
#include "inventoryHandler.hpp"
#include "PlayerHandler.hpp"

class BlockSelector : public gfx::GraphicalModule {
    void render() override;

    void onKeyDown(gfx::Key key) override;
    void onKeyUp(gfx::Key key) override;
    
    unsigned short prev_selected_x{}, prev_selected_y{}, selected_block_x{}, selected_block_y{};
    gfx::Rect select_rect{0, 0, BLOCK_WIDTH, BLOCK_WIDTH, {255, 0, 0}};
    
    bool is_left_button_pressed = false;
    
    networkingManager* manager;
    ClientBlocks* world_map;
    InventoryHandler* inventory_handler;
    PlayerHandler* player_handler;
public:
    BlockSelector(networkingManager* manager, ClientBlocks* world_map, InventoryHandler* inventory_handler, PlayerHandler* player_handler) : manager(manager), world_map(world_map), inventory_handler(inventory_handler), player_handler(player_handler) {}
};

#endif
