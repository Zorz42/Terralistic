#pragma once

#include <string>
#include <utility>
#include "graphics.hpp"
#include "clientBlocks.hpp"
#include "resourcePack.hpp"
#include "clientEntities.hpp"
#include "player.hpp"
#include "liquids.hpp"
#include "particles.hpp"

#define PLAYER_WIDTH 14
#define PLAYER_HEIGHT 24

class ClientPlayer : public Player {
public:
    ClientPlayer(const std::string& name, int x, int y, unsigned short id);
    bool flipped = false;
    unsigned char texture_frame = 0;
    gfx::Texture name_text;
    unsigned int started_moving = 0;
    bool has_jumped = false;
};

class ClientPlayers : public ClientModule, EventListener<ClientPacketEvent> {
    bool walking_left = false, walking_right = false, sneaking_left = false, sneaking_right = false, running_left = false, running_right = false;
    
    void render(ClientPlayer& player_to_draw);

    std::string username;
    ClientPlayer* main_player = nullptr;
    ClientPlayer* getPlayerById(unsigned short id);
    
    void init() override;
    void update(float frame_length) override;
    void onEvent(ClientPacketEvent& event) override;
    void render() override;
    void stop() override;
    
    ClientBlocks* blocks;
    Liquids* liquids;
    ClientNetworking* manager;
    ResourcePack* resource_pack;
    Entities* entities;
    Particles* particles;
public:
    ClientPlayers(ClientNetworking* manager, ClientBlocks* blocks, Liquids* liquids, ResourcePack* resource_pack, Entities* entities, Particles* particles, const std::string& username);
    
    const ClientPlayer* getMainPlayer() { return main_player; }
};
