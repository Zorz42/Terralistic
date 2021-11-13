#pragma once
#include "clientNetworking.hpp"
#include "entities.hpp"

class ClientEntities : EventListener<ClientPacketEvent>, public ClientModule, public Entities {
    ClientNetworking* manager;
    
    void onEvent(ClientPacketEvent& event) override;
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
public:
    ClientEntities(Blocks* blocks, ClientNetworking* manager) : Entities(blocks), manager(manager) {}
};
