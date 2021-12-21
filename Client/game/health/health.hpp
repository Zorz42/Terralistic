#pragma once
#include "clientNetworking.hpp"
#include "resourcePack.hpp"

class Health : public ClientModule, EventListener<ClientPacketEvent> {
    short health = 107;
    void init() override;
    void render() override;
    void onEvent(ClientPacketEvent &event) override;
    void stop() override;

    ResourcePack* resource_pack;
    ClientNetworking* manager;
public:
    Health(ClientNetworking* manager, ResourcePack* resource_pack) : manager(manager), resource_pack(resource_pack){}
};