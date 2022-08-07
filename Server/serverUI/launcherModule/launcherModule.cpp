#include "launcherModule.hpp"
#include "configManager.hpp"
#include <platform_folders.h>

LauncherModule::LauncherModule(const std::string &name): SceneModule(name){
    ConfigFile file(sago::getDataHome() + "/Terralistic/ServerSettings/" + name + ".config");
    enabled = file.getStr("enabled") == "true";

    if(file.keyExists("transform")) {

        std::string properties = file.getStr("transform");
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
        target_x = (float) nums[0] / (float) nums[1];
        target_y = (float) nums[2] / (float) nums[3];
        target_w = (float) nums[4] / (float) nums[5];
        target_h = (float) nums[6] / (float) nums[7];

    }

}