#pragma once
#include "player.hpp"
#include "particles.hpp"

#define PLAYER_WIDTH 14
#define PLAYER_HEIGHT 24

class ClientPlayer : public Player {
public:
    ClientPlayer(const std::string& name, int x, int y, int id);
    int texture_frame = 0;
    gfx::Texture name_text;
    int started_moving = 0;
    bool has_jumped = false;
};

class ClientPlayers : public ClientModule, EventListener<ClientPacketEvent> {
    bool walking_left = false, walking_right = false, sneaking_left = false, sneaking_right = false, running_left = false, running_right = false;
    
    void render(ClientPlayer& player_to_draw);

    std::string username;
    ClientPlayer* main_player = nullptr;
    ClientPlayer* getPlayerById(int id);
    gfx::Texture player_texture;
    
    void init() override;
    void loadTextures() override;
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
    Camera* camera;
public:
    ClientPlayers(ClientNetworking* manager, ClientBlocks* blocks, Liquids* liquids, ResourcePack* resource_pack, Entities* entities, Particles* particles, Camera* camera, const std::string& username) :
    manager(manager), blocks(blocks), liquids(liquids), resource_pack(resource_pack), entities(entities), particles(particles), camera(camera), username(username) {}
    
    const ClientPlayer* getMainPlayer() { return main_player; }
};
