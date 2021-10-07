#pragma once

#include "entities.hpp"
#include "serverNetworking.hpp"

class ServerEntities : public ServerModule, public Entities, EventListener<EntityPositionChangeEvent>, EventListener<EntityVelocityChangeEvent>, EventListener<EntityDeletionEvent> {
    ServerNetworking* networking;
    
    void onEvent(EntityPositionChangeEvent& event) override;
    void onEvent(EntityVelocityChangeEvent& event) override;
    void onEvent(EntityDeletionEvent& event) override;
    
    unsigned int seconds = 0;
    
public:
    ServerEntities(Blocks* blocks, ServerNetworking* networking) : Entities(blocks), networking(networking) {}
    
    void init() override;
    
    void syncEntityPositions();
    void update(float frame_length) override;
};
