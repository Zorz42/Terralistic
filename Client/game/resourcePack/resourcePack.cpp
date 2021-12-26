#include <fstream>
#include <filesystem>
#include "resourcePack.hpp"
#include "platform_folders.h"

const gfx::Texture& ResourcePack::getHeartTexture() {
    return hearts;
}

std::string ResourcePack::getFile(const std::string& file_name) {
    std::string file;
    for(int i = 0; i < paths.size(); i++) {
        file = paths[i] + file_name;
        if(std::filesystem::exists(file))
            return file;
    }
    throw Exception(file_name + " was not found.");
}

void ResourcePack::init() {
    std::vector<std::string> active_resource_packs = {gfx::getResourcePath() + "resourcePack"};
    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/activeMods.txt")) {
        std::ifstream active_mods_file(sago::getDataHome() + "/Terralistic/activeMods.txt");
        std::string line;
        while(std::getline(active_mods_file, line))
            active_resource_packs.insert(active_resource_packs.begin(), sago::getDataHome() + "/Terralistic/Mods/" + line);
    }
    
    paths = active_resource_packs;
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/Mods/");
}

void ResourcePack::loadTextures() {
    hearts.loadFromFile(getFile("/misc/hearts.png"));
}
