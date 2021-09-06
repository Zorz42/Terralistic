#ifndef serverEntity_hpp
#define serverEntity_hpp

#include <vector>
#include "serverBlocks.hpp"

class ServerEntity {
public:
    inline static unsigned short _curr_id = 0;
    ServerEntity() : id(_curr_id++) {}
    float x, y, velocity_x, velocity_y;
    virtual unsigned short getWidth() = 0;
    virtual unsigned short getHeight() = 0;
    bool isColliding(ServerBlocks* blocks);
    void updateEntity(ServerBlocks* blocks);
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
