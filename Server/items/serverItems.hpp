#pragma once
#include "items.hpp"
#include "serverNetworking.hpp"
#include "walls.hpp"

class ServerItems : public ServerModule, public Items, EventListener<ItemCreationEvent>, EventListener<ServerNewConnectionEvent>, EventListener<BlockBreakEvent>, EventListener<WallBreakEvent> {
    ServerNetworking* networking;
    Entities* entities;
    Blocks* blocks;
    Walls* walls;
    
    void onEvent(ItemCreationEvent& event) override;
    void onEvent(ServerNewConnectionEvent& event) override;
    void onEvent(BlockBreakEvent& event) override;
    void onEvent(WallBreakEvent& event) override;
    
    void init() override;
    void stop() override;
public:
    ServerItems(Entities* entities, Blocks* blocks, Walls* walls, ServerNetworking* networking) : Items(entities, blocks), networking(networking), entities(entities), blocks(blocks), walls(walls) {}
};
