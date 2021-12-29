#include <fstream>
#include <filesystem>
#include <set>
#include "platform_folders.h"
#include "modManager.hpp"

GuiMod::GuiMod(const std::string& name) : name(name) {
    fill_color = BLACK;
    fill_color.a = TRANSPARENCY;
    blur_intensity = BLUR;
    shadow_intensity = SHADOW_INTENSITY;
    text.loadFromText(name);
    setWidth(300);
    setHeight(text.getTextureHeight() * 2 + SPACING);
    smooth_factor = 3;
}

void GuiMod::renderTile() {
    Rect::render();
    text.render(2, getTranslatedX() + getWidth() / 2 - text.getTextureWidth(), getTranslatedY() + SPACING / 2);
}

bool GuiMod::hoversPoint(int x, int y) {
    return x > getTranslatedX() && x < getTranslatedX() + getWidth() && y > getTranslatedY() && y < getTranslatedY() + getHeight();
}

void ModManager::init() {
    std::set<std::string> active_mods_in_file;
    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/activeMods.txt")) {
        std::ifstream active_mods_file(sago::getDataHome() + "/Terralistic/activeMods.txt");
        std::string line;
        while(std::getline(active_mods_file, line))
            active_mods_in_file.insert(active_mods_in_file.begin(), line);
    }
    
    std::set<std::string> ignored_files = {".", "..", ".DS_Store"};
    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/Mods")) {
        for(const auto & mod : std::filesystem::directory_iterator(sago::getDataHome() + "/Terralistic/Mods")) {
            std::string mod_name = mod.path().filename().string();
            if(ignored_files.count(mod_name))
                continue;
            
            GuiMod* new_mod = new GuiMod(mod_name);
            new_mod->enabled = active_mods_in_file.count(mod_name);
            mods.push_back(new_mod);
        }
    }
    
    placeholder.fill_color = LIGHT_GREY;
    placeholder.fill_color.a = TRANSPARENCY;
    placeholder.orientation = gfx::TOP;
    placeholder.smooth_factor = 3;
    
    enabled_text.loadFromText("Enabled");
    enabled_text.scale = 3;
    enabled_text.orientation = gfx::TOP;
    enabled_text.x = 200;
    enabled_text.y = SPACING;
    
    disabled_text.loadFromText("Disabled");
    disabled_text.scale = 3;
    disabled_text.orientation = gfx::TOP;
    disabled_text.x = -200;
    disabled_text.y = SPACING;
    
    back_button.loadFromText("Back");
    back_button.orientation = gfx::BOTTOM;
    back_button.scale = 3;
    back_button.y = -SPACING;
}

bool ModManager::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        for(int i = 0; i < mods.size(); i++)
            if(mods[i]->hoversPoint(getMouseX(), getMouseY())) {
                holding = mods[i];
                hold_x = getMouseX() - holding->getTranslatedX();
                hold_y = getMouseY() - holding->getTranslatedY();
                holding_x = holding->getX();
                holding_y = holding->getY();
                holding->smooth_factor = 1;
            }
        return true;
    }
    return false;
}

bool ModManager::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered(getMouseX(), getMouseY()))
            returnFromScene();
        return true;
    }
    return false;
}

void ModManager::render() {
    int placeholder_x = 0, placeholder_y = 0;
    int curr_disabled_y = 2 * SPACING + enabled_text.getHeight(), curr_enabled_y = 2 * SPACING + enabled_text.getHeight();
    for(int i = 0; i < mods.size(); i++) {
        if(mods[i] == holding) {
            placeholder_x = mods[i]->enabled ? 200 : -200;
            placeholder_y = mods[i]->enabled ? curr_enabled_y : curr_disabled_y;
            placeholder.setX(placeholder_x);
            placeholder.setY(placeholder_y);
            placeholder.setWidth(mods[i]->getWidth());
            placeholder.setHeight(mods[i]->getHeight());
        } else {
            mods[i]->orientation = gfx::TOP;
            mods[i]->setY(mods[i]->enabled ? curr_enabled_y : curr_disabled_y);
            mods[i]->setX(mods[i]->enabled ? 200 : -200);
        }
        if(mods[i]->enabled)
            curr_enabled_y += mods[i]->getHeight() + SPACING;
        else
            curr_disabled_y += mods[i]->getHeight() + SPACING;
    }
    
    background->setBackWidth(800);
    background->renderBack();
    
    if(!getKeyState(gfx::Key::MOUSE_LEFT) && holding) {
        holding->smooth_factor = 3;
        holding = nullptr;
    }
    
    enabled_text.render();
    disabled_text.render();
    back_button.render(getMouseX(), getMouseY());
    
    for(int i = 0; i < mods.size(); i++)
        if(mods[i] != holding)
            mods[i]->renderTile();
    
    if(holding) {
        placeholder.render();
        holding->renderTile();
    }
    
    if(holding) {
        int holding_x_should_be = getMouseX() - gfx::getWindowWidth() / 2 + holding->getWidth() / 2 - hold_x, holding_y_should_be = getMouseY() - hold_y;
        
        holding_vel_y += (holding_y_should_be - holding_y) / 3;
        holding_vel_y /= 1.4;
        
        holding_vel_x += (holding_x_should_be - holding_x) / 3;
        holding_vel_x /= 1.4;
        
        holding_x += holding_vel_x;
        holding_y += holding_vel_y;
        
        holding->setX(holding_x);
        holding->setY(holding_y);
        
        holding->enabled = holding_x > 0;
    
        int min_distance = std::abs(holding_x - placeholder_x) + std::abs(holding_y - placeholder_y);;
        GuiMod* nearest = holding;
        for(int i = 0; i < mods.size(); i++) {
            if(mods[i] != holding) {
                int distance = std::abs(holding_x - mods[i]->getX()) + std::abs(holding_y - mods[i]->getY());
                if(!nearest || distance < min_distance) {
                    min_distance = distance;
                    nearest = mods[i];
                }
            }
        }
        
        if(nearest != holding) {
            bool looking_for_holding = true, looking_for_nearest = true;
            for(int i = 0; i < mods.size(); i++)
                if(mods[i] == holding && looking_for_holding) {
                    mods[i] = nearest;
                    looking_for_holding = false;
                } else if(mods[i] == nearest && looking_for_nearest) {
                    mods[i] = holding;
                    looking_for_nearest = false;
                }
        }
    }
}

void ModManager::stop() {
    std::ofstream mods_file(sago::getDataHome() + "/Terralistic/activeMods.txt", std::ios::trunc);
    for(int i = 0; i < mods.size(); i++) {
        if(mods[i]->enabled)
            mods_file << mods[i]->getName() << std::endl;
        delete mods[i];
    }
}
