#include "settings.hpp"
#include "configManager.hpp"
#include "platform_folders.h"

#define MENU_WIDTH 450

void Settings::init() {
    scale_text.renderText("Scale:");
    scale_text.scale = 3;
    scale_text.setColor(WHITE);
    scale_text.orientation = gfx::TOP;
    scale_text.y = 2 * SPACING;
    scale_text.x = -MENU_WIDTH / 2 + scale_text.getWidth() / 2 + SPACING;
    
    large_scale_button.renderText("Large");
    large_scale_button.scale = 2;
    large_scale_button.orientation = gfx::TOP;
    large_scale_button.margin = 7;
    large_scale_button.y = scale_text.getHeight() / 2 + 2 * SPACING - large_scale_button.getHeight() / 2;
    large_scale_button.x = MENU_WIDTH / 2 - scale_text.getWidth() / 2 - SPACING / 2;
    
    normal_scale_button.renderText("Normal");
    normal_scale_button.scale = 2;
    normal_scale_button.orientation = gfx::TOP;
    normal_scale_button.margin = 7;
    normal_scale_button.y = scale_text.getHeight() / 2 + 2 * SPACING - normal_scale_button.getHeight() / 2;
    normal_scale_button.x = large_scale_button.x - large_scale_button.getWidth() / 2 - normal_scale_button.getWidth() / 2 - SPACING / 2;
    
    small_scale_button.renderText("Small");
    small_scale_button.scale = 2;
    small_scale_button.orientation = gfx::TOP;
    small_scale_button.margin = 7;
    small_scale_button.y = scale_text.getHeight() / 2 + 2 * SPACING - small_scale_button.getHeight() / 2;
    small_scale_button.x = normal_scale_button.x - normal_scale_button.getWidth() / 2 - small_scale_button.getWidth() / 2 - SPACING / 2;
    
    scale_select_rect.orientation = gfx::TOP;
    scale_select_rect.fill_color = GREY;
    scale_select_rect.smooth_factor = 2;
    updateScaleRect();
    
    scale_back_rect.orientation = gfx::TOP;
    scale_back_rect.setY(SPACING);
    scale_back_rect.setWidth(MENU_WIDTH);
    scale_back_rect.setHeight(scale_text.getHeight() + 2 * SPACING);
    scale_back_rect.fill_color = BLACK;
    scale_back_rect.fill_color.a = TRANSPARENCY;
    scale_back_rect.shadow_intensity = SHADOW_INTENSITY;
    
    back_button.renderText("Back");
    back_button.scale = 3;
    back_button.orientation = gfx::BOTTOM;
    back_button.y = -SPACING;
}

void Settings::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        int new_scale = 0;
        if(back_button.isHovered(mouse_x, mouse_y))
            gfx::returnFromScene();
        else if(large_scale_button.isHovered(mouse_x, mouse_y))
            new_scale = 200;
        else if(normal_scale_button.isHovered(mouse_x, mouse_y))
            new_scale = 100;
        else if(small_scale_button.isHovered(mouse_x, mouse_y))
            new_scale = 50;
        if(new_scale) {
            {
                ConfigFile config(sago::getDataHome() + "/Terralistic/settings.txt");
                config.setInt("ui_scale", new_scale);
            }
            reloadSettings();
        }
    }
}

void Settings::render() {
    background->setBackWidth(MENU_WIDTH + 2 * SPACING);
    background->renderBack();
    
    scale_back_rect.render();
    scale_text.render();
    scale_select_rect.render();
    large_scale_button.render(mouse_x, mouse_y);
    normal_scale_button.render(mouse_x, mouse_y);
    small_scale_button.render(mouse_x, mouse_y);
    
    back_button.render(mouse_x, mouse_y);
}

void Settings::reloadSettings() {
    loadSettings();
    updateScaleRect();
}

void Settings::updateScaleRect() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/settings.txt");
    int curr_scale = config.getInt("ui_scale");
    
    gfx::Button* initially_hovered_button = nullptr;
    if(curr_scale == 50)
        initially_hovered_button = &small_scale_button;
    else if(curr_scale == 100)
        initially_hovered_button = &normal_scale_button;
    else if(curr_scale == 200)
        initially_hovered_button = &large_scale_button;
    scale_select_rect.setX(initially_hovered_button->x);
    scale_select_rect.setY(initially_hovered_button->y);
    scale_select_rect.setWidth(initially_hovered_button->getWidth());
    scale_select_rect.setHeight(initially_hovered_button->getHeight());
}

void loadSettings() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/settings.txt");
    
    config.setDefaultInt("ui_scale", 100);
    
    int scale = config.getInt("ui_scale");
    if(scale != 50 && scale != 100 && scale != 200)
        scale = 100;
    gfx::setScale((float)scale / 100);
    config.setInt("ui_scale", scale);
}
