#pragma once
#include "serverBlocks.hpp"
#include "walls.hpp"

class ServerWalls : public ServerModule, public Walls, EventListener<ServerConnectionWelcomeEvent>, EventListener<WallChangeEvent>, EventListener<WallStartedBreakingEvent>, EventListener<WallStoppedBreakingEvent>, EventListener<WorldSaveEvent>, EventListener<WorldLoadEvent> {
    WorldSaver* world_saver;
    ServerNetworking* networking;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(WallChangeEvent& event) override;
    void onEvent(WallStartedBreakingEvent& event) override;
    void onEvent(WallStoppedBreakingEvent& event) override;
    void onEvent(WorldSaveEvent& event) override;
    void onEvent(WorldLoadEvent& event) override;
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ServerWalls(Blocks* blocks, WorldSaver* world_saver, ServerNetworking* networking) : Walls(blocks), world_saver(world_saver), networking(networking) {}
};
