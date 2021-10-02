#include "mainMenu.hpp"
#include "worldSelector.hpp"
#include "multiplayerSelector.hpp"
#include "settings.hpp"
#include "versions.hpp"

#define BUTTON_SPACING 1

void MainMenu::init() {
    singleplayer_button.scale = 3;
    singleplayer_button.loadFromText("Singleplayer");
    singleplayer_button.orientation = gfx::TOP;
    
    multiplayer_button.scale = 3;
    multiplayer_button.loadFromText("Multiplayer");
    multiplayer_button.orientation = gfx::TOP;
    
    settings_button.scale = 3;
    settings_button.loadFromText("Settings");
    settings_button.orientation = gfx::TOP;

    exit_button.scale = 3;
    exit_button.loadFromText("Exit");
    exit_button.orientation = gfx::TOP;
    
    debug_title.loadFromText("DEBUG MODE", GREY);
    debug_title.orientation = gfx::TOP;
    debug_title.scale = 2;
    debug_title.y = SPACING / 4;
    
    title.loadFromText("Terralistic");
    title.scale = 4;
    title.orientation = gfx::TOP;
    title.y = debug_title.y + debug_title.getHeight() + SPACING / 2;
    
    version.loadFromText(CURR_VERSION_STR, GREY);
    version.orientation = gfx::BOTTOM;
    version.scale = 2;
    version.y = -5;
}

bool MainMenu::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(singleplayer_button.isHovered(getMouseX(), getMouseY())) {
            WorldSelector world_selector(menu_back);
            switchToScene(world_selector);
            return true;
        } else if(multiplayer_button.isHovered(getMouseX(), getMouseY())) {
            MultiplayerSelector multiplayer_selector(menu_back);
            switchToScene(multiplayer_selector);
            return true;
        } else if(settings_button.isHovered(getMouseX(), getMouseY())) {
            Settings settings(menu_back);
            switchToScene(settings);
            return true;
        } else if(exit_button.isHovered(getMouseX(), getMouseY())) {
            returnFromScene();
            return true;
        }
    }
    return false;
}

void MainMenu::render() {
    int height = singleplayer_button.getHeight() + multiplayer_button.getHeight() + settings_button.getHeight() + exit_button.getHeight() + 3 * BUTTON_SPACING;
    
    singleplayer_button.y = gfx::getWindowHeight() / 2 - height / 2;
    multiplayer_button.y = singleplayer_button.y + singleplayer_button.getHeight() + BUTTON_SPACING;
    settings_button.y = multiplayer_button.y + multiplayer_button.getHeight() + BUTTON_SPACING;
    exit_button.y = settings_button.y + settings_button.getHeight() + BUTTON_SPACING;
    
    menu_back->setBackWidth(singleplayer_button.getWidth() + 100);
    menu_back->renderBack();

    title.render();
#ifdef DEVELOPER_MODE
    debug_title.render();
#endif
    version.render();
    singleplayer_button.render(getMouseX(), getMouseY());
    multiplayer_button.render(getMouseX(), getMouseY());
    settings_button.render(getMouseX(), getMouseY());
    exit_button.render(getMouseX(), getMouseY());
}
