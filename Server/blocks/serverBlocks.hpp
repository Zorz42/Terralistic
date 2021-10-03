#pragma once

#include "blocks.hpp"
#include "serverNetworking.hpp"

class ServerBlocks : public Blocks, EventListener<ServerConnectionWelcomeEvent>, EventListener<BlockChangeEvent>, EventListener<BlockBreakStageChangeEvent> {
    ServerNetworking* networking_manager;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(BlockBreakStageChangeEvent& event) override;
public:
    ServerBlocks(ServerNetworking* networking_manager) : networking_manager(networking_manager) {}
    
    void init();
};
