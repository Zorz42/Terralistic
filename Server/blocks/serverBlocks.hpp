#pragma once

#include "blocks.hpp"
#include "serverNetworking.hpp"

class ServerBlocks : public Blocks, EventListener<ServerConnectionWelcomeEvent>, EventListener<BlockChangeEvent>, EventListener<BlockBreakStageChangeEvent> {
    ServerNetworking* networking;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(BlockBreakStageChangeEvent& event) override;
public:
    ServerBlocks(ServerNetworking* networking) : networking(networking) {}
    
    void init();
};
