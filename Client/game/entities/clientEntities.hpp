#pragma once

#include <vector>
#include "clientNetworking.hpp"
#include "entities.hpp"

class ClientEntities : EventListener<ClientPacketEvent>, public ClientModule, public Entities {
    NetworkingManager* manager;
    
    void onEvent(ClientPacketEvent& event) override;
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
public:
    explicit ClientEntities(Blocks* blocks, NetworkingManager* manager) : Entities(blocks), manager(manager) {}
};
