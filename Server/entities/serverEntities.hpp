#pragma once
#include "entities.hpp"
#include "serverNetworking.hpp"

class ServerEntities : public ServerModule, public Entities, EventListener<EntityPositionChangeEvent>, EventListener<EntityVelocityChangeEvent>, EventListener<EntityDeletionEvent> {
    ServerNetworking* networking;
    gfx::Timer timer;
    
    void onEvent(EntityPositionChangeEvent& event) override;
    void onEvent(EntityVelocityChangeEvent& event) override;
    void onEvent(EntityDeletionEvent& event) override;
    
    int seconds = 0;
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
    
public:
    ServerEntities(Blocks* blocks, ServerNetworking* networking) : Entities(blocks), networking(networking) {}
};
