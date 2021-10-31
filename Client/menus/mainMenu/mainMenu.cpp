#include "mainMenu.hpp"
#include "worldSelector.hpp"
#include "multiplayerSelector.hpp"
#include "settingsMenu.hpp"
#include "versions.hpp"
#include "modManager.hpp"

void MainMenu::init() {
    singleplayer_button.scale = 3;
    singleplayer_button.loadFromText("Singleplayer");
    singleplayer_button.orientation = gfx::CENTER;
    
    multiplayer_button.scale = 3;
    multiplayer_button.loadFromText("Multiplayer");
    multiplayer_button.orientation = gfx::CENTER;
    
    settings_button.scale = 3;
    settings_button.loadFromText("Settings");
    settings_button.orientation = gfx::CENTER;
    
    mods_button.scale = 3;
    mods_button.loadFromText("Mods");
    mods_button.orientation = gfx::CENTER;

    exit_button.scale = 3;
    exit_button.loadFromText("Exit");
    exit_button.orientation = gfx::CENTER;
    
    singleplayer_button.y = -(multiplayer_button.getHeight() + settings_button.getHeight() + mods_button.getHeight() + exit_button.getHeight() + 4) / 2 + singleplayer_button.getHeight() / 3;
    multiplayer_button.y = singleplayer_button.y + singleplayer_button.getHeight() / 2 + multiplayer_button.getHeight() / 2 + 1;
    settings_button.y = multiplayer_button.y + multiplayer_button.getHeight() / 2 + settings_button.getHeight() / 2 + 1;
    mods_button.y = settings_button.y + settings_button.getHeight() / 2 + mods_button.getHeight() / 2 + 1;
    exit_button.y = mods_button.y + mods_button.getHeight() / 2 + exit_button.getHeight() / 2 + 1;
    
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
        } else if(mods_button.isHovered(getMouseX(), getMouseY())) {
            ModManager mod_manager(menu_back);
            switchToScene(mod_manager);
            return true;
        } else if(exit_button.isHovered(getMouseX(), getMouseY())) {
            returnFromScene();
            return true;
        }
    }
    return false;
}

void MainMenu::render() {
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
    mods_button.render(getMouseX(), getMouseY());
    exit_button.render(getMouseX(), getMouseY());
}
