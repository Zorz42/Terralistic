#pragma once
#include "resourcePack.hpp"
#include "server.hpp"
#include "scene.hpp"
#include "serverModule.hpp"

class LauncherModule : public ServerModule, public gfx::SceneModule{
public:
    void loadDefaultConfig();
    bool loadConfig();
    void changeConfig(const std::string& key, const std::string& value);
    LauncherModule(const std::string& name, std::string resource_path);
    Server* server = nullptr;
    std::string resource_path;
    gfx::Container base_container;
    float target_x = 0, target_y = 0, target_w = 0, target_h = 0;
    int min_width = 10, min_height = 10;
    [[nodiscard]] int getMinWindowWidth() const{return (int)((float)min_width / target_w);}
    [[nodiscard]] int getMinWindowHeight() const{return (int)((float)min_height / target_h);}
};