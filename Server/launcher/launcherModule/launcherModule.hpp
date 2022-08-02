#pragma once
#include "resourcePack.hpp"
#include "server.hpp"

class LauncherModule{
public:
    Server* server = nullptr;
    gfx::Texture texture;
    float target_x = 0, target_y = 0, target_w = 0, target_h = 0;
    int width = 100, height = 100;
    int min_width = 10, min_height = 10;
    virtual void render(){};
    virtual void init(){};
    int getMinWindowWidth(){return (int)((float)min_width / target_w);}
    int getMinWindowHeight(){return (int)((float)min_height / target_h);}
};