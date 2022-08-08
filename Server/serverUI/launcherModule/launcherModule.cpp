#include "launcherModule.hpp"
#include "configManager.hpp"
#include <platform_folders.h>
#include <fstream>

LauncherModule::LauncherModule(const std::string &name, std::string resource_path): SceneModule(name), resource_path(std::move(resource_path)){

    loadDefaultConfig();

    loadConfig();
}

bool LauncherModule::loadConfig() {
    ConfigFile file(sago::getDataHome() + "/Terralistic-Server/ServerSettings/" + *getModuleName() + ".config");
    if(file.getStr("enabled") != "true" && file.getStr("enabled") != "false")
        return false;
    enabled = file.getStr("enabled") == "true";

    std::string properties = file.getStr("transform");
    properties += ' ';
    int nums[8];
    size_t pos;
    for (int j = 0; j < 4; j++) {
        pos = properties.find('/');
        nums[2 * j] = std::stoi(properties.substr(0, pos));
        properties.erase(0, pos + 1);
        pos = properties.find(' ');
        nums[2 * j + 1] = std::stoi(properties.substr(0, pos));
        properties.erase(0, pos + 1);
    }
    if(nums[1] == 0 || nums[3] == 0 || nums[5] == 0 || nums[7] == 0)
        return false;

    target_x = (float) nums[0] / (float) nums[1];
    target_y = (float) nums[2] / (float) nums[3];
    target_w = (float) nums[4] / (float) nums[5];
    target_h = (float) nums[6] / (float) nums[7];

    if(target_x + target_w > 1.001 || target_y + target_h > 1.001)
        return false;

    return true;//successful config change
}

void LauncherModule::changeConfig(const std::string &key, const std::string &value) {
    ConfigFile file(sago::getDataHome() + "/Terralistic-Server/ServerSettings/" + *getModuleName() + ".config");
    std::string temp;
    if(file.keyExists(key))
        temp = file.getStr(key);
    file.setStr(key, value);
    file.saveConfig();
    if(!loadConfig()){
        file.setStr(key, temp);
        file.saveConfig();
        loadConfig();
    }
}

void LauncherModule::loadDefaultConfig() {
    std::ifstream file;
    file.open(sago::getDataHome() + "/Terralistic-Server/ServerSettings/" + *getModuleName() + ".config");
    file.close();//creates empty file if it isn't there already


    std::ifstream read_from(resource_path + "/resourcePack/userinterface/" + *getModuleName() + ".config");//resource path
    ConfigFile read_to(sago::getDataHome() + "/Terralistic-Server/ServerSettings/" + *getModuleName() + ".config");

    std::string line;
    while(std::getline(read_from, line)) {
        unsigned long separator_index = line.find(':');
        if(separator_index < line.length()) {
            std::string value = line.substr(separator_index + 1, line.size());
            while(!value.empty() && value[0] == ' ')
                value.erase(value.begin());
            read_to.setDefaultStr(line.substr(0, separator_index), value);
        }
    }
    read_from.close();
    read_to.saveConfig();

}