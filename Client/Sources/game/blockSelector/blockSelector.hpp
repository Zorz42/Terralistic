#ifndef blockSelector_hpp
#define blockSelector_hpp

#include "graphics.hpp"
#include "clientMap.hpp"
#include "inventoryHandler.hpp"
#include "playerHandler.hpp"

class BlockSelector : public gfx::GraphicalModule {
    void render() override;

    void onKeyDown(gfx::Key key) override;
    void onKeyUp(gfx::Key key) override;
    
    unsigned short prev_selected_x{}, prev_selected_y{}, selected_block_x{}, selected_block_y{};
    gfx::Rect select_rect{0, 0, BLOCK_WIDTH, BLOCK_WIDTH, {255, 0, 0}};
    
    bool is_left_button_pressed = false;
    
    map* world_map;
    networkingManager* manager;
    InventoryHandler* inventory_handler;
    playerHandler* player_handler;
public:
    BlockSelector(map* world_map, networkingManager* manager, InventoryHandler* inventory_handler, playerHandler* player_handler) : world_map(world_map), manager(manager), inventory_handler(inventory_handler), player_handler(player_handler) {}
};

#endif /* blockSelector_hpp */
