#include <filesystem>
#include <fstream>
#include <set>
#include "platform_folders.h"
#include "modManager.hpp"

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
    
    int curr_y = 0;
    for(GuiMod* mod : inactive_mods) {
        mod->orientation = gfx::TOP;
        mod->y = curr_y;
        mod->x = -200;
        curr_y += mod->getHeight();
    }
    
    curr_y = 0;
    for(GuiMod* mod : active_mods) {
        mod->orientation = gfx::TOP;
        mod->y = curr_y;
        mod->x = 200;
        curr_y += mod->getHeight();
    }
}

GuiMod::GuiMod(const std::string& name) : name(name) {
    renderText(name);
    scale = 2;
}

void ModManager::onKeyDown(gfx::Key key) {
    
}

void ModManager::render() {
    background->setBackWidth(800);
    background->renderBack();
    for(GuiMod* mod : inactive_mods)
        mod->render(mouse_x, mouse_y);
    for(GuiMod* mod : active_mods)
        mod->render(mouse_x, mouse_y);
}

void ModManager::stop() {
    for(GuiMod* mod : inactive_mods)
        delete mod;
    for(GuiMod* mod : active_mods)
        delete mod;
}
