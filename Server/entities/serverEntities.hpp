#pragma once

#include "entities.hpp"
#include "serverNetworking.hpp"

class ServerEntities : public Entities, EventListener<EntityPositionChangeEvent>, EventListener<EntityVelocityChangeEvent>, EventListener<EntityDeletionEvent> {
    ServerNetworking* networking;
    
    void onEvent(EntityPositionChangeEvent& event) override;
    void onEvent(EntityVelocityChangeEvent& event) override;
    void onEvent(EntityDeletionEvent& event) override;
    
public:
    ServerEntities(Blocks* blocks, ServerNetworking* networking) : Entities(blocks), networking(networking) {}
    
    void init();
    
    void syncEntityPositions();
};
