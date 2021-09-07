#ifndef serverEntity_hpp
#define serverEntity_hpp

#include <vector>
#include "serverBlocks.hpp"

enum class EntityType { ITEM, PLAYER };

class ServerEntity {
    float x, y, velocity_x = 0, velocity_y = 0;
    bool has_moved;
public:
    inline static unsigned short _curr_id = 0;
    ServerEntity(EntityType type, int x, int y) : id(_curr_id++), type(type), x(x), y(y) {}
    const EntityType type;
    bool gravity = false;
    const unsigned short id;
    
    virtual unsigned short getWidth() = 0;
    virtual unsigned short getHeight() = 0;
    
    int getX() const { return x; }
    int getY() const { return y; }
    bool hasMoved() const { return has_moved; }
    bool isColliding(ServerBlocks* blocks);
    void updateEntity(ServerBlocks* blocks, float frame_length);
    bool isTouchingGround(ServerBlocks* blocks);
    void addVelocityX(float vel_x) { velocity_x += vel_x; }
    void addVelocityY(float vel_y) { velocity_y += vel_y; }
    
    virtual void onSpawn() {}
    virtual void update() {}
    virtual void onDestroy() {}
    
    virtual ~ServerEntity() {}
};

class ServerEntityManager {
    std::vector<ServerEntity*> entities;
    ServerBlocks* blocks;
public:
    ServerEntityManager(ServerBlocks* blocks) : blocks(blocks) {}
    void updateAllEntities(float frame_length);
    void addEntity(ServerEntity* entity);
    void removeEntity(ServerEntity* entity);
    const std::vector<ServerEntity*>& getEntities() { return entities; }
};

#endif
