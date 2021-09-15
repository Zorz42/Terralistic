#ifndef serverEntity_hpp
#define serverEntity_hpp

#include <vector>
#include "serverBlocks.hpp"
#include "entityType.hpp"

class ServerEntity {
    float x, y, velocity_x = 0, velocity_y = 0;
public:
    inline static unsigned short _curr_id = 0;
    ServerEntity(EntityType type, int x, int y) : id(_curr_id++), type(type), x(x), y(y) {}
    const EntityType type;
    bool gravity = true, friction = true;
    const unsigned short id;
    
    virtual unsigned short getWidth() = 0;
    virtual unsigned short getHeight() = 0;
    
    int getX() const { return x; }
    int getY() const { return y; }
    bool isColliding(ServerBlocks* blocks);
    void updateEntity(ServerBlocks* blocks, float frame_length);
    bool isTouchingGround(ServerBlocks* blocks);
    void addVelocityX(float vel_x);
    void addVelocityY(float vel_y);
    void setVelocityX(float vel_x);
    void setVelocityY(float vel_y);
    float getVelocityX() const { return velocity_x; }
    float getVelocityY() const { return velocity_y; }
    
    virtual void onSpawn() {}
    virtual void update() {}
    virtual void onDestroy() {}
    
    virtual ~ServerEntity() = default;
};

class ServerEntities {
    std::vector<ServerEntity*> entities;
    ServerBlocks* blocks;
public:
    explicit ServerEntities(ServerBlocks* blocks) : blocks(blocks) {}
    void updateAllEntities(float frame_length);
    void registerEntity(ServerEntity* entity);
    void removeEntity(ServerEntity* entity);
    const std::vector<ServerEntity*>& getEntities() { return entities; }
};



class ServerEntityVelocityChangeEvent : public Event<ServerEntityVelocityChangeEvent> {
public:
    explicit ServerEntityVelocityChangeEvent(ServerEntity& entity) : entity(entity) {}
    ServerEntity& entity;
};

class ServerEntityDeletionEvent : public Event<ServerEntityDeletionEvent> {
public:
    explicit ServerEntityDeletionEvent(const ServerEntity& entity) : entity(entity) {}
    const ServerEntity& entity;
};


#endif
