#pragma once
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "clientPlayers.hpp"

class ClientHealth : public ClientModule, EventListener<ClientPacketEvent>, EventListener<WelcomePacketEvent> {
    int health = 80, max_health = 80;
    gfx::Texture heart_texture;
    void init() override;
    void loadTextures() override;
    void update(float frame_length) override;
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
    void onEvent(WelcomePacketEvent &event) override;
    void stop() override;

    ResourcePack* resource_pack;
    ClientNetworking* networking;
    ClientPlayers* players;
public:
    ClientHealth(ClientNetworking* networking, ResourcePack* resource_pack, ClientPlayers* players) : networking(networking), resource_pack(resource_pack), players(players) {}
};
