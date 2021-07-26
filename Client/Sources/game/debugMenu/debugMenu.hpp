#ifndef debugMenu_hpp
#define debugMenu_hpp

#include "graphics.hpp"

#include "playerHandler.hpp"
#include "clientBlocks.hpp"

class debugMenu : public gfx::GraphicalModule {
    bool debug_menu_open = false, m_down = false;
    gfx::Sprite fps_text, coords_text;
    MainPlayer* main_player;
    void renderFpsText();
    unsigned int fps_count = 0;
    ClientBlocks* world_map;
public:
    debugMenu(MainPlayer* main_player, ClientBlocks* world_map) : main_player(main_player), world_map(world_map) {}
    void init() override;
    void update() override;
    void render() override;
    void onKeyDown(gfx::Key key) override;
    void onKeyUp(gfx::Key key) override;
};

#endif
