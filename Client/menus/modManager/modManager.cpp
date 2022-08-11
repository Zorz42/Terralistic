#include <fstream>
#include <filesystem>
#include <set>
#include "platform_folders.h"
#include "modManager.hpp"

GuiMod::GuiMod(const std::string& name) : name(name) {
    fill_color = BLACK;
    fill_color.a = TRANSPARENCY;
    blur_radius = BLUR;
    shadow_intensity = SHADOW_INTENSITY;
    text.loadFromSurface(gfx::textToSurface(name));
    w = 300;
    h = text.getTextureHeight() * 2 + SPACING;
    smooth_factor = 3;
}

void GuiMod::renderTile() {
    Rect::render();
    text.render(2, getTranslatedRect().x + w / 2 - text.getTextureWidth(), getTranslatedRect().y + SPACING / 2);
}

bool GuiMod::hoversPoint(int x_, int y_) {
    return x_ > getTranslatedRect().x && x_ < getTranslatedRect().x + w && y_ > getTranslatedRect().y && y_ < getTranslatedRect().y + h;
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
            
            auto new_mod = new GuiMod(mod_name);
            new_mod->enabled = active_mods_in_file.count(mod_name);
            mods.push_back(new_mod);
        }
    }
    
    placeholder.fill_color = LIGHT_GREY;
    placeholder.fill_color.a = TRANSPARENCY;
    placeholder.orientation = gfx::TOP;
    placeholder.smooth_factor = 1;
    
    enabled_text.loadFromSurface(gfx::textToSurface("Enabled"));
    enabled_text.setScale(3);
    enabled_text.orientation = gfx::TOP;
    enabled_text.x = 200;
    enabled_text.y = SPACING;
    
    disabled_text.loadFromSurface(gfx::textToSurface("Disabled"));
    disabled_text.setScale(3);
    disabled_text.orientation = gfx::TOP;
    disabled_text.x = -200;
    disabled_text.y = SPACING;
    
    back_button.loadFromSurface(gfx::textToSurface("Back"));
    back_button.orientation = gfx::BOTTOM;
    back_button.setScale(3);
    back_button.y = -SPACING;
}

bool ModManager::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        for(auto & mod : mods)
            if(mod->hoversPoint(getMouseX(), getMouseY())) {
                holding = mod;
                hold_x = getMouseX() - holding->getTranslatedRect().x;
                hold_y = getMouseY() - holding->getTranslatedRect().x;
                holding_x = holding->x;
                holding_y = holding->y;
                holding->smooth_factor = 1;
                changed_mods = true;
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
    int placeholder_x, placeholder_y = 0;
    int curr_disabled_y = 2 * SPACING + enabled_text.getHeight(), curr_enabled_y = 2 * SPACING + enabled_text.getHeight();
    for(auto & mod : mods) {
        if(mod == holding) {
            placeholder_x = mod->enabled ? 200 : -200;
            placeholder_y = mod->enabled ? curr_enabled_y : curr_disabled_y;
            placeholder.x = placeholder_x;
            placeholder.y = placeholder_y;
            placeholder.w = mod->w;
            placeholder.h = mod->h;
        } else {
            mod->orientation = gfx::TOP;
            mod->y = mod->enabled ? curr_enabled_y : curr_disabled_y;
            mod->x = mod->enabled ? 200 : -200;
        }
        if(mod->enabled)
            curr_enabled_y += mod->h + SPACING;
        else
            curr_disabled_y += mod->h + SPACING;
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
    
    for(auto & mod : mods)
        if(mod != holding)
            mod->renderTile();
    
    if(holding) {
        placeholder.render();
        holding->renderTile();
        placeholder.smooth_factor = 3;
    }else
        placeholder.smooth_factor = 1;
    
    if(holding) {
        int holding_x_should_be = getMouseX() - gfx::getWindowWidth() / 2 + holding->w / 2 - hold_x, holding_y_should_be = getMouseY() - hold_y;
        
        holding_vel_y += (holding_y_should_be - holding_y) / 3;
        holding_vel_y /= 1.4;
        
        holding_vel_x += (holding_x_should_be - holding_x) / 3;
        holding_vel_x /= 1.4;
        
        holding_x += holding_vel_x;
        holding_y += holding_vel_y;
        
        holding->x = holding_x;
        holding->y = holding_y;
        
        holding->enabled = holding_x > 0;
    
        int min_distance = std::abs(holding_y - placeholder_y);
        GuiMod* nearest = holding;
        for(auto & mod : mods) {
            if(mod->enabled == holding->enabled && mod != holding) {
                int distance = std::abs(holding_y - mod->y);
                if(!nearest || distance < min_distance) {
                    min_distance = distance;
                    nearest = mod;
                }
            }
        }
        
        if(nearest != holding) {
            bool looking_for_holding = true, looking_for_nearest = true;
            for(auto & mod : mods)
                if(mod == holding && looking_for_holding) {
                    mod = nearest;
                    looking_for_holding = false;
                } else if(mod == nearest && looking_for_nearest) {
                    mod = holding;
                    looking_for_nearest = false;
                }
        }
    }
}

void ModManager::stop() {
    std::ofstream mods_file(sago::getDataHome() + "/Terralistic/activeMods.txt", std::ios::trunc);
    for(auto & mod : mods) {
        if(mod->enabled)
            mods_file << mod->getName() << std::endl;
        delete mod;
    }

}
