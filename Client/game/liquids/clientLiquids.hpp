#pragma once
#include "clientBlocks.hpp"

class ClientLiquids : public Liquids, public ClientModule, EventListener<ClientPacketEvent>, EventListener<WelcomePacketEvent> {
    void onEvent(ClientPacketEvent& event) override;
    
    ClientBlocks* blocks;
    ResourcePack* resource_pack;
    ClientNetworking* networking;
    
    gfx::RectArray liquid_rects;
    int most_blocks_on_screen = 0;
    
    void onEvent(WelcomePacketEvent& event) override;
    
    void init() override;
    void render() override;
    void stop() override;
public:
    ClientLiquids(ClientBlocks* blocks, ResourcePack* resource_pack, ClientNetworking* networking) : Liquids(blocks), resource_pack(resource_pack), networking(networking), blocks(blocks) {}
};
