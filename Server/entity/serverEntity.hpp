#ifndef serverEntity_hpp
#define serverEntity_hpp

#include <vector>
#include "serverBlocks.hpp"

class ServerEntity {
public:
    ServerEntity(unsigned short id) : id(id) {}
    float x, y, velocity_x, velocity_y;
    virtual unsigned short getWidth();
    virtual unsigned short getHeight();
    bool gravity = false;
    const unsigned short id;
};

class ServerEntityManager {
    std::vector<ServerEntity*> entities;
    ServerBlocks* blocks;
public:
    ServerEntityManager(ServerBlocks* blocks) : blocks(blocks) {}
    void updateAllEntities();
    void addEntity(ServerEntity* entity);
};

#endif
