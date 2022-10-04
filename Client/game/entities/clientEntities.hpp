#pragma once
#include "clientNetworking.hpp"
#include "entities.hpp"
#include <queue>

class ClientEntities : EventListener<ClientPacketEvent>, public ClientModule, public Entities {
    ClientNetworking* networking;
    std::queue<Entity*> entity_deletion_queue;
    
    void onEvent(ClientPacketEvent& event) override;
    
    void init() override;
    void updatePerMs() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ClientEntities(Blocks* blocks, ClientNetworking* networking) : ClientModule("ClientEntities"), Entities(blocks), networking(networking) {}
};
