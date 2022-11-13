#pragma once
#include "launcherModule.hpp"

class PlayerCard{
    gfx::Rect background;
    gfx::Container card_container;
    gfx::Sprite name_text;
public:
    Player* player;
    void render();
    PlayerCard(Player* cplayer, gfx::Container* parent_cont);
    void transform(int x, int y, int w, int h);
    void initTransform(int x, int y, int w, int h);
    gfx::Container* getContainer(){return &card_container;}
};


class PlayerInfo : LauncherModule, EventListener<ServerNewConnectionEvent>, EventListener<ServerDisconnectEvent>{
    std::vector<PlayerCard*> player_cards;
public:
    void render() override;
    void init() override;
    void stop() override;
    PlayerInfo(std::string resource_path);

    void onEvent(ServerNewConnectionEvent &event) override;
    void onEvent(ServerDisconnectEvent &event) override;
};