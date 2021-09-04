#include <filesystem>
#include <fstream>
#include <set>
#include "platform_folders.h"
#include "modManager.hpp"

GuiMod::GuiMod(const std::string& name) : name(name) {
    fill_color = BLACK;
    fill_color.a = TRANSPARENCY;
    blur_intensity = BLUR;
    shadow_intensity = SHADOW_INTENSITY;
    text.renderText(name);
    setWidth(300);
    setHeight(text.getTextureHeight() * 2 + SPACING);
}

void GuiMod::render() {
    Rect::render();
    text.render(2, getTranslatedX() + getWidth() / 2 - text.getTextureWidth(), getTranslatedY() + SPACING / 2);
}

bool GuiMod::hoversPoint(unsigned short x, unsigned short y) {
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
    for(const auto & mod : std::filesystem::directory_iterator(sago::getDataHome() + "/Terralistic/Mods")) {
        std::string mod_name = mod.path().filename();
        if(ignored_files.count(mod_name))
            continue;
        
        GuiMod* new_mod = new GuiMod(mod_name);
        if(active_mods_in_file.count(mod_name))
            active_mods.push_back(new_mod);
        else
            inactive_mods.push_back(new_mod);
    }
    
    placeholder.fill_color = LIGHT_GREY;
    placeholder.fill_color.a = TRANSPARENCY;
    placeholder.orientation = gfx::TOP;
}

void ModManager::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        for(GuiMod* mod : inactive_mods)
            if(mod->hoversPoint(mouse_x, mouse_y)) {
                holding = mod;
                hold_x = mouse_x - mod->getTranslatedX();
                hold_y = mouse_y - mod->getTranslatedY();
            }
        for(GuiMod* mod : active_mods)
            if(mod->hoversPoint(mouse_x, mouse_y)) {
                holding = mod;
                hold_x = mouse_x - mod->getTranslatedX();
                hold_y = mouse_y - mod->getTranslatedY();
            }
    }
}

void ModManager::render() {
    int curr_y = SPACING;
    for(GuiMod* mod : inactive_mods) {
        if(mod == holding) {
            placeholder.setX(-200);
            placeholder.setY(curr_y);
            placeholder.setWidth(mod->getWidth());
            placeholder.setHeight(mod->getHeight());
        } else {
            mod->orientation = gfx::TOP;
            mod->setY(curr_y);
            mod->setX(-200);
        }
        curr_y += mod->getHeight() + SPACING;
    }
    
    curr_y = SPACING;
    for(GuiMod* mod : active_mods) {
        if(mod == holding) {
            placeholder.setX(200);
            placeholder.setY(curr_y);
            placeholder.setWidth(mod->getWidth());
            placeholder.setHeight(mod->getHeight());
        } else {
            mod->orientation = gfx::TOP;
            mod->setY(curr_y);
            mod->setX(200);
        }
        curr_y += mod->getHeight() + SPACING;
    }
    
    background->setBackWidth(800);
    background->renderBack();
    
    if(!getKeyState(gfx::Key::MOUSE_LEFT))
        holding = nullptr;
    
    if(holding) {
        holding->setX(mouse_x - gfx::getWindowWidth() / 2 + holding->getWidth() / 2 - hold_x);
        holding->setY(mouse_y - hold_y);
    }
    
    for(GuiMod* mod : inactive_mods)
        if(mod != holding)
            mod->render();
    for(GuiMod* mod : active_mods)
        if(mod != holding)
            mod->render();
    
    if(holding) {
        placeholder.render();
        holding->render();
    }
}

void ModManager::stop() {
    for(GuiMod* mod : inactive_mods)
        delete mod;
    for(GuiMod* mod : active_mods)
        delete mod;
}
