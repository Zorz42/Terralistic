//
//  debugMenu.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/06/2021.
//

#ifndef debugMenu_hpp
#define debugMenu_hpp

#ifdef _WIN32
#include "graphics.hpp"
#else

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#endif

#include "playerHandler.hpp"

class debugMenu : public gfx::sceneModule {
    bool debug_menu_open = false, m_down = false;
    gfx::sprite fps_text, coords_text;
    mainPlayer* main_player;
    void renderFpsText();
    unsigned int fps_count = 0;
public:
    debugMenu(mainPlayer* main_player) : main_player(main_player) {}
    void init() override;
    void update() override;
    void render() override;
    void onKeyDown(gfx::key key) override;
    void onKeyUp(gfx::key key) override;
};

#endif /* debugMenu_hpp */
