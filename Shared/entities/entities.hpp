#ifndef entities_hpp
#define entities_hpp

#include <vector>
#include "blocks.hpp"

enum class EntityType { ITEM, PLAYER };

class Entities;

class Entity {
    friend Entities;
    float x, y, velocity_x = 0, velocity_y = 0;
    inline static unsigned short curr_id = 0;
public:
    Entity(EntityType type, int x, int y, unsigned short id=curr_id++) : type(type), x(x), y(y), id(id) {}
    virtual unsigned short getWidth() = 0;
    virtual unsigned short getHeight() = 0;
    bool gravity = true, friction = true, has_moved_x;
    const unsigned short id;
    const EntityType type;
    virtual bool isColliding(Blocks* blocks) { return isCollidingWithBlocks(blocks); }
    bool isCollidingWithBlocks(Blocks* blocks, float colliding_x, float colliding_y);
    bool isCollidingWithBlocks(Blocks* blocks);
    void updateEntity(Blocks* blocks, float frame_length);
    bool isTouchingGround(Blocks* blocks);
    
    float getVelocityX() const;
    float getVelocityY() const;
    int getX() const;
    int getY() const;
    
    virtual ~Entity() = default;
};

class EntityVelocityChangeEvent {
public:
    explicit EntityVelocityChangeEvent(Entity* entity) : entity(entity) {}
    Entity* entity;
};

class EntityPositionChangeEvent {
public:
    explicit EntityPositionChangeEvent(Entity* entity) : entity(entity) {}
    Entity* entity;
};

class Entities {
    std::vector<Entity*> entities;
    Blocks* blocks;
public:
    explicit Entities(Blocks* blocks) : blocks(blocks) {}
    
    void updateAllEntities(float frame_length);
    void registerEntity(Entity* entity);
    void removeEntity(Entity* entity);
    Entity* getEntityById(unsigned short id);
    const std::vector<Entity*>& getEntities();
    void setVelocityX(Entity* entity, float velocity_x);
    void setVelocityY(Entity* entity, float velocity_y);
    void addVelocityX(Entity* entity, float velocity_x);
    void addVelocityY(Entity* entity, float velocity_y);
    void setX(Entity* entity, float x);
    void setY(Entity* entity, float y);
    
    EventSender<EntityPositionChangeEvent> entity_position_change_event;
    EventSender<EntityVelocityChangeEvent> entity_velocity_change_event;
};

#endif
