#ifndef clientEntity_hpp
#define clientEntity_hpp

#include <vector>
#include "clientNetworking.hpp"
#include "entities.hpp"

class ClientEntities : EventListener<ClientPacketEvent>, public Entities {
    NetworkingManager* manager;
    
    void onEvent(ClientPacketEvent& event) override;
public:
    explicit ClientEntities(Blocks* blocks, NetworkingManager* manager) : Entities(blocks), manager(manager) {}
    void init();
};

#endif
