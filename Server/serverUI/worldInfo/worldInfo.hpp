#pragma once
#include "launcherModule.hpp"

class WorldInfo : LauncherModule{
    gfx::Sprite name_text, seed_text, port_text, state_text, clock_text;
    ServerState lastState = (ServerState)-1;
public:
    void render() override;
    void init() override;
    WorldInfo(std::string resource_path);
};
