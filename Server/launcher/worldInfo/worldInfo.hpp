#pragma once
#include "launcherModule.hpp"

class WorldInfo : LauncherModule{
    gfx::Sprite name_text, seed_text, port_text;
public:
    void render() override;
    void init() override;
    WorldInfo(float x_, float y_, float w_, float h_);
};