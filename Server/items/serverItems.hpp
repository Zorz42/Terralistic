#pragma once

#include "items.hpp"
#include "serverNetworking.hpp"

class ServerItems : public ServerModule, public Items, EventListener<ItemCreationEvent> {
    ServerNetworking* networking;
    
    void onEvent(ItemCreationEvent& event) override;
    
    void init() override;
    void stop() override;
public:
    ServerItems(Entities* entities, Blocks* blocks, ServerNetworking* networking) : Items(entities, blocks), networking(networking) {}
};
