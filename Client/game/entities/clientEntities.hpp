#ifndef clientEntity_hpp
#define clientEntity_hpp

#include <vector>
#include "clientBlocks.hpp"
#include "clientNetworking.hpp"
#include "entityType.hpp"

class ClientEntity {
public:
    ClientEntity(unsigned short id, EntityType type, int x, int y) : id(id), type(type), x(x), y(y) {}
    float x, y, velocity_x = 0, velocity_y = 0;
    virtual unsigned short getWidth() = 0;
    virtual unsigned short getHeight() = 0;
    bool gravity = true, friction = true, has_moved_x;
    const unsigned short id;
    const EntityType type;
    virtual bool isColliding(ClientBlocks* blocks) { return isCollidingWithBlocks(blocks); }
    bool isCollidingWithBlocks(ClientBlocks* blocks, float colliding_x, float colliding_y);
    bool isCollidingWithBlocks(ClientBlocks* blocks);
    void updateEntity(ClientBlocks* blocks, float frame_length);
    bool isTouchingGround(ClientBlocks* blocks);
    virtual ~ClientEntity() = default;
};

class ClientEntities : EventListener<ClientPacketEvent> {
    std::vector<ClientEntity*> entities;
    ClientBlocks* blocks;
    NetworkingManager* manager;
    
    void onEvent(ClientPacketEvent& event) override;
public:
    explicit ClientEntities(ClientBlocks* blocks, NetworkingManager* manager) : blocks(blocks), manager(manager) {}
    void init();
    void updateAllEntities(float frame_length);
    void registerEntity(ClientEntity* entity);
    void removeEntity(ClientEntity* entity);
    ClientEntity* getEntityById(unsigned short id);
    const std::vector<ClientEntity*>& getEntities();
};

#endif
