#pragma once
#include "launcherModule.hpp"

class WorldInfo : LauncherModule{
    gfx::Sprite text;
    void render() override;
public:
    WorldInfo(float x_, float y_, float w_, float h_){
        x = x_;
        y = y_;
        target_w = w_;
        target_h = h_;
        text.scale = 2;
        text.loadFromText("Test");
        min_width = 100;
        min_height = 50;
    }
};