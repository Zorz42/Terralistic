#include "settingsMenu.hpp"
#include "configManager.hpp"
#include "platform_folders.h"
#include "modManager.hpp"

void SettingsMenu::init() {
    for(Setting* setting : settings->getSettings()) {
        switch(setting->type) {
            case SettingType::CHOICE_SETTING: {
                ChoiceSetting* choice_setting = (ChoiceSetting*)setting;
                RenderChoiceSetting* render_choice_setting = new RenderChoiceSetting(choice_setting);
                render_settings.push_back(render_choice_setting);
                break;
            }
        }
        
        if(render_settings.back()->getWidth() > required_width)
            required_width = render_settings.back()->getWidth();
    }
    
    back_button.loadFromText("Back");
    back_button.scale = 3;
    back_button.orientation = gfx::BOTTOM;
    back_button.y = -SPACING;
}

void SettingsMenu::stop() {
    for(RenderSetting* setting : render_settings)
        delete setting;
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
    background->setBackWidth(required_width + 2 * SPACING);
    background->renderBack();
    
    int y = 0;
    for(RenderSetting* setting : render_settings) {
        int width = setting->getWidth(), height = setting->getHeight();
        y += SPACING;
        
        gfx::Color c = BLACK;
        c.a = TRANSPARENCY;
        gfx::RectShape(gfx::getWindowWidth() / 2 - width / 2, y, width, height).render(c);
        
        setting->render(y);
        
        y += height;
    }
    
    
    back_button.render(getMouseX(), getMouseY());
}

RenderChoiceSetting::RenderChoiceSetting(ChoiceSetting* setting) : setting(setting) {
    
}

void RenderChoiceSetting::render(int y) {
    
}

int RenderChoiceSetting::getHeight() {
    return 100;
}

int RenderChoiceSetting::getWidth() {
    return 500;
}
