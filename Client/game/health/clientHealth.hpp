#pragma once
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "clientPlayers.hpp"

class DamageText {
    int damage, x, y;
    float offset = 20;
    bool initialized = false;
    gfx::Texture texture;
    gfx::Timer timer;
public:
    DamageText(int damage, int x, int y) : damage(damage), x(x), y(y) {}
    
    bool hasDespawned();
    void render(Camera* camera);
};

class ClientHealth : public ClientModule, EventListener<ClientPacketEvent>, EventListener<PlayerCreationEvent>, EventListener<PlayerDeletionEvent>, EventListener<PlayerHealthChangeEvent>  {
    gfx::Texture heart_texture;
    void init() override;
    void loadTextures() override;
    void update(float frame_length) override;
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
    void onEvent(PlayerCreationEvent &event) override;
    void onEvent(PlayerDeletionEvent &event) override;
    void onEvent(PlayerHealthChangeEvent &event) override;
    void stop() override;

    ResourcePack* resource_pack;
    ClientNetworking* networking;
    ClientPlayers* players;
    Camera* camera;
    std::vector<DamageText*> damage_texts;
public:
    ClientHealth(ClientNetworking* networking, ResourcePack* resource_pack, ClientPlayers* players, Camera* camera) : ClientModule("ClientHealth"), networking(networking), resource_pack(resource_pack), players(players), camera(camera) {}
};
