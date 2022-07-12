#pragma once
#include "clientNetworking.hpp"
#include "entities.hpp"

class ClientEntities : EventListener<ClientPacketEvent>, public ClientModule, public Entities {
    ClientNetworking* networking;
    
    void onEvent(ClientPacketEvent& event) override;
    
    void init() override;
    void updateParallel(float frame_length) override;
    void stop() override;
public:
    ClientEntities(Blocks* blocks, ClientNetworking* networking) : ClientModule("ClientEntities"), Entities(blocks), networking(networking) {}
};
