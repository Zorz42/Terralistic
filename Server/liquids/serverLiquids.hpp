#pragma once

#include "liquids.hpp"
#include "serverNetworking.hpp"

class ServerLiquids : public Liquids, EventListener<ServerConnectionWelcomeEvent> {
    ServerNetworkingManager* networking_manager;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
public:
    ServerLiquids(Blocks* blocks, ServerNetworkingManager* networking_manager) : Liquids(blocks), networking_manager(networking_manager) {}
    
    void init();
};
