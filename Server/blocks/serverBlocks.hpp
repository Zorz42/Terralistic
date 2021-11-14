#pragma once
#include "blocks.hpp"
#include "serverNetworking.hpp"

class ServerBlocks : public ServerModule, public Blocks, EventListener<ServerConnectionWelcomeEvent>, EventListener<BlockChangeEvent>, EventListener<BlockStartedBreakingEvent>, EventListener<BlockStoppedBreakingEvent> {
    ServerNetworking* networking;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(BlockStartedBreakingEvent& event) override;
    void onEvent(BlockStoppedBreakingEvent& event) override;
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ServerBlocks(ServerNetworking* networking) : networking(networking) {}
};
