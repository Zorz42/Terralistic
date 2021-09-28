#ifndef serverEntity_hpp
#define serverEntity_hpp

#include <vector>
#include "blocks.hpp"
#include "entityType.hpp"

class ServerEntities;

class ServerEntity {
    float x, y, velocity_x = 0, velocity_y = 0;
    ServerEntities* entities;
public:
    inline static unsigned short _curr_id = 0;
    ServerEntity(ServerEntities* entities, int x, int y) : id(_curr_id++), x(x), y(y), entities(entities) {}
    bool gravity = true, friction = true;
    const unsigned short id;
    
    virtual unsigned short getWidth() = 0;
    virtual unsigned short getHeight() = 0;
    
    int getX() const { return x; }
    int getY() const { return y; }
    virtual bool isColliding(Blocks* blocks) { return isCollidingWithBlocks(blocks); }
    bool isCollidingWithBlocks(Blocks* blocks, float colliding_x, float colliding_y);
    bool isCollidingWithBlocks(Blocks* blocks);
    void updateEntity(Blocks* blocks, float frame_length);
    bool isTouchingGround(Blocks* blocks);
    void addVelocityX(float vel_x);
    void addVelocityY(float vel_y);
    void setVelocityX(float vel_x);
    void setVelocityY(float vel_y);
    float getVelocityX() const { return velocity_x; }
    float getVelocityY() const { return velocity_y; }
    void setPosition(int x, int y);
};

class ServerEntityVelocityChangeEvent {
public:
    explicit ServerEntityVelocityChangeEvent(ServerEntity& entity) : entity(entity) {}
    ServerEntity& entity;
};

class ServerEntityPositionChangeEvent {
public:
    explicit ServerEntityPositionChangeEvent(ServerEntity& entity) : entity(entity) {}
    ServerEntity& entity;
};

class ServerEntities {
    std::vector<ServerEntity*> entities;
    Blocks* blocks;
public:
    explicit ServerEntities(Blocks* blocks) : blocks(blocks) {}
    void updateAllEntities(float frame_length);
    void registerEntity(ServerEntity* entity);
    void removeEntity(ServerEntity* entity);
    const std::vector<ServerEntity*>& getEntities() { return entities; }
    EventSender<ServerEntityVelocityChangeEvent> entity_velocity_change_event;
    EventSender<ServerEntityPositionChangeEvent> entity_position_change_event;
};


#endif
