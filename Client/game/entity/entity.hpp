#ifndef entity_hpp
#define entity_hpp

#include <vector>
#include "clientBlocks.hpp"

class Entity {
public:
    Entity(unsigned short id) : id(id) {}
    float x, y, velocity_x, velocity_y;
    virtual unsigned short getWidth();
    virtual unsigned short getHeight();
    bool gravity = false;
    const unsigned short id;
};

class EntityManager {
    std::vector<Entity*> entities;
    ClientBlocks* blocks;
public:
    EntityManager(ClientBlocks* blocks) : blocks(blocks) {}
    void updateAllEntities();
    void addEntity(Entity* entity);
};

#endif
