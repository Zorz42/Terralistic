//
//  debugMenu.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/06/2021.
//

#ifndef debugMenu_hpp
#define debugMenu_hpp

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#else
#include "graphics.hpp"
#endif

#include "playerHandler.hpp"
#include "clientMap.hpp"

class debugMenu : public gfx::sceneModule {
    bool debug_menu_open = false, m_down = false;
    gfx::sprite fps_text, coords_text;// biome_text;
    mainPlayer* main_player;
    void renderFpsText();
    unsigned int fps_count = 0;
    map* world_map;
public:
    debugMenu(mainPlayer* main_player, map* world_map) : main_player(main_player), world_map(world_map) {}
    void init() override;
    void update() override;
    void render() override;
    void onKeyDown(gfx::key key) override;
    void onKeyUp(gfx::key key) override;
};

#endif /* debugMenu_hpp */
