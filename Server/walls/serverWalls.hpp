#pragma once
#include "serverBlocks.hpp"
#include "walls.hpp"

class ServerWalls : public ServerModule, public Walls, EventListener<ServerConnectionWelcomeEvent>, EventListener<WorldSaveEvent>, EventListener<WorldLoadEvent>, EventListener<WallChangeEvent> {
    WorldSaver* world_saver;
    ServerNetworking* networking;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(WorldSaveEvent& event) override;
    void onEvent(WorldLoadEvent& event) override;
    void onEvent(WallChangeEvent& event) override;
    
    void init() override;
    void stop() override;
public:
    ServerWalls(Blocks* blocks, WorldSaver* world_saver, ServerNetworking* networking) : Walls(blocks), world_saver(world_saver), networking(networking) {}
};
