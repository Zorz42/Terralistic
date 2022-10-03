#pragma once
#include "launcherModule.hpp"

class PlayerCard{
    gfx::Container card_container;
    gfx::Sprite name;
public:
    void render();
};


class PlayerInfo : LauncherModule{
    std::vector<PlayerCard> players;
public:
    void render() override;
    void init() override;
    PlayerInfo(std::string resource_path);
};
