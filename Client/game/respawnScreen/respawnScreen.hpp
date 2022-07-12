#pragma once
#include "clientModule.hpp"
#include "clientPlayers.hpp"

class RespawnScreen : public ClientModule {
    void init() override;
    bool onKeyUp(gfx::Key key) override;
    void render() override;
    void loadTextures() override;
    
    ClientNetworking* networking;
    ClientPlayers* players;
    
    gfx::Rect back_rect;
    gfx::Sprite you_died_text;
    gfx::Button respawn_button;
    
    bool first_time = true, is_active = false;
public:
    RespawnScreen(ClientNetworking* networking, ClientPlayers* players) : ClientModule("RespawnScreen"), players(players), networking(networking) {}
};
