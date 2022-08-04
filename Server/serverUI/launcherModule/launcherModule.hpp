#pragma once
#include "resourcePack.hpp"
#include "server.hpp"
#include "scene.hpp"
#include "serverModule.hpp"

class LauncherModule : public ServerModule, public gfx::SceneModule{
public:

    LauncherModule(const std::string& name): SceneModule(name){}
    Server* server = nullptr;
    gfx::Texture texture;
    float target_x = 0, target_y = 0, target_w = 0, target_h = 0;
    int width = 100, height = 100;
    int min_width = 10, min_height = 10;
    [[nodiscard]] int getMinWindowWidth() const{return (int)((float)min_width / target_w);}
    [[nodiscard]] int getMinWindowHeight() const{return (int)((float)min_height / target_h);}
};