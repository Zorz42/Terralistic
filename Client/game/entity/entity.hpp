#ifndef entity_hpp
#define entity_hpp

#include <vector>
#include "clientBlocks.hpp"

class Entity {
public:
    int x, y;
    float velocity_x, velocity_y;
    virtual unsigned short getWidth();
    virtual unsigned short getHeight();
    bool gravity = false;
    unsigned short id;
};

class EntityManager {
    std::vector<Entity*> entities;
    ClientBlocks* blocks;
public:
    EntityManager(ClientBlocks* blocks) : blocks(blocks) {}
    void updateAllEntities();
};

#endif
