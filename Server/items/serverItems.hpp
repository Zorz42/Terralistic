#pragma once

#include "items.hpp"
#include "serverNetworking.hpp"

class ServerItems : public ServerModule, public Items, EventListener<ItemCreationEvent>, EventListener<ServerNewConnectionEvent>, EventListener<BlockBreakEvent> {
    ServerNetworking* networking;
    Entities* entities;
    Blocks* blocks;
    
    void onEvent(ItemCreationEvent& event) override;
    void onEvent(ServerNewConnectionEvent& event) override;
    void onEvent(BlockBreakEvent& event) override;
    
    void init() override;
    void stop() override;
public:
    ServerItems(Entities* entities, Blocks* blocks, ServerNetworking* networking) : Items(entities, blocks), networking(networking), entities(entities), blocks(blocks) {}
};
