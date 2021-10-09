#pragma once

#include "graphics.hpp"
#include "clientNetworking.hpp"
#include "resourcePack.hpp"
#include "liquids.hpp"
#include "clientBlocks.hpp"

class ClientLiquids : public Liquids, EventListener<ClientPacketEvent> {
    void onEvent(ClientPacketEvent& event) override;
    
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
    NetworkingManager* networking;
public:
    ClientLiquids(ClientBlocks* blocks, ResourcePack* resource_pack, NetworkingManager* networking) : Liquids(blocks), resource_pack(resource_pack), networking(networking), blocks(blocks) {}
    
    void init();
    void render();
    void stop();
    
    void onWelcomePacket(sf::Packet& packet, WelcomePacketType type);
};
