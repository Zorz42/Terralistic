#ifndef entities_hpp
#define entities_hpp

#include <vector>
#include "entityType.hpp"
#include "blocks.hpp"

class Entity {
public:
    Entity(unsigned short id, EntityType type, int x, int y) : id(id), type(type), x(x), y(y) {}
    float x, y, velocity_x = 0, velocity_y = 0;
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
    virtual ~Entity() = default;
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
};

#endif
