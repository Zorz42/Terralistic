#include "settingsMenu.hpp"
#include "configManager.hpp"
#include "platform_folders.h"
#include "modManager.hpp"

#define MENU_WIDTH 450

void SettingsMenu::init() {
    back_button.loadFromText("Back");
    back_button.scale = 3;
    back_button.orientation = gfx::BOTTOM;
    back_button.y = -SPACING;
}

bool SettingsMenu::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered(getMouseX(), getMouseY()))
            returnFromScene();
        return true;
    }
    return false;
}

void SettingsMenu::render() {
    background->setBackWidth(MENU_WIDTH + 2 * SPACING);
    background->renderBack();
    
    back_button.render(getMouseX(), getMouseY());
}
