#pragma once
#include "blocks.hpp"
#include "serverNetworking.hpp"
#include "worldSaver.hpp"

class ServerBlocks : public ServerModule, public Blocks, EventListener<ServerConnectionWelcomeEvent>, EventListener<BlockChangeEvent>, EventListener<BlockStartedBreakingEvent>, EventListener<BlockStoppedBreakingEvent>, EventListener<WorldSaveEvent>, EventListener<WorldLoadEvent> {
    ServerNetworking* networking;
    WorldSaver* world_saver;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(BlockStartedBreakingEvent& event) override;
    void onEvent(BlockStoppedBreakingEvent& event) override;
    void onEvent(WorldSaveEvent& event) override;
    void onEvent(WorldLoadEvent& event) override;
    
    void preInit() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ServerBlocks(ServerNetworking* networking, WorldSaver* world_saver) : networking(networking), world_saver(world_saver) {}
};
