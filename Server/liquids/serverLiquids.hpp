#pragma once

#include "liquids.hpp"
#include "serverNetworking.hpp"

class ServerLiquids : public Liquids, EventListener<ServerConnectionWelcomeEvent>, EventListener<LiquidChangeEvent> {
    ServerNetworking* networking_manager;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(LiquidChangeEvent& event) override;
public:
    ServerLiquids(Blocks* blocks, ServerNetworking* networking_manager) : Liquids(blocks), networking_manager(networking_manager) {}
    
    void init();
};
