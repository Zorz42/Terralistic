#ifndef clientEntity_hpp
#define clientEntity_hpp

#include <vector>
#include "clientNetworking.hpp"
#include "entities.hpp"

class ClientEntities : EventListener<ClientPacketEvent> {
    Entities* entities;
    NetworkingManager* manager;
    
    void onEvent(ClientPacketEvent& event) override;
public:
    explicit ClientEntities(Entities* entities, NetworkingManager* manager) : entities(entities), manager(manager) {}
    void init();
};

#endif
