#include "mainMenu.hpp"
#include "singleplayerSelector.hpp"
#include "multiplayerSelector.hpp"
#include "settingsMenu.hpp"
#include "versions.hpp"
#include "modManager.hpp"

void MainMenu::init() {
    singleplayer_button.setScale(3);
    singleplayer_button.loadFromSurface(gfx::textToSurface("Singleplayer"));
    singleplayer_button.orientation = gfx::CENTER;
    
    multiplayer_button.setScale(3);
    multiplayer_button.loadFromSurface(gfx::textToSurface("Multiplayer"));
    multiplayer_button.orientation = gfx::CENTER;
    
    settings_button.setScale(3);
    settings_button.loadFromSurface(gfx::textToSurface("Settings"));
    settings_button.orientation = gfx::CENTER;
    
    mods_button.setScale(3);
    mods_button.loadFromSurface(gfx::textToSurface("Mods"));
    mods_button.orientation = gfx::CENTER;

    exit_button.setScale(3);
    exit_button.loadFromSurface(gfx::textToSurface("Exit"));
    exit_button.orientation = gfx::CENTER;
    
    singleplayer_button.y = -(multiplayer_button.getHeight() + settings_button.getHeight() + mods_button.getHeight() + exit_button.getHeight() + 4) / 2 + singleplayer_button.getHeight() / 3;
    multiplayer_button.y = singleplayer_button.y + singleplayer_button.getHeight() / 2 + multiplayer_button.getHeight() / 2 + 1;
    settings_button.y = multiplayer_button.y + multiplayer_button.getHeight() / 2 + settings_button.getHeight() / 2 + 1;
    mods_button.y = settings_button.y + settings_button.getHeight() / 2 + mods_button.getHeight() / 2 + 1;
    exit_button.y = mods_button.y + mods_button.getHeight() / 2 + exit_button.getHeight() / 2 + 1;
    
    debug_title.loadFromSurface(gfx::textToSurface("DEBUG MODE", GREY));
    debug_title.orientation = gfx::TOP;
    debug_title.setScale(2);
    debug_title.y = SPACING / 4;
    
    title.loadFromSurface(gfx::textToSurface("Terralistic"));
    title.setScale(4);
    title.orientation = gfx::TOP;
    title.y = debug_title.y + debug_title.getHeight() + SPACING / 2;
    
    version.loadFromSurface(gfx::textToSurface(CURR_VERSION_STR, GREY));
    version.orientation = gfx::BOTTOM;
    version.setScale(2);
    version.y = -5;
}

bool MainMenu::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(singleplayer_button.isHovered(getMouseX(), getMouseY())) {
            SingleplayerSelector(menu_back, settings).run();
            return true;
        } else if(multiplayer_button.isHovered(getMouseX(), getMouseY())) {
            MultiplayerSelector(menu_back, settings).run();
            return true;
        } else if(settings_button.isHovered(getMouseX(), getMouseY())) {
            SettingsMenu(menu_back, settings).run();
            return true;
        } else if(mods_button.isHovered(getMouseX(), getMouseY())) {
            ModManager(menu_back).run();
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
    singleplayer_button.render(getMouseX(), getMouseY(), getKeyState(gfx::Key::MOUSE_LEFT));
    multiplayer_button.render(getMouseX(), getMouseY(), getKeyState(gfx::Key::MOUSE_LEFT));
    settings_button.render(getMouseX(), getMouseY(), getKeyState(gfx::Key::MOUSE_LEFT));
    mods_button.render(getMouseX(), getMouseY(), getKeyState(gfx::Key::MOUSE_LEFT));
    exit_button.render(getMouseX(), getMouseY(), getKeyState(gfx::Key::MOUSE_LEFT));
}
