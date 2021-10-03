#pragma once

#include "items.hpp"
#include "serverNetworking.hpp"

class ServerItems : public Items, EventListener<ItemCreationEvent> {
    ServerNetworking* networking;
    
    void onEvent(ItemCreationEvent& event);
public:
    ServerItems(Entities* entities, Blocks* blocks, ServerNetworking* networking) : Items(entities, blocks), networking(networking) {}
    
    void init();
};
