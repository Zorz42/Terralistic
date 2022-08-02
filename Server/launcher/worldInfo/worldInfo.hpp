#pragma once
#include "launcherModule.hpp"

class WorldInfo : LauncherModule{
    gfx::Sprite name_text;
    gfx::Sprite seed_text;
    void render() override;
public:
    WorldInfo(float x_, float y_, float w_, float h_);
};