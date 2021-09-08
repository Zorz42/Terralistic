#ifndef clientEntity_hpp
#define clientEntity_hpp

#include <vector>
#include "clientBlocks.hpp"
#include "clientNetworking.hpp"
#include "entityType.hpp"

class ClientEntity {
public:
    ClientEntity(unsigned short id, EntityType type, int x, int y) : id(id), type(type), x(x), y(y) {}
    float x, y, velocity_x, velocity_y;
    virtual unsigned short getWidth() = 0;
    virtual unsigned short getHeight() = 0;
    bool gravity = false;
    const unsigned short id;
    const EntityType type;
    virtual ~ClientEntity() {}
};

class ClientEntities : EventListener<ClientPacketEvent> {
    std::vector<ClientEntity*> entities;
    ClientBlocks* blocks;
    
    void onEvent(ClientPacketEvent& event) override;
public:
    ClientEntities(ClientBlocks* blocks) : blocks(blocks) {}
    void updateAllEntities();
    void addEntity(ClientEntity* entity);
    ClientEntity* getEntityById(unsigned short id);
    const std::vector<ClientEntity*>& getEntities();
};

#endif
