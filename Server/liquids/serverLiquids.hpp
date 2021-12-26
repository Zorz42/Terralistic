#pragma once
#include "liquids.hpp"
#include "serverNetworking.hpp"
#include "worldSaver.hpp"

class LiquidUpdate {
public:
    int x, y;
    int time;
};

class ServerLiquids : public ServerModule, public Liquids, EventListener<ServerConnectionWelcomeEvent>, EventListener<LiquidChangeEvent>, EventListener<BlockChangeEvent>, EventListener<WorldSaveEvent>, EventListener<WorldLoadEvent> {
    ServerNetworking* networking;
    Blocks* blocks;
    WorldSaver* world_saver;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(LiquidChangeEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(WorldSaveEvent& event) override;
    void onEvent(WorldLoadEvent& event) override;
    
    std::priority_queue<LiquidUpdate, std::vector<LiquidUpdate>, bool(*)(LiquidUpdate&, LiquidUpdate&)> liquid_update_queue;
    
    bool* liquid_schedules;
    bool& getLiquidSchedule(int x, int y);
    bool isLiquidScheduled(int x, int y);
    
    void init() override;
    void postInit() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ServerLiquids(Blocks* blocks, ServerNetworking* networking, WorldSaver* world_saver) : Liquids(blocks), blocks(blocks), networking(networking), world_saver(world_saver) {}
    
    void scheduleLiquidUpdate(int x, int y);
    void scheduleLiquidUpdateForNeighbours(int x, int y);
};
