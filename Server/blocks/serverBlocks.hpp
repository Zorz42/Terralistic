#pragma once

#include "blocks.hpp"
#include "serverNetworking.hpp"

class ServerBlocks : public Blocks, EventListener<ServerConnectionWelcomeEvent> {
    ServerNetworkingManager* networking_manager;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
public:
    ServerBlocks(ServerNetworkingManager* networking_manager) : networking_manager(networking_manager) {}
    
    void init();
};
