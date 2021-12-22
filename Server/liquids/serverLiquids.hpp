#pragma once
#include "liquids.hpp"
#include "serverNetworking.hpp"

class LiquidUpdate {
public:
    int x, y;
    int time;
};

class ServerLiquids : public ServerModule, public Liquids, EventListener<ServerConnectionWelcomeEvent>, EventListener<LiquidChangeEvent>, EventListener<BlockChangeEvent> {
    ServerNetworking* networking;
    Blocks* blocks;
    
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(LiquidChangeEvent& event) override;
    void onEvent(BlockChangeEvent& event) override;
    
    std::priority_queue<LiquidUpdate, std::vector<LiquidUpdate>, bool(*)(LiquidUpdate&, LiquidUpdate&)> liquid_update_queue;
    
    bool* liquid_schedules;
    bool& getLiquidSchedule(int x, int y);
    bool isLiquidScheduled(int x, int y);
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ServerLiquids(Blocks* blocks, ServerNetworking* networking) : Liquids(blocks), blocks(blocks), networking(networking) {}
    
    void scheduleLiquidUpdate(int x, int y);
    void scheduleLiquidUpdateForNeighbours(int x, int y);
};
