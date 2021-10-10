#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "liquids.hpp"
#include "clientBlocks.hpp"
class ClientLiquids : public Liquids, public ClientModule, EventListener<ClientPacketEvent>, EventListener<WelcomePacketEvent> {
    void onEvent(ClientPacketEvent& event) override;
    
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
    NetworkingManager* networking;
    
    void onEvent(WelcomePacketEvent& event) override;
    
    void init() override;
    void render() override;
    void stop() override;
public:
    ClientLiquids(ClientBlocks* blocks, ResourcePack* resource_pack, NetworkingManager* networking) : Liquids(blocks), resource_pack(resource_pack), networking(networking), blocks(blocks) {}
};
