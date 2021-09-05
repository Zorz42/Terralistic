#ifndef clientEntity_hpp
#define clientEntity_hpp

#include <vector>
#include "clientBlocks.hpp"

class ClientEntity {
public:
    ClientEntity(unsigned short id) : id(id) {}
    float x, y, velocity_x, velocity_y;
    virtual unsigned short getWidth();
    virtual unsigned short getHeight();
    bool gravity = false;
    const unsigned short id;
};

class ClientEntityManager {
    std::vector<ClientEntity*> entities;
    ClientBlocks* blocks;
public:
    ClientEntityManager(ClientBlocks* blocks) : blocks(blocks) {}
    void updateAllEntities();
    void addEntity(ClientEntity* entity);
};

#endif
