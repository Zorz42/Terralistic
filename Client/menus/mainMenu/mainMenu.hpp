#ifndef mainMenu_hpp
#define mainMenu_hpp

#include "graphics.hpp"
#include "menuBack.hpp"

class MainMenu : public gfx::Scene {
    gfx::Button singleplayer_button, multiplayer_button, settings_button, exit_button;
    gfx::Sprite title, version;
    MenuBack menu_back;
    gfx::Sprite debug_title;
public:
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
};

#endif
