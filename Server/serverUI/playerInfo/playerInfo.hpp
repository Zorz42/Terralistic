#pragma once
#include "launcherModule.hpp"

class PlayerInfo : LauncherModule{
public:
    void render() override;
    void init() override;
    PlayerInfo(std::string resource_path);
};