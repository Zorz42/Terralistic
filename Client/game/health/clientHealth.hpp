#pragma once
#include "clientNetworking.hpp"
#include "resourcePack.hpp"

class Health : public ClientModule, EventListener<ClientPacketEvent>, EventListener<WelcomePacketEvent> {
    int health = 107, max_health = 80;
    gfx::Texture heart_texture;
    void init() override;
    void loadTextures() override;
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
    void onEvent(WelcomePacketEvent &event) override;
    void stop() override;

    ResourcePack* resource_pack;
    ClientNetworking* networking;
public:
    Health(ClientNetworking* networking, ResourcePack* resource_pack) : networking(networking), resource_pack(resource_pack){}
};
