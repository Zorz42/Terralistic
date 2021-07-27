#ifndef debugMenu_hpp
#define debugMenu_hpp

#include "graphics.hpp"

#include "playerHandler.hpp"
#include "clientBlocks.hpp"

class debugMenu : public gfx::GraphicalModule {
    bool debug_menu_open = false, m_down = false;
    gfx::Sprite fps_text, coords_text;
    PlayerHandler* player_handler;
    void renderFpsText();
    unsigned int fps_count = 0;
    ClientBlocks* world_map;
public:
    debugMenu(PlayerHandler* player_handler, ClientBlocks* world_map) : player_handler(player_handler), world_map(world_map) {}
    void init() override;
    void update() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
    void onKeyUp(gfx::Key key) override;
};

#endif
